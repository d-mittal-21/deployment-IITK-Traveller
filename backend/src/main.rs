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

    println!("Starting the server...");

    HttpServer::new(|| App::new()
    .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
    .service(interpret_handler))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    
}


// fn main(){
//     let code = "start, 0, iit_gate_in_1
//     iit_gate_in_1, 0, lecture_hall_eq
//     lecture_hall_eq_f, 0, southern_labs_1
//     southern_labs_1, 0, lecture_hall_eq
//     lecture_hall_eq_t, 0, finish";

//     let input = "8000";

//     let out = interpret(code, input).unwrap();

//     println!("{}", out);
// }
