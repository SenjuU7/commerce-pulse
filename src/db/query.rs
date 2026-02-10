use tokio_postgres::Client;

pub async fn trx_table(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    client
        .execute(
            "
        CREATE TABLE IF NOT EXISTS sales_transactions (
            TransactionNo VARCHAR(50) NOT NULL,
            Date TIMESTAMP NOT NULL,
            ProductNo VARCHAR(50) NOT NULL,
            ProductName VARCHAR(255) NOT NULL,
            Price DOUBLE PRECISION NOT NULL,
            Quantity INTEGER NOT NULL,
            CustomerNo VARCHAR(50),
            Country VARCHAR(100),

            PRIMARY KEY (TransactionNo, ProductNo)
        );
        ",
            &[],
        )
        .await?;

    Ok(())
}
