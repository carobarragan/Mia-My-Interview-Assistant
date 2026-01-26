use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Cargar variables de entorno
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL debe estar configurada en .env");

    println!("Intentando conectar a: {}", database_url);

    // Crear pool de conexiones
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("✅ Conexión exitosa a PostgreSQL!");

    // Probar una query simple
    let result: (i32,) = sqlx::query_as("SELECT 1 + 1").fetch_one(&pool).await?;

    println!("✅ Query de prueba: 1 + 1 = {}", result.0);

    Ok(())
}
