use actix_web::{web::{Json, Data, Path},  HttpResponse, Responder
,post, get, put, delete};

use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
 use crate::AppState;

#[derive( FromRow,Serialize, Deserialize)]
pub struct User{
     id: i32,
     name: String,
     email: String,
     password: String,
    
}

#[derive(Deserialize, Serialize)]
pub struct CreateUser{
    name: String,
    email: String,
    password: String,
    
}

#[derive(Deserialize)]
pub struct UpdateUser{
    name: String,
    email: String,
    password: String,
   
}

/*#[derive(Deserialize)]
pub struct DeleteUser{
    id: i32,
}*/


#[post("/users")]
pub async fn create_user(state:Data<AppState>, body:Json<CreateUser>) -> impl Responder {
    match sqlx::query("INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING  id, name, email, password")
.bind(&body.name.to_string())
    .bind(&body.email.to_string())
    .bind(&body.password.to_string())   
    .fetch_one(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("User Added Successfully"),
        Err(_)=> HttpResponse::NotFound().json("Internal Server Error"),
    }
}

#[get("/users")]
pub async fn get_users(state:Data<AppState>,) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&state.db)
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().json("users not found"),
    }
}
//getting users by email address
#[get("/users/{email}")]
pub async fn get_user(state:Data<AppState>, email:Path<String>) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE email=$1")
    .bind(email.into_inner())
        .fetch_one(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().json("user not found"),
    }
}
#[put("/users/{id}")]
pub async fn update_user(state:Data<AppState>, body:Json<UpdateUser>, id:Path<i32>) -> impl Responder {
    match sqlx::query("UPDATE users SET name=$1, email=$2, password=$3, WHERE id=$4")
    .bind(&body.name.to_string())
    .bind(&body.email.to_string())
    .bind(&body.password.to_string())
    .bind(id.into_inner())
    .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("User Updated Successfully"),
        Err(_) => HttpResponse::NotFound().json("Internal Server Error"),
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(state:Data<AppState>, id:Path<i32>) -> impl Responder {
    match sqlx::query("DELETE FROM users WHERE id=$1")
    .bind(id.into_inner())
    .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("User Deleted Successfully"),
        Err(_) => HttpResponse::NotFound().json("Internal Server Error"),
    }
}
