use std::fmt::Debug;

use serde::Deserialize;

use log::info;
use serde::de::DeserializeOwned;
use sqlx::error::Error;
use sqlx::postgres::PgListener;
use sqlx::Pool;
use sqlx::Postgres;

// An enum representing the types of database actions.
#[derive(Deserialize, Debug)]
pub enum ActionType {
    INSERT,
    UPDATE,
    DELETE,
}

// A struct representing the payload of a notification.
// @table: The table name in the database.
// @action_type: The type of action (using the ActionType enum).
// @key: The key of the affected data.
// @value: The value of the affected data.
#[derive(Deserialize, Debug)]
pub struct Payload {
    pub table: String,
    pub action_type: ActionType,
    pub key: String,
    pub value: String,
}

// Listens to notifications from PostgreSQL channels.
// @pool: A reference to a connection pool for PostgreSQL.
// @channels: A vector of channel names to listen to.
// @call_back: A callback function to handle the deserialized payload.
pub async fn start_listening<T: DeserializeOwned + Sized + Debug>(
    pool: &Pool<Postgres>,
    channels: Vec<&str>,
    call_back: impl Fn(T),
) -> Result<(), Error> {
    // Initiate the logger.
    env_logger::init();

    let mut listener: PgListener = PgListener::connect_with(pool).await.unwrap();
    listener.listen_all(channels).await?;
    loop {
        while let Some(notification) = listener.try_recv().await? {
            info!(
                "Getting notification with payload: {:?} from channel {:?}",
                notification.payload(),
                notification.channel()
            );

            let strr = notification.payload().to_owned();
            let payload: T = serde_json::from_str::<T>(&strr).unwrap();
            info!("des payload is {:?}", payload);

            call_back(payload);
        }
    }
}