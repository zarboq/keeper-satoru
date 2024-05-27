use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use keeper_satoru::listen_db::{start_listening, Payload, ActionType};

#[tokio::main]
async fn main() {
    let pool = sqlx::PgPool::connect(
        "postgres://passforyourbot:passforyourbot@localhost:5432/passforyourbot",
    )
    .await
    .unwrap();

    let channels = vec!["table_update"];

    let hm: HashMap<String, String> = HashMap::new();
    let constants = Arc::new(RwLock::new(hm));

    let call_back = |payload: Payload| {
        match payload.action_type {
            ActionType::INSERT => {
                let mut constants = constants.write().unwrap();
                constants.insert(payload.key, payload.value);
            }
            ActionType::UPDATE => {
                let mut constants = constants.write().unwrap();
                constants.insert(payload.key, payload.value);
            }
            ActionType::DELETE => {
                let mut constants = constants.write().unwrap();
                constants.remove(&payload.key);
            }
        };
        println!("constants: {:?}", constants);
        println!(" ");
    };

    let _ = start_listening(&pool, channels, call_back).await;
}