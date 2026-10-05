use actix_web::{HttpResponse, Responder, web};
use tokio::sync::mpsc;

use crate::structs::write_request::WriteRequest;

pub async fn write_route(req: web::Json<WriteRequest>, tx: web::Data<mpsc::Sender<WriteRequest>>) -> impl Responder {
    if tx.send(req.0).await.is_err() {
        return HttpResponse::InternalServerError().body("Write queue is down, please restart the LogKernel");
    }

    HttpResponse::Ok().body("Request has been added to queue")
}
