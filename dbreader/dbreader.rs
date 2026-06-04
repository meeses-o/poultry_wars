mod database;

use database::create_database_client;
use database::DatabaseTable;
use core::error;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let client = create_database_client();
    let records = client.fetch_table("poultrywars").await;
    match records {
        Ok(records_ok) => {
            for record in &records_ok {
                client.update_record_by_id("poultrywars", record.id as i64, "chicken_clown", "f").await?;
            }
        }
        Err(e) => {
            println!("Fetch failed because: {:?}", e); 
        }
    }
    Ok(())
}
