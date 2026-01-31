// use actix_web::{
//     dev::Payload, error::ErrorUnauthorized, web::Data, Error, FromRequest, HttpRequest,
// };
// use serde::{Deserialize, Serialize};
// use std::future::{ready, Ready};

// use crate::infrastructure::{config::Config, security::verify_jwt};

// #[derive(Debug, Clone, Serialize)]
// pub struct UserExtractor {
//     pub id: u32,
//     pub email: String,
// }

// impl FromRequest for UserExtractor {
//     type Error = Error;
//     type Future = Ready<Result<Self, Self::Error>>;

//     fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
//         // Извлекаем токен из заголовка Authorization
//         let auth_header = req
//             .headers()
//             .get("Authorization")
//             .and_then(|h| h.to_str().ok());

//         let token = match auth_header {
//             Some(header) if header.starts_with("Bearer ") => &header[7..],
//             _ => {
//                 return ready(Err(ErrorUnauthorized(
//                     "Missing or invalid Authorization header",
//                 )))
//             }
//         };

//         // Получаем конфигурацию с секретом
//         let config = match req.app_data::<Data<Config>>() {
//             Some(cfg) => cfg,
//             None => return ready(Err(ErrorUnauthorized("Configuration not found"))),
//         };

//         // Валидируем токен (пример валидации)
//         match verify_jwt(token, &config.jwt_secret) {
//             Ok(claims) => match claims.user_id.parse::<u32>() {
//                 Ok(user_id) => ready(Ok(UserExtractor {
//                     id: user_id,
//                     email: claims.email,
//                 })),
//                 Err(_) => ready(Err(ErrorUnauthorized("Invalid user ID in token"))),
//             },
//             Err(_) => ready(Err(ErrorUnauthorized("Invalid or expired token"))),
//         }
//     }
// }
