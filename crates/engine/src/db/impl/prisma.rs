use std::collections::HashSet;

use async_trait::async_trait;
use prisma_client_rust::{
    chrono::{FixedOffset, TimeZone, Utc},
    raw, NewClientError, QueryError,
};
use serde::Deserialize;

use crate::{
    db::{
        MinecraftPlatform, Pagination, SearchTmQuery, SearchTmResponse, TmDatabase, TmWord,
        TmWordPair, Upload,
    },
    prisma::{self, language_file, word, PrismaClient},
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
        let source = query.source.guess(&query.text);
        let target = source.as_e2k_counterpart();
        #[derive(Deserialize)]
        struct QueryResult {
            key: String,
            src: String,
            dst: String,
        }
        let result: Vec<QueryResult> = self
            .client
            ._query_raw(raw!(
                r#"
                    SELECT
                        t1.key,
                        t1.value AS src,
                        t2.value AS dst
                    FROM
                        Word t1,
                        Word t2
                    WHERE
                        t1.key = t2.key AND
                        t1.language = {} AND
                        t2.language = {} AND
                        t1.platform = {} AND
                        t2.platform = {} AND
                        t1.namespace = t2.namespace AND
                        t1.value COLLATE utf8mb4_unicode_ci LIKE CONCAT('%', {}, '%')
                    ORDER BY t1.key ASC
                    LIMIT {}
                    OFFSET {}
                "#,
                source.as_str().into(),
                target.as_str().into(),
                platform.to_string().into(),
                platform.to_string().into(),
                query.text.clone().into(),
                query.take.into(),
                query.skip.into()
            ))
            .exec()
            .await?;
        let items = result
            .into_iter()
            .map(|res| TmWordPair {
                key: res.key,
                source: TmWord {
                    language: source.clone(),
                    content: res.src,
                },
                targets: vec![TmWord {
                    language: target.clone(),
                    content: res.dst,
                }],
            })
            .collect();
        let game_version = self
            .client
            .language_file()
            .find_first(vec![language_file::platform::equals(platform.clone())])
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
                        Word
                    WHERE
                        Word.language = {} AND
                        Word.platform = {} AND
                        Word.value COLLATE utf8mb4_unicode_ci LIKE CONCAT('%', {}, '%')
                "#,
                source.as_str().into(),
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
            .words
            .iter()
            .map(|word| word.namespace.clone())
            .collect();

        let (controller, client) = self
            .client
            ._transaction()
            .with_timeout(120 * 1000)
            .begin()
            .await?;

        client
            .language_file()
            .delete_many(vec![
                language_file::platform::equals(platform.clone()),
                language_file::language::equals(upload.language.as_str().to_string()),
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
                            platform.clone(),
                            namespace,
                            upload.language.as_str().to_string(),
                            upload.game_version.clone(),
                            now.clone(),
                            vec![],
                        )
                    })
                    .collect(),
            )
            .exec()
            .await?;

        client
            .word()
            .delete_many(vec![
                word::platform::equals(platform.clone()),
                word::language::equals(upload.language.as_str().to_string()),
            ])
            .exec()
            .await?;

        client
            .word()
            .create_many(
                upload
                    .words
                    .into_iter()
                    .map(|word| {
                        (
                            platform.clone(),
                            word.namespace,
                            upload.language.as_str().to_string(),
                            word.key,
                            word.value,
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
