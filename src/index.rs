use actix_web::{Responder, get, web};

use crate::{
    db::DB,
    jobs::{get_jobs, render_jobs},
};

#[get("/")]
pub async fn index_route(db: web::Data<DB>) -> impl Responder {
    let jobs = get_jobs(db.as_ref()).await;

    index(Some(render_jobs(jobs)))
}

pub fn index(content: Option<maud::Markup>) -> maud::Markup {
    let content = content.unwrap();
    maud::html! {
        (maud::DOCTYPE)
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            title {
                "Hodis"
            }
            (js("/assets/tw.js"))
            (js("/assets/theme-switcher.js"))
            (js("/assets/htmx.js"))
            (css("/assets/daisy.css"))
            (css("/assets/themes.css"))
            (css("/assets/app.css"))

        }
        body hx-boost="true" {
            (js("/assets/htmxListener.js"))
            (js("/assets/htmx-reload.js"))

            div class="h-screen" {
                div class="shadow-lg" {
                    (navbar())
                }
                div class="flex-1"{
                    (content)
                }

            }
        }
    }
}

pub fn css(path: impl Into<String>) -> maud::Markup {
    let path: String = path.into();
    maud::html! {link href=(path) rel="stylesheet" type="text/css";}
}

pub fn js(path: impl Into<String>) -> maud::Markup {
    let path: String = path.into();
    maud::html! {script src=(path) {}}
}

pub fn navbar() -> maud::Markup {
    maud::html! {
        div class="navbar bg-base-100 shadow-sm" {
            div class="flex-1" {
                a class="btn btn-ghost text-xl" {
                    "Hodis"
                }
            }
            div class="flex-none" {
                ul class="menu menu-horizontal px-1" {
                    li {
                        a href="/jobs"{
                            "Ämtli"
                        }
                    }
                }
            }
        }
    }
}
