use sqlx::{Error, PgPool, QueryBuilder};

use crate::api::http::dto::storage_settings_dto::UpdateStorageSettingsDto;
use crate::domain::entity::storage_settings_entity::StorageSettingsEntity;

pub struct StorageRepository {
    pub pool: PgPool,
}

impl StorageRepository {
    pub async fn get_settings(&self) -> Result<StorageSettingsEntity, anyhow::Error> {
        let row = sqlx::query!(
            r#"
            SELECT max_storage_days, video_interval_minutes, screenshots_interval_minutes
            FROM "storage_settings"
            LIMIT 1
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(StorageSettingsEntity {
            max_storage_days: row.max_storage_days,
            video_interval_minutes: row.video_interval_minutes,
            screenshots_interval_minutes: row.screenshots_interval_minutes,
        })
    }

    pub async fn update_settings_partial(
        &self,
        update: UpdateStorageSettingsDto,
    ) -> Result<StorageSettingsEntity, sqlx::Error> {
        let mut qb = QueryBuilder::new("UPDATE storage_settings SET ");

        let mut has_any = false;

        if let Some(days) = update.max_storage_days {
            qb.push("max_storage_days = ").push_bind(days).push(",");
            has_any = true;
        }
        if let Some(video_interval) = update.video_interval_minutes {
            qb.push("video_interval_minutes = ").push_bind(video_interval).push(",");
            has_any = true;
        }
        if let Some(screenshots_interval) = update.screenshots_interval_minutes {
            qb
                .push("screenshots_interval_minutes = ")
                .push_bind(screenshots_interval);
            has_any = true;
        }

        if !has_any {
            return Err(Error::RowNotFound);
        }

        qb.push(" WHERE id = 1 RETURNING id, max_storage_days, video_interval_minutes, screenshots_interval_minutes");

        let query = qb.build_query_as::<StorageSettingsEntity>();

        let updated = query.fetch_one(&self.pool).await?;

        Ok(updated)
    }


}
