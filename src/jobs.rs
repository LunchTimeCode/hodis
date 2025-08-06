use actix_web::{Responder, get, web};

use crate::{
    db::{self, DB},
    index,
};

#[allow(unused)]
pub struct Job {
    id: String,
    title: String,
    description: String,
}

pub fn job_from_row(row: &turso::Row) -> Job {
    Job {
        id: row.get_value(0).unwrap().as_text().unwrap().to_owned(),
        title: row.get_value(1).unwrap().as_text().unwrap().to_owned(),
        description: row.get_value(2).unwrap().as_text().unwrap().to_owned(),
    }
}

pub async fn get_jobs(database: &DB) -> Vec<Job> {
    let db = db::get_connection(database).await;

    let mut rows: turso::Rows = db.query("SELECT * FROM jobs", ()).await.unwrap();

    let mut jobs = Vec::new();
    while let Some(row) = rows.next().await.unwrap() {
        let job = job_from_row(&row);
        jobs.push(job);
    }
    jobs
}

#[get("/jobs")]
pub async fn jobs_index(db: web::Data<DB>) -> impl Responder {
    let jobs = get_jobs(db.as_ref()).await;

    index::index(Some(render_jobs(jobs)))
}

pub fn render_jobs(jobs: Vec<Job>) -> maud::Markup {
    maud::html! {
        h1 {
            "Jobs"
        }
        (table(jobs))
    }
}

fn table(jobs: Vec<Job>) -> maud::Markup {
    maud::html! {
        div class="overflow-x-auto" {
        table class="table table-xs" {
            thead {
                tr {
                    th { "Verantwortlich" }
                    th { "Stellvertretung" }
                    th { "Ämtli" }
                    th { "Beschreibung" }
                }
            }
            tbody {
                @for job in jobs {
                    tr {
                        td { "no name" }
                        td { "no name" }
                        td { (job.title) }
                        td { (job.description) }
                    }
                }
            }
        }
    }
    }
}
