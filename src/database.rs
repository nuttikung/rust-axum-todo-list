use crate::setting::Setting;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::{sync::Arc, time::Duration};

pub async fn conn_getting(setting: Arc<Setting>) -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = setting.database.url_getting();
    let connection_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("can't connect to database");

    // region :      --- Migration Script
    sqlx::migrate!().run(&connection_pool).await?;
    // end region :  --- Migration Script

    Ok(connection_pool)
}
