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


