use crate::db::{self, DB};

#[allow(unused)]
pub struct Job {
    id: String,
    title: String,
    description: String,
}

#[allow(unused)]
pub async fn get_jobs(database: DB) -> Vec<Job> {
    let _db = db::get_connection(&database).await;

    vec![]
}
