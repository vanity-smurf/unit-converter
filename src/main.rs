mod models;
use crate::models::{Category, ConvertRequest, ConvertResponse};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

const HOST: &str = "127.0.0.1:8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/convert", web::get().to(convert)))
        .bind(HOST)?
        .run()
        .await
}

async fn convert(query: web::Query<ConvertRequest>) -> impl Responder {
    let convert_request = query.into_inner();

    let result = match convert_request.category {
        Category::Length => convert_request.convert_length(),
        Category::Weight => convert_request.convert_weight(),
        Category::Temperature => convert_request.convert_temperature(),
    };

    match result {
        Some(res) => HttpResponse::Ok().json(ConvertResponse { result: res }),
        None => HttpResponse::BadRequest().body("Invalid units"),
    }
}
