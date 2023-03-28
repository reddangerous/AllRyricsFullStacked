use actix_web::{web::{Json, Data, Path},  HttpResponse, Responder
,post, get, put, delete};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
 use crate::AppState;


#[derive(FromRow, Serialize)]
pub struct Music {
    id: i32,
    music_title: String,
    artist: String,
    lyrics: String,
}

#[derive(Deserialize)]
pub struct NewMusic {
    music_title: String,
    artist: String,
    lyrics: String,
}

#[derive(Deserialize)]
pub struct UpdateMusic {
    music_title: String,
    artist: String,
    lyrics: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewMusicData {
    file_data: Vec<u8>,
}
#[derive(serde::Deserialize)]
struct MediaUpload {
    media: String,
}
#[get("/music")]

pub async fn get_music(state: Data<AppState>,) -> impl Responder {
    match sqlx::query_as::<_, Music>("SELECT * FROM Allyrics")
        .fetch_all(&state.db)
        .await
    {
        Ok(music) => HttpResponse::Ok().json(music),
        Err(_) => HttpResponse::NotFound().json("music not found"),
    }
}
#[get("/music/{id}")]
pub async fn get_music_by_id(state: Data<AppState>, id: Path<i32>) -> impl Responder {
    match sqlx::query_as::<_, Music>("SELECT * FROM Allyrics WHERE id = $1")
        .bind(id.into_inner())
        .fetch_one(&state.db)
        .await
    {
        Ok(music) => HttpResponse::Ok().json(music),
        Err(_) => HttpResponse::NotFound().json("music not found"),
    }
}
#[post("/music")]
pub async fn add_music(
   state: Data<AppState>,
   new_music: Json<NewMusic>,
) -> impl Responder {
   match sqlx::query("INSERT INTO Allyrics (Music_title, Artist, Lyrics) VALUES ($1,$2,$3)")
       .bind(&new_music.music_title.to_string())
       .bind(&new_music.artist.to_string())
       .bind(&new_music.lyrics.to_string())
       .execute(&state.db)
       .await
   {
       Ok(_) => HttpResponse::Ok().json("Music Added Successfully"),
       Err(_) => HttpResponse::NotFound().json("Internal Server Error"),
   }
}
#[put("/music/{id}")]
pub async fn update_music(
   state: Data<AppState>,
   new_music: Json<UpdateMusic>,
   id: Path<i32>,
) -> impl Responder {
   match sqlx::query("UPDATE Allyrics SET Music_title=$1, Artist=$2, Lyrics=$3 WHERE id=$4")
       .bind(&new_music.music_title.to_string())
       .bind(&new_music.artist.to_string())
       .bind(&new_music.lyrics.to_string())
       .bind(id.into_inner())
       .execute(&state.db)
       .await
   {
       Ok(_) => HttpResponse::Ok().json("Music Updated Successfully"),
       Err(_) => HttpResponse::NotFound().json("Internal Server Error"),
   }
}
#[delete("/music/{id}")]
pub async fn delete_music(state: Data<AppState>, id: Path<i32>) -> impl Responder {
   match sqlx::query("DELETE FROM Allyrics WHERE id=$1")
       .bind(id.into_inner())
       .execute(&state.db)
       .await
   {
       Ok(_) => HttpResponse::Ok().json("Music Deleted Successfully"),
       Err(_) => HttpResponse::NotFound().json("Internal Server Error"),
   }
}
