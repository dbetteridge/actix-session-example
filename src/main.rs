use actix_cors::Cors;
use actix_session::CookieSession;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use handlers::{add_card, add_trade, add_user, cards, login, trades};
use tokio_postgres::NoTls;

mod config;
mod db;
mod errors;
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .wrap(
                Cors::default()
                    .allow_any_method()
                    .allow_any_origin()
                    .allow_any_header()
                    .max_age(3600),
            )
            .service(
                web::resource("/trades")
                    .route(web::get().to(trades))
                    .route(web::post().to(add_trade)),
            )
            .service(
                web::resource("/cards")
                    .route(web::get().to(cards))
                    .route(web::post().to(add_card)),
            )
            .service(web::resource("/register").route(web::post().to(add_user)))
            .service(web::resource("/login").route(web::post().to(login)))
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
