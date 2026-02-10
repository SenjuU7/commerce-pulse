use dotenv::dotenv;
use tokio_postgres::{Client, NoTls};

pub async fn conn() -> Result<Client, Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")?;

    let (client, connection) =
        tokio_postgres::connect(&db_url, NoTls).await?;

    // wajib jalan di background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    println!("Connected to the database. ✅");

    Ok(client)
}
