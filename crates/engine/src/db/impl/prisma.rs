use async_trait::async_trait;
use prisma_client_rust::{
    chrono::{FixedOffset, TimeZone, Utc},
    raw, NewClientError, QueryError,
};
use serde::Deserialize;

use crate::{
    db::{
        MinecraftPlatform, Pagination, SearchTmQuery, SearchTmResultEntry, TmDatabase, TmEntry,
        Upload,
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

    async fn search(
        &self,
        query: SearchTmQuery,
    ) -> Result<Pagination<SearchTmResultEntry>, Self::Error> {
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
                        t1.value LIKE CONCAT('%', {}, '%')
                    ORDER BY t1.key ASC
                    LIMIT {}
                    OFFSET {}
                "#,
                source.as_str().into(),
                target.as_str().into(),
                query.text.clone().into(),
                query.take.into(),
                query.skip.into()
            ))
            .exec()
            .await?;
        let items = result
            .into_iter()
            .map(|res| SearchTmResultEntry {
                key: res.key,
                source: TmEntry {
                    language: source.clone(),
                    content: res.src,
                },
                targets: vec![TmEntry {
                    language: target.clone(),
                    content: res.dst,
                }],
            })
            .collect();
        let total = self
            .client
            .word()
            .count(vec![word::value::contains(query.text)])
            .exec()
            .await?;
        Ok(Pagination {
            total: total as usize,
            items,
        })
    }

    async fn upload(&self, upload: Upload) -> Result<(), Self::Error> {
        let (controller, client) = self
            .client
            ._transaction()
            .with_timeout(120 * 1000)
            .begin()
            .await?;

        let now = FixedOffset::east_opt(9 * 60 * 60)
            .unwrap()
            .from_utc_datetime(&Utc::now().naive_utc());

        client
            .language_file()
            .upsert(
                language_file::UniqueWhereParam::PlatformFilenameEquals(
                    into_prisma_minecraft_platform(upload.platform.clone()),
                    upload.filename.clone(),
                ),
                language_file::create(
                    into_prisma_minecraft_platform(upload.platform.clone()),
                    upload.filename.clone(),
                    upload.game_version.clone(),
                    now.clone(),
                    upload.language.as_str().to_string(),
                    vec![],
                ),
                vec![
                    language_file::SetParam::SetGameVersion(upload.game_version.clone()),
                    language_file::SetParam::SetLatestUpdate(now.clone()),
                    language_file::SetParam::SetLanguage(upload.language.as_str().to_string()),
                ],
            )
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
                            into_prisma_minecraft_platform(upload.platform.clone()),
                            upload.filename.clone(),
                            word.key,
                            word.value,
                            upload.language.as_str().to_string(),
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
