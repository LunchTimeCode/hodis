use actix_web::{App, HttpServer, middleware::Logger, web};
use env_logger::Env;
use std::env;

mod db;
mod jobs;
mod tec;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let log_level = env::var("LOG_LEVEL").unwrap_or("INFO".to_string());
    env_logger::init_from_env(Env::default().default_filter_or(log_level));
    let settings = db::Settings::from_env();

    let db = db::init(&settings).await;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(tec::healthz)
            .app_data(web::Data::new(db.clone()))
    })
    .bind(("127.0.0.1", 9898))?
    .run()
    .await
}
