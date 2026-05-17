use tokio::sync::mpsc;

use super::Player;
use crate::message::Message;

const BASE_URL: &str = "https://secure.runescape.com/m=hiscore_oldschool/index_lite.json?player=";

pub async fn fetch_player(tx: mpsc::Sender<Message>, player_name: String) {
    let response = reqwest::get(format!("{}{}", BASE_URL, player_name))
        .await
        .unwrap();

    let player_data_json = response.text().await.unwrap();
    let player_data: Player = serde_json::from_str(&player_data_json).unwrap();

    if tx.send(Message::PlayerLoaded(player_data)).await.is_err() {
        return;
    }
}
