use actix_web::{ middleware, web, App, HttpResponse, HttpServer };

use std::error::Error;

pub async fn http_server(config: &toml::value::Table) -> Result<(), Box<dyn Error>> {
    let host = config["http"]["host"].as_str().unwrap();
    let port = config["http"]["port"].as_integer().unwrap() as u16;

    println!("Starting HTTP server on {}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web
                    ::resource("/")
                    .route(web::get().to(|| async { HttpResponse::Ok().body("Hello world!") }))
            )
    })
        .bind(format!("{}:{}", host, port))?
        .run().await?;
    Ok(())
}