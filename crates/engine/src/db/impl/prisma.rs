use std::collections::HashSet;

use async_trait::async_trait;
use prisma_client_rust::{
    chrono::{FixedOffset, TimeZone, Utc},
    raw, NewClientError, QueryError,
};
use serde::Deserialize;

use crate::{
    db::{
        MinecraftPlatform, Pagination, SearchTmQuery, SearchTmResponse, TmDatabase, TmEntry,
        TmEntryPair, Upload,
    },
    prisma::{self, entry, language_file, PrismaClient},
};

pub struct PrismaDatabase {
    client: PrismaClient,
}

impl PrismaDatabase {
    pub async fn connect() -> Result<PrismaDatabase, NewClientError> {
        let client = PrismaClient::_builder().build().await?;

        Ok(PrismaDatabase { client })
    }
}

fn into_prisma_minecraft_platform(platform: MinecraftPlatform) -> prisma::MinecraftPlatform {
    match platform {
        MinecraftPlatform::Java => prisma::MinecraftPlatform::Java,
        MinecraftPlatform::Bedrock => prisma::MinecraftPlatform::Bedrock,
        MinecraftPlatform::Dungeons => prisma::MinecraftPlatform::Dungeons,
    }
}

#[async_trait]
impl TmDatabase for PrismaDatabase {
    type Error = QueryError;

    async fn search(&self, query: SearchTmQuery) -> Result<SearchTmResponse, Self::Error> {
        let platform = into_prisma_minecraft_platform(query.platform);
        let source = query.source;
        let target = query.target;
        #[derive(Deserialize)]
        struct QueryResult {
            key: String,
            src: String,
            dst: Option<String>,
        }
        let result: Vec<QueryResult> = self
            .client
            ._query_raw(raw!(
                r#"
                    SELECT
                        t1.key AS `key`, src, dst
                    FROM
                        (
                            SELECT `key`, namespace, value AS src FROM
                                Entry t1
                            WHERE
                                language = {} AND
                                platform = {} AND
                                value COLLATE utf8mb4_unicode_ci LIKE CONCAT('%', {}, '%')
                        ) AS t1
                        LEFT JOIN
                        (
                            SELECT `key`, namespace, value AS dst FROM
                                Entry t2
                            WHERE
                                language = {} AND
                                platform = {}
                        ) AS t2
                        ON
                            t1.key = t2.key AND
                            t1.namespace = t2.namespace
                    ORDER BY `key` ASC
                    LIMIT {}
                    OFFSET {}
                "#,
                source.id().into(),
                platform.to_string().into(),
                query.text.clone().into(),
                target.id().into(),
                platform.to_string().into(),
                query.take.into(),
                query.skip.into()
            ))
            .exec()
            .await?;
        let items = result
            .into_iter()
            .map(|res| TmEntryPair {
                key: res.key,
                source: TmEntry {
                    language: source.clone(),
                    content: res.src,
                },
                targets: res
                    .dst
                    .map(|content| TmEntry {
                        language: target.clone(),
                        content,
                    })
                    .into_iter()
                    .collect(),
            })
            .collect();
        let game_version = self
            .client
            .language_file()
            .find_first(vec![language_file::platform::equals(platform)])
            .exec()
            .await?
            .map(|language_file| language_file.game_version)
            .unwrap_or("".to_string());
        #[derive(Deserialize)]
        struct Count {
            count: usize,
        }
        let total: Vec<Count> = self
            .client
            ._query_raw(raw!(
                r#"
                    SELECT
                        COUNT(*) AS count
                    FROM
                        Entry
                    WHERE
                        Entry.language = {} AND
                        Entry.platform = {} AND
                        Entry.value COLLATE utf8mb4_unicode_ci LIKE CONCAT('%', {}, '%')
                "#,
                source.id().into(),
                platform.to_string().into(),
                query.text.clone().into()
            ))
            .exec()
            .await?;
        Ok(SearchTmResponse {
            game_version,
            list: Pagination {
                total: total[0].count,
                items,
            },
        })
    }

    async fn upload(&self, upload: Upload) -> Result<(), Self::Error> {
        let platform = into_prisma_minecraft_platform(upload.platform);
        let now = FixedOffset::east_opt(9 * 60 * 60)
            .unwrap()
            .from_utc_datetime(&Utc::now().naive_utc());

        let namespaces: HashSet<_> = upload
            .entries
            .iter()
            .map(|entry| entry.namespace.clone())
            .collect();

        let (controller, client) = self
            .client
            ._transaction()
            .with_timeout(120 * 1000)
            .begin()
            .await?;

        client
            .entry()
            .delete_many(vec![
                entry::platform::equals(platform),
                entry::language::equals(upload.language.id().to_string()),
            ])
            .exec()
            .await?;

        client
            .language_file()
            .delete_many(vec![
                language_file::platform::equals(platform),
                language_file::language::equals(upload.language.id().to_string()),
            ])
            .exec()
            .await?;

        client
            .language_file()
            .create_many(
                namespaces
                    .into_iter()
                    .map(|namespace| {
                        (
                            platform,
                            namespace,
                            upload.language.id().to_string(),
                            upload.game_version.clone(),
                            now,
                            vec![],
                        )
                    })
                    .collect(),
            )
            .exec()
            .await?;

        client
            .entry()
            .create_many(
                upload
                    .entries
                    .into_iter()
                    .map(|entry| {
                        (
                            platform,
                            entry.namespace,
                            upload.language.id().to_string(),
                            entry.key,
                            entry.value,
                            vec![],
                        )
                    })
                    .collect(),
            )
            .exec()
            .await?;

        controller.commit(client).await?;
        Ok(())
    }
}
