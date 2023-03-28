use actix_web::{web, HttpResponse, Responder, get};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

pub async fn login_handler(info: web::Json<LoginInfo>, client: web::Data<reqwest::Client>) -> impl Responder {
    let users = match client.get("http://localhost:12345/users").send().await {
        Ok(response) => match response.json::<Vec<User>>().await {
            Ok(users) => users,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        },
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match authenticate_user(&info.email, &info.password, &users) {
        Ok(role) => {
            if role == "admin" {
                HttpResponse::Found()
                    .finish()
            } else {
                HttpResponse::Ok()
                    .json("Login successful!")
            }
        }
        Err(error) => HttpResponse::Unauthorized().json(error.to_string()),
    }
}

pub fn authenticate_user(email: &str, password: &str, users: &[User]) -> Result<String, &'static str> {
    let user = match users.iter().find(|u| u.email == email) {
        Some(user) => user,
        None => return Err("Invalid email or password."),
    };

    if user.password == password {
        Ok(user.role.clone())
    } else {
        Err("Invalid email or password.")
    }
}

pub async fn get_user_role(email: web::Path<String>, client: web::Data<reqwest::Client>) -> impl Responder {
    let url = format!("http://localhost:12345/users/{}", email);
    let user = match client.get(&url).send().await {
        Ok(response) => match response.json::<User>().await {
            Ok(user) => user,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        },
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    HttpResponse::Ok().json(user.role)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users/{email}")
        .route(web::get().to(get_user_role)));
}


