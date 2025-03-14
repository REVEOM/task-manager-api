use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use std::time::{SystemTime, UNIX_EPOCH};

// Kullanıcı giriş verisi
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

// JWT Claim yapısı
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String, // Kullanıcı adı
    exp: usize,  // Token süresi (UNIX timestamp formatında)
}

// Secret key (JWT için)
const SECRET: &[u8] = b"your_secret_key_here";

// JWT üretimi
fn create_jwt(username: &str) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() + 3600; // 1 saatlik geçerlilik süresi

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET)).unwrap()
}

// Login endpoint'i
async fn login(data: web::Json<LoginRequest>) -> impl Responder {
    // Basit bir kontrol (gerçek senaryoda kullanıcı veritabanında kontrol edilir)
    if data.username == "admin" && data.password == "password" {
        let token = create_jwt(&data.username);
        HttpResponse::Ok().json(serde_json::json!({ "token": token }))
    } else {
        HttpResponse::Unauthorized().body("Invalid username or password")
    }
}
