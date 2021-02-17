use crate::{
    db,
    errors::MyError,
    models::{Card, Trade, User},
};
use actix_web::{web, Error, HttpResponse};
use chrono::Utc;
use deadpool_postgres::{Client, Pool};

extern crate bcrypt;
use bcrypt::{hash, DEFAULT_COST};

pub async fn add_user(
    user: web::Json<User>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let mut user_info: User = user.into_inner();

    println!("{}", user_info.email);
    let hashed = hash(&user_info.password, DEFAULT_COST).unwrap();
    user_info.password = hashed;

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_user = db::add_user(&client, user_info).await?;
    println!("{:?}", new_user);
    Ok(HttpResponse::Ok().json(new_user))
}

pub async fn login(
    session: actix_session::Session,
    user: web::Json<User>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: User = user.into_inner();
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let db_session = db::login(&client, user_info).await?;
    session.set("session", db_session.token)?;
    Ok(HttpResponse::Ok().json(db_session.user))
}

pub async fn add_trade(
    session: actix_session::Session,
    trade: web::Json<Trade>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    match auth(session, db_pool.clone()).await {
        Ok(user) => {
            let trade_info: Trade = trade.into_inner();
            let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
            if trade_info.sender == user.email {
                let db_session = db::add_trade(&client, trade_info).await?;

                Ok(HttpResponse::Ok().json(db_session))
            } else {
                Err(MyError::AuthError(String::from("You are not the sender")))
            }
        }
        Err(_) => Err(MyError::AuthError(String::from("No user session"))),
    }
}

pub async fn trades(
    session: actix_session::Session,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    match auth(session, db_pool.clone()).await {
        Ok(user) => {
            let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
            let db_trades = db::trades(&client, &user).await?;
            Ok(HttpResponse::Ok().json(db_trades))
        }
        Err(err) => Err(err),
    }
}

pub async fn add_card(
    session: actix_session::Session,
    card: web::Json<Card>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    match auth(session, db_pool.clone()).await {
        Ok(user) => {
            let card_info: Card = card.into_inner();
            let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
            if card_info.owner == user.email {
                let db_session = db::add_card(&client, card_info).await?;
                Ok(HttpResponse::Ok().json(db_session))
            } else {
                Err(MyError::AuthError(String::from("You are not the owner")))
            }
        }
        Err(_) => Err(MyError::AuthError(String::from("No user session"))),
    }
}

pub async fn cards(
    session: actix_session::Session,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    match auth(session, db_pool.clone()).await {
        Ok(user) => {
            let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
            let db_cards = db::cards(&client, &user).await?;
            Ok(HttpResponse::Ok().json(db_cards))
        }
        Err(err) => Err(err),
    }
}

pub async fn auth(
    session: actix_session::Session,
    db_pool: web::Data<Pool>,
) -> Result<User, MyError> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError).unwrap();
    let session_token = session.get("session").unwrap().unwrap_or_default();
    db::auth(&client, session_token).await
}
