use std::{collections::HashSet, env};

use sqlx::{
    postgres::PgPoolOptions,
    query,
    types::chrono::{FixedOffset, TimeZone, Utc},
    Pool, Postgres,
};
use thiserror::Error;

use crate::db::{
    Pagination, SearchTmQuery, SearchTmResponse, TmDatabase, TmEntry, TmEntryPair, Upload,
};
#[derive(Clone, sqlx::Type)]
#[sqlx(transparent)]
struct MinecraftPlatform(String);

#[derive(Debug, Error)]
pub enum SqlxDatabaseError {
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("env var error: {0}")]
    EnvVarError(#[from] env::VarError),
}

pub struct SqlxDatabase {
    pool: Pool<Postgres>,
}

impl SqlxDatabase {
    pub async fn connect() -> Result<SqlxDatabase, SqlxDatabaseError> {
        Ok(SqlxDatabase {
            pool: PgPoolOptions::new()
                .max_connections(5)
                .connect(&env::var("DATABASE_URL")?)
                .await?,
        })
    }
}

impl TmDatabase for SqlxDatabase {
    type Error = SqlxDatabaseError;

    async fn search(&self, query: SearchTmQuery) -> Result<SearchTmResponse, Self::Error> {
        let res = query!(
            r#"
                SELECT
                    t1.key AS "key", src, dst AS "dst?"
                FROM
                    (
                        SELECT "key", namespace, value AS src FROM
                            public."Entry" t1
                        WHERE
                            language = $1 AND
                            platform::text = $3 AND
                            value ILIKE CONCAT('%', ($4::text), '%')
                    ) AS t1
                    LEFT JOIN
                    (
                        SELECT "key", namespace, value AS dst FROM
                            public."Entry" t2
                        WHERE
                            language = $2 AND
                            platform::text = $3
                    ) AS t2
                    ON
                        t1.key = t2.key AND
                        t1.namespace = t2.namespace
                ORDER BY "key" ASC
                LIMIT $5
                OFFSET $6
            "#,
            query.source.id(),
            query.target.id(),
            query.platform.id(),
            query.text,
            query.take as i32,
            query.skip as i32,
        )
        .fetch_all(&self.pool)
        .await?;
        let items = res
            .into_iter()
            .map(|res| TmEntryPair {
                key: res.key,
                source: TmEntry {
                    language: query.source.clone(),
                    content: res.src,
                },
                targets: res
                    .dst
                    .map(|content| TmEntry {
                        language: query.target.clone(),
                        content,
                    })
                    .into_iter()
                    .collect(),
            })
            .collect();
        let game_version = query!(
            r#"
                SELECT game_version FROM "LanguageFile"
                WHERE platform::text = $1
            "#,
            query.platform.id()
        )
        .fetch_one(&self.pool)
        .await?
        .game_version;

        let total = query!(
            r#"
                SELECT
                    COUNT(*) AS count
                FROM
                    "Entry"
                WHERE
                    language = $1 AND
                    platform::text = $2 AND
                    value ILIKE CONCAT('%', $3::text, '%')
            "#,
            query.source.id(),
            query.platform.id(),
            query.text.clone()
        )
        .fetch_one(&self.pool)
        .await?
        .count
        .unwrap_or(0) as usize;
        Ok(SearchTmResponse {
            game_version,
            list: Pagination { total, items },
        })
    }

    async fn upload(&self, upload: Upload) -> Result<(), Self::Error> {
        let now = FixedOffset::east_opt(9 * 60 * 60)
            .unwrap()
            .from_utc_datetime(&Utc::now().naive_utc());

        let namespaces: HashSet<_> = upload
            .entries
            .iter()
            .map(|entry| entry.namespace.clone())
            .collect();

        let mut tx = self.pool.begin().await?;

        query!(
            r#"
                DELETE FROM "Entry"
                WHERE
                    platform::text = $1 AND
                    language = $2
            "#,
            upload.platform.id(),
            upload.language.id(),
        )
        .execute(&mut *tx)
        .await?;

        query!(
            r#"
                DELETE FROM "LanguageFile"
                WHERE
                    platform::text = $1 AND
                    language = $2
            "#,
            upload.platform.id(),
            upload.language.id(),
        )
        .execute(&mut *tx)
        .await?;

        let platforms = vec![MinecraftPlatform(upload.platform.id().to_owned()); namespaces.len()];
        let namespaces = Vec::from_iter(namespaces.into_iter());
        let languages = vec![upload.language.id().to_owned(); namespaces.len()];
        let game_versions = vec![upload.game_version.clone(); namespaces.len()];
        let latest_updates = vec![now.naive_utc(); namespaces.len()];
        query!(
            r#"
                INSERT INTO "LanguageFile"
                (platform, namespace, language, game_version, latest_update)
                SELECT * FROM UNNEST($1::"MinecraftPlatform"[], $2::text[], $3::text[], $4::text[], $5::timestamp[])
                RETURNING (platform, namespace, language, game_version, latest_update)
            "#,
            &platforms as &[MinecraftPlatform],
            &namespaces ,
            &languages ,
            &game_versions,
            &latest_updates,
        ).fetch_one(&mut *tx).await?;

        let (namespaces, (keys, values)): (Vec<_>, (Vec<_>, Vec<_>)) = upload
            .entries
            .into_iter()
            .map(|entry| (entry.namespace, (entry.key, entry.value)))
            .unzip();
        let platforms = vec![MinecraftPlatform(upload.platform.id().to_owned()); keys.len()];
        let languages = vec![upload.language.id().to_owned(); keys.len()];

        query!(
            r#"
                INSERT INTO "Entry"
                (platform, namespace, language, key, value)
                SELECT * FROM UNNEST($1::"MinecraftPlatform"[], $2::text[], $3::text[], $4::text[], $5::text[])
                RETURNING (platform, namespace, language, key, value)
            "#,
            &platforms as &[MinecraftPlatform],
            &namespaces,
            &languages,
            &keys,
            &values,
        )
        .fetch_one(&mut *tx).await?;

        tx.commit().await?;
        Ok(())
    }
}
