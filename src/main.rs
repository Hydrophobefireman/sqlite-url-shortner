mod db;
mod util;
use db::{
    delete_url, get_all_urls, get_url_by_slug, increment_visit_count, init_db, insert_url,
    update_url, Url, DB,
};
use rocket::form::Form;
use rocket::fs::{relative, FileServer, Options};
use rocket::serde::json::Json;
use rocket::{response::Redirect, routes};
use rocket_dyn_templates::{context, Template};
use std::env;
use util::generate_slug;
#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    println!("Launching!");
    init_db().unwrap();
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                create_url,
                redirect,
                all_urls,
                edit_url,
                detail,
                detail_api,
                delete_url_route
            ],
        )
        .mount(
            "/.static",
            FileServer::new(relative!("static"), Options::DotFiles | Options::Index),
        )
        .attach(Template::fairing())
}

#[get("/")]
fn index() -> Template {
    let conn = DB.lock().unwrap();
    let urls = get_all_urls(&conn).unwrap();
    Template::render("index", context! { urls })
}

#[post("/.create", data = "<data>")]
fn create_url(data: Form<CreateData>) -> Result<Redirect, Json<CreateError>> {
    let data = data.into_inner();
    let mut conn = DB.lock().unwrap();
    let slug = if (&data.slug).len() > 0 {
        (&data.slug).clone()
    } else {
        generate_slug(6)
    };

    let x = slug.chars().next().unwrap_or('.');
    if x == '.' {
        return Err(Json(CreateError {
            error: ("Invalid slug".into()),
        }));
    }
    match insert_url(&mut conn, &data.url, &slug) {
        Ok(_) => Ok(Redirect::found(format!("/.detail/{slug}"))),
        Err(e) => Err(Json(CreateError {
            error: format!("Error creating URL: {e}"),
        })),
    }
}

#[get("/<slug>")]
fn redirect(slug: &str) -> Option<Redirect> {
    let conn = DB.lock().unwrap();
    if let Ok(url) = get_url_by_slug(&conn, &slug) {
        increment_visit_count(&conn, &slug).unwrap();
        Some(Redirect::found(url.original_url))
    } else {
        None
    }
}

#[get("/.detail/<slug>")]
fn detail(slug: &str) -> Option<Template> {
    let conn = DB.lock().unwrap();
    if let Ok(url) = get_url_by_slug(&conn, &slug) {
        Some(Template::render("detail", context! { url }))
    } else {
        None
    }
}
#[get("/.detail/api/<slug>")]
fn detail_api(slug: &str) -> Json<Option<Url>> {
    let conn = DB.lock().unwrap();
    if let Ok(url) = get_url_by_slug(&conn, &slug) {
        Json(Some(url))
    } else {
        Json(None)
    }
}

#[get("/.all")]
fn all_urls() -> Json<Vec<Url>> {
    let conn = DB.lock().unwrap();
    let urls = get_all_urls(&conn).unwrap();
    Json(urls)
}
#[post("/.edit/<slug>", data = "<data>")]
fn edit_url(slug: &str, data: Form<EditData>) -> Result<Redirect, Json<EditError>> {
    let data = data.into_inner();
    let conn = DB.lock().unwrap();

    match update_url(&conn, slug, &data.slug.as_str(), &data.url.as_str()) {
        Ok(rows_affected) if rows_affected > 0 => {
            Ok(Redirect::found(format!("/.detail/{}", data.slug)))
        }
        Ok(_) => Err(Json(EditError {
            message: "No changes made".into(),
        })),
        Err(e) => Err(Json(EditError {
            message: format!("Error updating url: {e}"),
        })),
    }
}
#[post("/.delete/<slug>")]
fn delete_url_route(slug: &str) -> Result<Template, Json<EditError>> {
    let conn = DB.lock().unwrap();
    if let Ok(url) = get_url_by_slug(&conn, &slug) {
        match delete_url(&conn, slug) {
            Ok(rows_affected) if rows_affected > 0 => {
                Ok(Template::render("deleted", context! { url }))
            }
            Ok(_) => Err(Json(EditError {
                message: "No changes made".into(),
            })),
            Err(e) => Err(Json(EditError {
                message: format!("Error updating URL {e}"),
            })),
        }
    } else {
        return Err(Json(EditError {
            message: "URL Does not exist!".into(),
        }));
    }
}

#[derive(serde::Serialize)]
struct CreateError {
    error: String,
}

#[derive(FromForm, Debug)]
struct CreateData {
    url: String,
    slug: String,
}

#[derive(FromForm)]
struct EditData {
    url: String,
    slug: String,
}

#[derive(serde::Serialize)]
struct EditError {
    message: String,
}
