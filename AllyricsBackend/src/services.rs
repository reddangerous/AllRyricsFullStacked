use actix_web::{web::{Json, Data, Path, self},  HttpResponse, Responder
,post, get, put, delete};

use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
 use crate::AppState;

#[derive( FromRow,Serialize, Deserialize)]
pub struct User{
     name: String,
     email: String,
     role:String,
     password: String,
    
}

#[derive(Deserialize, Serialize)]
pub struct CreateUser{
    name: String,
    email: String,
    role: String,
    password: String,
    
}

#[derive(Deserialize)]
pub struct UpdateUser{
    name: String,
    email: String,
    role: String,
    password: String,
   
}

/*#[derive(Deserialize)]
pub struct DeleteUser{
    id: i32,
}*/
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}


#[post("/users")]
pub async fn create_user(state: Data<AppState>, body: Json<CreateUser>) -> impl Responder {
    // Validate email address
    if !is_valid_email(&body.email) {
        return HttpResponse::BadRequest().json("Invalid Email Address");
    }
    
    // Validate password complexity
    if !is_valid_password(&body.password) {
        return HttpResponse::BadRequest().json("Invalid Password: Password should be at least 8 characters and a combination of numbers and special characters small and capital letters");
    }
    
    // Insert user into database
    match sqlx::query("INSERT INTO users (name, email, role, password) VALUES ($1, $2, $3, $4) RETURNING   name, email, role, password")
        .bind(&body.name.to_string())
        .bind(&body.email.to_string())
        .bind(&body.role.to_string())
        .bind(&body.password.to_string())
        .fetch_one(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("User Added Successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Internal Server Error"),
    }
}

// Function to validate email address
fn is_valid_email(email: &str) -> bool {
    // Simple email validation using regex
    let re = regex::Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$").unwrap();
    re.is_match(email)
}

// Function to validate password
fn is_valid_password(password: &str) -> bool {
    // Password should be at least 8 characters long
    if password.len() < 8 {
        return false;
    }

    // Password should contain at least one lowercase letter
    let re = regex::Regex::new(r"[a-z]").unwrap();
    if !re.is_match(password) {
        return false;
    }

    // Password should contain at least one uppercase letter
    let re = regex::Regex::new(r"[A-Z]").unwrap();
    if !re.is_match(password) {
        return false;
    }

    // Password should contain at least one digit
    let re = regex::Regex::new(r"\d").unwrap();
    if !re.is_match(password) {
        return false;
    }

    // Password should contain at least one special character
    let re = regex::Regex::new(r"[@$!%*?&]").unwrap();
    if !re.is_match(password) {
        return false;
    }

    true
}


#[get("/users")]
pub async fn get_users(state:Data<AppState>,) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&state.db)
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
    println!("Error fetching users: {}", e);
    HttpResponse::NotFound().json("users not found")
}
    }
}

#[post("/users/{email}")]
pub async fn get_user(state:Data<AppState>, email:Path<String>) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE email=$1")
    .bind(email.into_inner())
    .fetch_one(&state.db)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().json("User not found"),
    }
}

#[put("/users/{email}")]
pub async fn update_user(state:Data<AppState>, email:Path<String>, body:Json<UpdateUser>) -> impl Responder {
    match sqlx::query("UPDATE users SET name=$1, email=$2, role=$3, password=$4 WHERE email=$5")
    .bind(&body.name.to_string())
    .bind(&body.email.to_string())
    .bind(&body.role.to_string())
    .bind(&body.password.to_string())
    .bind(email.into_inner())
    .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("User Updated Successfully"),
        Err(_) => HttpResponse::NotFound().json("Internal Server Error"),
    }
}

#[delete("/users/{email}")]
pub async fn delete_user(state:Data<AppState>, email:Path<String>) -> impl Responder {
    match sqlx::query("DELETE FROM users WHERE email=$1")
    .bind(email.into_inner())
    .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("User Deleted Successfully"),
        Err(_) => HttpResponse::NotFound().json("Internal Server Error"),
    }
}
