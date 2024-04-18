use rocket::request::{FromRequest, Request, Outcome};
use rocket::http::Status;

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::env;


//start of JWT implementation
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    subject: i32, //User id
    iat: usize, //issued at
    exp: usize, // time expires,
    role: i32,
}
#[derive(Debug, Deserialize)]
pub struct AuthenticatedUser {
    pub user_id: i32, 
    pub role: i32,     
}
//TODO test with new endpoint, define results via id and role.


#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("Authorization") {
            Some(header) if header.starts_with("Bearer ") => {
                let token = &header[7..]; // Skip "Bearer "
                match validate_jwt(token) {
                    Ok(claims) => {
                        Outcome::Success(AuthenticatedUser {
                            user_id: claims.subject,
                            role: claims.role,
                        })
                    },
                    Err(_) => {
                        // Token is invalid
                        Outcome::Error((Status::Unauthorized, ()))
                        //TODO add a log here
                    }
                }
            },
            _ => Outcome::Error((Status::Unauthorized, ()))
        }
    }
}


fn create_jwt(id: &i32, role: i32) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = (chrono::Utc::now() + chrono::Duration::minutes(20)).timestamp() as usize;
    let claims = Claims { subject: id.to_owned(), iat: chrono::Utc::now().timestamp() as usize, role: role.to_owned(), exp: expiration };
    let secret_key = env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");
    encode(&Header::new(jsonwebtoken::Algorithm::HS256), &claims, &EncodingKey::from_secret(secret_key.as_bytes()))
}

fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    decode::<Claims>(token, &DecodingKey::from_secret(secret_key.as_bytes()), &Validation::default())
        .map(|data| data.claims)
}

//end of JWT implementation 
pub struct BasicAuth {
    pub username: String,
    pub password: String,

}
impl BasicAuth {
    fn from_authorization_header(header: &str) -> Option<BasicAuth> {
        let split = header.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2 {
            return None;
        }
        if split[0] != "Basic" {
            return None;
        }

        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base64_string: &str) -> Option<BasicAuth> {
        let decoded = base64::decode(base64_string).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split = decoded_str.split(":").collect::<Vec<_>>();

        if split.len() != 2 {
            return None;
        }

        let (username, password) = (split[0].to_string(), split[1].to_string());

        Some(BasicAuth {
            username,
            password
        })
    }
}

//guard for auth
//curl 127.0.0.1:8000/users -H 'Authorization: Basic Zm9vOmJhcg=='

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_authorization_header(auth_header) {
                //TODO change this auth
                if auth.username == String::from("foo") && auth.password == String::from("bar") {

                    return Outcome::Success(auth)
            }
        }
    }

        Outcome::Error((Status::Unauthorized, ()))

    }
}

