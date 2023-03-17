
use actix_web::{get, post, web::{self, Data}, HttpResponse, Responder};
use serde_json::json;


use std::{path::PathBuf,string::String};
use reqwest::multipart::{Form, Part};
use tokio::fs::{File, self};


use crate::AppState;
use serde::{Deserialize};

#[post("/upload")]
pub async fn upload_file(file: web::Form<UploadFile>) -> impl Responder {
    let client = reqwest::Client::new();
    let file_path = format!("{}", file.filename).to_string();
    let file = File::open(&file_path).await.unwrap();
    let bytes = fs::read(&file_path).await.unwrap();
    let part = Part::bytes(bytes).file_name(file_path.clone());
    let form = Form::new()
        .part("file", part)
        .text("upload_preset", "date time");
    let response = client
        .post("https://431226661557672:oun6bQyaL-UjaHftgnx6L_NK3ZY@api.cloudinary.com/v1_1/ddhpc9gby/resources/video/")
        .header("Content-Type", "application/binary")
        .multipart(form)
        .send()
        .await
        .unwrap();

    let data = response.json::<serde_json::Value>().await.unwrap();
    let url = data["url"].as_str().unwrap();

    HttpResponse::Ok().json(json!({ "url": url }))

}


#[get("/musicdata")]
pub async fn get(_state:Data<AppState>,) -> impl Responder {
    let client = reqwest::Client::new();
    let response = client
        .get("https://431226661557672:oun6bQyaL-UjaHftgnx6L_NK3ZY@api.cloudinary.com/v1_1/ddhpc9gby/resources/video")
        .header("Cloud-Name", "ddhpc9gby")
        .header("Api-Secret", "oun6bQyaL-UjaHftgnx6L_NK3ZY")
        .header("Api-Key", "431226661557672")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .send()
        .await
        .unwrap();

    let data = response.bytes().await.unwrap();

    HttpResponse::Ok().body(data)
}


#[derive(Deserialize)]
pub struct UploadFile {
    filename: String,
    file: PathBuf,
}
