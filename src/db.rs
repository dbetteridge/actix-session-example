use std::ops::Add;

use crate::{
    errors::MyError,
    models::Session,
    models::Trade,
    models::{Card, User},
};
use chrono::{DateTime, Utc};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
extern crate bcrypt;
use bcrypt::verify;

pub async fn add_user(client: &Client, user_info: User) -> Result<User, MyError> {
    let _stmt = include_str!("../sql/add_user.sql");
    println!("Read statement");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &user_info.email,
                &user_info.name,
                &user_info.username,
                &user_info.password,
            ],
        )
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound) // more applicable for SELECTs
}

pub async fn login(client: &Client, user_info: User) -> Result<Session, MyError> {
    let _stmt = "
    SELECT * 
    FROM testing.users 
    WHERE email = $1;";
    let stmt = client.prepare(&_stmt).await.unwrap();

    let user: User = client
        .query(&stmt, &[&user_info.email])
        .await?
        .first()
        .map(|row| User::from_row_ref(row).unwrap())
        .ok_or(MyError::NotFound)
        .unwrap();
    let password = user.password.clone();
    let is_valid = verify(user_info.password, &password.clone()).unwrap();

    if !is_valid {
        let error: std::result::Result<Session, MyError> = Err(MyError::AuthError(String::from(
            "Username or password is incorrect",
        )));
        return error;
    }
    let session = client
        .query_one(
            "
            INSERT INTO testing.sessions (\"user\", \"token\") 
            VALUES ($1, $2) 
            ON CONFLICT ON CONSTRAINT sessions_pkey 
            DO UPDATE SET (token,created) = ($2,$3) RETURNING *;",
            &[&user.email, &String::from("token"), &Utc::now()],
        )
        .await
        .map_err(|err| println!("{}", err))
        .unwrap();

    match Session::from_row_ref(&session) {
        Err(_) => Err(MyError::NotFound),
        Ok(s) => Ok(s),
    }
}

pub async fn trades(client: &Client, user: &User) -> Result<Vec<Trade>, MyError> {
    let _statement = "
    SELECT * 
    FROM testing.trades 
    WHERE trades.sender = $1";
    let statement = client.prepare(_statement).await.unwrap();

    match client
        .query(&statement, &[&user.email])
        .await?
        .iter()
        .map(|row| Trade::from_row_ref(row).unwrap())
        .collect::<Vec<Trade>>()
    {
        trades => Ok(trades),
    }
}

pub async fn add_trade(client: &Client, trade_info: Trade) -> Result<Trade, MyError> {
    let _update = "UPDATE testing.cards SET owner = $1;";
    let update = client.prepare(_update).await.unwrap();

    client.query(&update, &[&trade_info.recipient]).await?;

    let _stmt = "
    INSERT INTO testing.trades (sender, recipient, card) VALUES ($1, $2, $3) RETURNING *;
    ";
    let stmt = client.prepare(_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[&trade_info.sender, &trade_info.recipient, &trade_info.card],
        )
        .await?
        .iter()
        .map(|row| Trade::from_row_ref(row).unwrap())
        .collect::<Vec<Trade>>()
        .pop()
        .ok_or(MyError::NotFound) // more applicable for SELECTs
}

pub async fn add_card(client: &Client, card_info: Card) -> Result<Card, MyError> {
    let _stmt =
        "INSERT INTO testing.cards (owner, name, description) VALUES ($1, $2, $3) RETURNING *;";
    let stmt = client.prepare(_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[&card_info.owner, &card_info.name, &card_info.description],
        )
        .await?
        .iter()
        .map(|row| Card::from_row_ref(row).unwrap())
        .collect::<Vec<Card>>()
        .pop()
        .ok_or(MyError::NotFound) // more applicable for SELECTs
}

pub async fn cards(client: &Client, user: &User) -> Result<Vec<Card>, MyError> {
    let _statement = "
    SELECT * 
    FROM testing.cards 
    WHERE cards.owner = $1";
    let statement = client.prepare(_statement).await.unwrap();

    match client
        .query(&statement, &[&user.email])
        .await?
        .iter()
        .map(|row| Card::from_row_ref(row).unwrap())
        .collect::<Vec<Card>>()
    {
        cards => Ok(cards),
    }
}

pub async fn auth(client: &Client, session_token: String) -> Result<User, MyError> {
    let _stmt = "
    SELECT * 
    FROM testing.users 
    JOIN testing.sessions
    ON sessions.user = users.email
    WHERE token = $1;";
    let stmt = client.prepare(&_stmt).await.unwrap();

    match client
        .query(&stmt, &[&session_token])
        .await?
        .first()
        .map(|row| {
            (
                User::from_row_ref(row).unwrap(),
                Session::from_row_ref(row).unwrap(),
            )
        })
        .ok_or(MyError::AuthError(String::from("No valid session")))
    {
        Ok(r) => {
            let created: DateTime<Utc> = r.1.created;
            let now = Utc::now();
            let hour = chrono::Duration::milliseconds(60 * 60 * 1000);
            let has_not_expired = now.le(&created.add(hour));
            if has_not_expired {
                Ok(r.0)
            } else {
                Err(MyError::AuthError(String::from("No valid session")))
            }
        }
        Err(err) => Err(err),
    }
}
