#[warn(unused)]
use chrono::NaiveDate;
use csv::Reader;
use tokio_postgres::Client;

pub async fn importer(file_path: &str, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    println!("CSV file opened successfully.");

    for result in rdr.records() {
        let record = result?;

        
        let transaction_no = &record[0]; 
        let date_str = &record[1];       
        let product_no = &record[2];
        let product_name = &record[3];
        let price: f64 = record[4].trim().parse()?;
        let quantity: i32 = record[5].trim().parse()?;
        let customer_no = &record[6];
        let country = &record[7];

        let date = NaiveDate::parse_from_str(date_str, "%m/%d/%Y")?;
        let date_time = date.and_hms_opt(0, 0, 0).unwrap();

        client
            .execute(
                "
        INSERT INTO sales_transactions
        (TransactionNo, Date, ProductNo, ProductName, Price, Quantity, CustomerNo, Country)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
        ON CONFLICT (TransactionNo, ProductNo) DO NOTHING;
        ",
                &[
                    &transaction_no,
                    &date_time,
                    &product_no,
                    &product_name,
                    &price,
                    &quantity,
                    &customer_no,
                    &country,
                ],
            )
            .await?;
    }

    println!("Data imported successfully from CSV ✅");

    Ok(())
}
