#[path = "types.rs"]
mod types;

use actix_cors::Cors;
use actix_web::{ get, web, App, HttpResponse, HttpServer };
use json::object;
use std::error::Error;

struct AppState {
    cache: types::Table,
}

#[get("/size")]
async fn get_size(state: web::Data<AppState>) -> HttpResponse {
    let length = state.cache.lock().unwrap().len();
    let res = object! {
        "length" => length
    };
    HttpResponse::Ok().content_type("application/json").body(res.dump())
}

#[get("/data")]
async fn get_data(state: web::Data<AppState>) -> HttpResponse {
    let data = state.cache.lock().unwrap();
    // convert to json
    let res = data
        .iter()
        .map(|(k, v)| object! { k => v.to_string() })
        .collect::<Vec<json::JsonValue>>();

    let res = object! {
        "data" => res
    };

    HttpResponse::Ok().content_type("application/json").body(res.dump())
}

pub async fn http_server(
    config: &toml::value::Table,
    cache: types::Table
) -> Result<(), Box<dyn Error>> {
    let host = config["http"]["host"].as_str().unwrap();
    let port = config["http"]["port"].as_integer().unwrap() as u16;

    println!("Starting HTTP server on {}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(actix_web::middleware::Logger::default())
            .app_data(
                web::Data::new(AppState {
                    cache: cache.clone(),
                })
            )
            .service(get_size)
            .service(get_data)
    })
        .bind(format!("{}:{}", host, port))?
        .run().await?;

    Ok(())
}