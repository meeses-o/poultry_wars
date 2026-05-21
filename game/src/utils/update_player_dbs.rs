use crate::utils::database::{create_database_client, DatabaseTable};
use crate::screens::login::LoginRecord;

pub fn update_player_dbs(info: Vec<LoginRecord>) {
    let client = create_database_client();
    client.update_record_by_id("poultrywars", info[0].id, , value)
}