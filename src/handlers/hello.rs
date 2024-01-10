use actix_web::{HttpResponse, Responder, web};
use crate::entities::hello::Hello;

pub fn handler_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(hello))
    );

    cfg.service(
        web::resource("/{name}")
            .route(web::get().to(greetings))
    );

}

async fn hello() -> impl Responder {
    HttpResponse::Ok().json(Hello {
        message: "Hello, World!".to_string()
    })
}

async fn greetings(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(Hello {
        message: format!("Hello, {}!", name.into_inner()),
    })
}