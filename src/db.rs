use sqlx::{Pool, Postgres};
use std::env;

pub type PgPool = Pool<Postgres>;

pub async fn connect_pg() -> Result<PgPool, sqlx::Error>{
    let db_url = env::var("DATABASE_URL").expect("URL MUST BE SET !!");
    let pool = Pool::<Postgres>::connect(&db_url).await?;
    Ok(pool)
}