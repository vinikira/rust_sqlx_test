use sqlx::postgres::PgPoolOptions;

#[derive(Debug)]
struct User {
    id: i64,
    name: Option<String>,
    profile_id: Option<i64>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/db_test")
        .await?;

    let users = sqlx::query_as!(User, "SELECT id, name, profile_id from users;")
        .fetch_all(&pool)
        .await?;

    println!("{:#?}", users);

    Ok(())
}
