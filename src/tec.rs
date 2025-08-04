use actix_web::{HttpResponse, Responder, get, web};

use crate::db::{self, DB};

pub async fn get_jobs(database: &DB) -> bool {
    let db = db::get_connection(database).await;
    let res = db.query("SELECT * from jobs", ()).await;

    match res {
        Ok(_) => true,
        Err(e) => {
            log::error!("{e}");
            false
        }
    }
}

#[get("/healthz")]
pub async fn healthz(db: web::Data<DB>) -> impl Responder {
    if get_jobs(db.as_ref()).await {
        HttpResponse::Ok()
    } else {
        HttpResponse::NotAcceptable()
    }
}
