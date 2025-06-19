use sqlx::{PgPool, QueryBuilder};

use crate::api::http::dto::storage_settings_dto::UpdateStorageSettingsDto;
use crate::domain::entity::storage_settings_entity::StorageSettingsEntity;

pub struct StorageRepository {
    pub pool: PgPool,
}

impl StorageRepository {
    pub(crate) async fn get_settings(&self) -> Result<StorageSettingsEntity, anyhow::Error> {
        let row = sqlx::query!(
            r#"
            SELECT max_storage_days, video_interval_minutes, screenshots_interval_minutes
            FROM "StorageSettings"
            LIMIT 1
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(StorageSettingsEntity {
            max_storage_days: row.max_storage_days.unwrap(),
            video_interval_minutes: row.video_interval_minutes.unwrap(),
            screenshots_interval_minutes: row.screenshots_interval_minutes.unwrap(),
        })
    }

    pub async fn update_settings_partial(
        &self,
        update: UpdateStorageSettingsDto,
    ) -> Result<(), sqlx::Error> {
        let mut qb = QueryBuilder::new("UPDATE storage_settings SET ");

        let mut separated = qb.separated(", ");

        if let Some(days) = update.max_storage_days {
            separated.push("max_storage_days = ").push_bind(days as i32);
        }
        if let Some(video_interval) = update.video_interval_minutes {
            separated.push("video_interval_minutes = ").push_bind(video_interval as i32);
        }
        if let Some(screenshots_interval) = update.screenshots_interval_minutes {
            separated.push("screenshots_interval_minutes = ").push_bind(screenshots_interval as i32);
        }

        qb.push(" WHERE id = 1"); // или какой у тебя критерий обновления

        let query = qb.build();

        query.execute(&self.pool).await?;

        Ok(())
    }
}
