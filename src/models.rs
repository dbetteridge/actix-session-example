use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize, Debug)]
#[pg_mapper(table = "users")] // singular 'user' is a keyword..
pub struct User {
    pub email: String,
    pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, PostgresMapper, Serialize, Debug)]
#[pg_mapper(table = "sessions")] 
pub struct Session {
    pub user: String,
    pub token: String,
    pub created: DateTime<Utc>,
}

#[derive(Deserialize, PostgresMapper, Serialize, Debug)]
#[pg_mapper(table = "trades")] 
pub struct Trade {
    pub sender: String,
    pub recipient: String,
    pub card: i64,
}

#[derive(Deserialize, PostgresMapper, Serialize, Debug)]
#[pg_mapper(table = "cards")]
pub struct Card {
    pub owner: String,
    pub name: String,
    pub description: String,
}

