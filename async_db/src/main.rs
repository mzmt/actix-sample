use std::io;

use actix_web::{middleware, web, App, Error as AWError, HttpResponse, HttpServer};
use futures::future::join_all;
use r2d2_sqlite::{self, SqliteConnectionManager};

mod db;
use db::{Pool, Queries};

async fn asyncio_weather(db: web::Data<Pool>) -> Result<HttpResponse, AWEoor> {
    let result = vec![
        db::execute(&db, Queries::GetTopTenHottestYears).await?,
        db::execute(&db, Queries::GetTopTenColdestYears).await?,
        db::execute(&db, Queries::GetTopTenHottestMonths).await?,
        db::execute(&db, Queries::GetTopTenColdestMonths).await?,
    ];

    Ok(HttpResponse::Ok().json(result))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let manager = SqliteConnectionManager::file("weather.db");
    let pool = Pool::new(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/asyncio_weather").route(web::get().to(asyncio_weather)))
            .service(web::resource("/parallel_weather").route(web::get().to(parallel_weather)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
