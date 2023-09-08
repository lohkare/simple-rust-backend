use actix_web::{get, HttpResponse, Responder, web};
use crate::go_rest_client;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[get("/user/all")]
async fn get_users() -> impl Responder {
    match go_rest_client::get_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().body("Error occurred :/")
    }
}

#[get("/user/{userId}")]
async fn get_user(path: web::Path<u32>) -> impl Responder {
    match go_rest_client::get_user(path.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().body("Error occurred :(")
    }
}