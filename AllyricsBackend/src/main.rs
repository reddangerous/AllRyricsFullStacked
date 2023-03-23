use actix_web::{web::{Data, self}, App, HttpServer, http};
use dotenv::dotenv;
use actix_cors::Cors;
use sqlx::{postgres::PgPoolOptions, PgPool};
mod services;
mod music;
use services::{create_user, get_users,update_user, delete_user,get_user};
use music ::{get_music, add_music};
mod upload;
use upload::{get, upload_file };
use actix_files::Files;
pub struct AppState {
    pub db: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to connect to Postgres");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8081")
            .allowed_origin("http://127.0.0.1:8081")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
       
        App::new()
            .app_data(Data::new(AppState { db:pool.clone() }))
            .wrap(cors)
           .service(web::resource("/").route(web::post()))
            .service(create_user)
            .service(get_users)
            .service(update_user)
            .service(delete_user)
            .service(get_user)
            .service(get_music)
            .service(add_music)
            .service(upload_file)
            .service(get)
            .service(Files::new("/", "./static"))
            
        
    })
    .bind(("127.0.0.1",9000))?
    .run()
    .await

}