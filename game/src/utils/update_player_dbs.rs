use crate::utils::database::create_database_client;
use crate::screens::login::LoginRecord;

pub async fn update_player_dbs(info: Vec<LoginRecord>) -> Result<(), Box<dyn std::error::Error>> {
    if info.len() < 2 {
        return Err("Expected at least 2 login records".into());
    }
    
    let client = create_database_client();
    
    // Wrap the values in the json! macro
    client.update_record_by_id("poultrywars", info[0].id as i64, "money", &serde_json::json!(info[0].money)).await?;
    client.update_record_by_id("poultrywars", info[0].id as i64, "rank", &serde_json::json!(info[0].rank)).await?;
    client.update_record_by_id("poultrywars", info[1].id as i64, "money", &serde_json::json!(info[1].money)).await?;
    client.update_record_by_id("poultrywars", info[1].id as i64, "rank", &serde_json::json!(info[1].rank)).await?;
    
    Ok(())
}