use lib::interpreter::interpreter::interpret;
use actix_web::{post, web, App, HttpServer, Responder, HttpResponse, get};
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

#[derive(Serialize,Deserialize)]
struct Payload {
    code: String,
    input: String
}

#[post("/interpret")]
async fn interpret_handler(data: web::Form<Payload>) -> impl Responder{
    let out = interpret(&data.code, &data.input);

    let response_data = json!({
        "output": out,
    });

    HttpResponse::Ok()
        .json(response_data)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // let code = "start, 0, iit_gate_in_1;
    // iit_gate_in_1, 0 ,oat_stairs_2;
    // oat_stairs_2, 0, mt_3_2;
    // mt_3_2, 0, oat_stairs_c;
    // oat_stairs_c, 1, hall_13_2;
    // hall_13_2, 1, southern_labs_c;
    // southern_labs_c, 0, lecture_hall_gt; 
    // lecture_hall_gt_t, 0, mt_2_3;
    // mt_2_3, 0, hall_3;
    // hall_3, 0, southern_labs_1;
    // southern_labs_1, 0, hall_13_2;
    // hall_13_2, 0, lecture_hall_gt;
    // lecture_hall_gt_f, 0, mt_1_3;
    // mt_1_3, 0, iit_gate_out_1;
    // iit_gate_out_1, 0, finish;";

    // let inp = "5";

    // let out = interpret(code, inp);

    HttpServer::new(|| App::new().service(interpret_handler))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
