use rocket::request::{self, FromRequest};
use rocket::{Request, Outcome};
use jsonwebtoken::{decode, Validation};

#[derive(Serialize, Deserialize)]
pub struct UserToken {
  pub id: Option<i32>,
  pub username: Option<String>,
}

fn get_token_data(token: &str) -> UserToken {
  decode::<UserToken>(&token, "secret".as_ref(), &Validation {validate_exp: false, ..Default::default()}).unwrap().claims
}

impl<'a, 'r> FromRequest<'a, 'r> for UserToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let mut auth_token: Option<String> = None;
        let headers = request.headers();
        let authorization: Vec<_> = headers.get("Authorization").collect();

        if authorization.len() == 1 {
          auth_token = Some(authorization[0].to_owned());
        }

        let token = if auth_token.is_some() {
          get_token_data(&auth_token.unwrap())
        } else {
          UserToken { id: None, username: None }
        };

        Outcome::Success(token)
    }
}
