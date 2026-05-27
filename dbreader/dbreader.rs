mod database;

use database::create_database_client;
use database::DatabaseTable;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = create_database_client();

    println!("Reading full poultrywars table from the database...");

    let rows: Vec<DatabaseTable> = client.fetch_table("poultrywars").await?;
    if rows.is_empty() {
        println!("No rows found in poultrywars.");
        return Ok(());
    }

    println!("Found {} rows:\n", rows.len());
    for row in rows {
        println!("ID: {}", row.id);
        println!("username: {}", row.username);
        println!("password: {}", row.password);
        println!("rank: {}", row.rank);
        println!("money: {}", row.money);
        println!("---");
    }

    Ok(())
}
