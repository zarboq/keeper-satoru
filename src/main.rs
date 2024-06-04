use std::{
    collections::HashMap,
};

use keeper_satoru::{listen_db::{start_listening, ActionType}, order::types::PayloadOrder};

#[tokio::main]
async fn main() {
    let pool = sqlx::PgPool::connect(
        "postgres://postgres:123@localhost:5432/zohal",
    )
    .await
    .unwrap();

    let channels: Vec<&str> = vec!["order_update"];

    let hm: HashMap<String, String> = HashMap::new();

    let call_back = |payload: PayloadOrder| {
        match payload.action_type {
            ActionType::INSERT => {
            }
            ActionType::UPDATE => {
            }
            ActionType::DELETE => {
            }
        };
    };
    println!("Keeper connected to DB and listening...");

    let _ = start_listening(&pool, channels, call_back).await;
}