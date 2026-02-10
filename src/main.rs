mod db;
mod etl;
use db::connect::conn;
use db::query::trx_table;
use etl::import::importer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = conn().await?;
    println!("DB READY 🚀");

    trx_table(&client).await?;
    println!("TABLE READY 🚀");

    importer("data/sales_trx.csv", &client).await?;

    Ok(())
}
