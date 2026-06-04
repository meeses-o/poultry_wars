use crate::utils::database::create_database_client;
use crate::screens::login::LoginRecord;

pub async fn update_player_dbs(info: Vec<LoginRecord>) -> Result<(), Box<dyn std::error::Error>> {
    if info.len() < 2 {
        return Err("Expected at least 2 login records".into());
    }
    
    let client = create_database_client();
    
    client.update_record_by_id("poultrywars", info[0].id as i64, "money", &serde_json::json!(info[0].money)).await?;
    client.update_record_by_id("poultrywars", info[0].id as i64, "rank", &serde_json::json!(info[0].rank)).await?;
    client.update_record_by_id("poultrywars", info[1].id as i64, "money", &serde_json::json!(info[1].money)).await?;
    client.update_record_by_id("poultrywars", info[1].id as i64, "rank", &serde_json::json!(info[1].rank)).await?;
    
    Ok(())
}

// pub async fn update_player_dbs(info: Vec<LoginRecord>) -> Result<(), Box<dyn std::error::Error>> {
//     if info.len() < 2 {
//         return Err("Expected at least 2 login records".into());
//     }
    
//     let client = create_database_client();
//     let table = "poultrywars"; // Verify this is your exact table name!

//     println!("{:?}", info);
    
//     for i in 0..2 {
//         let id = info[i].id as i64;
        
//         let money_count = client.update_record_by_id(table, id, "money", &serde_json::json!(info[i].money)).await?;
//         if money_count == 0 {
//             println!("Warning: 0 rows updated for money! Does ID {} exist in table '{}'?", id, table);
//         } else {
//             println!("Successfully updated money for ID {}", id);
//         }

//         let rank_count = client.update_record_by_id(table, id, "rank", &serde_json::json!(info[i].rank)).await?;
//         if rank_count == 0 {
//             println!("Warning: 0 rows updated for rank! Does ID {} exist in table '{}'?", id, table);
//         } else {
//             println!("Successfully updated rank for ID {}", id);
//         }
//     }
    
//     Ok(())
// }