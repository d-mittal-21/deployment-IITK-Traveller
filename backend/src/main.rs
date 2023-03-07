use lib::interpreter::interpreter::interpret;
use actix_web::{post, web, App, HttpServer, Responder, HttpResponse,};
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use actix_cors::Cors;

#[derive(Serialize,Deserialize)]
struct Payload {
    code: String,
    input: String
}

#[post("/interpret")]
async fn interpret_handler(data: web::Form<Payload>) -> impl Responder{
    let out = match interpret(&data.code, &data.input){
        Ok(s) => s,
        Err(e) => format!("Error: {}", e)
    };

    let response_data = json!({
        "output": out,
    });

    HttpResponse::Ok()
        .json(response_data)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| App::new()
    .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
    .service(interpret_handler))
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
    
}

