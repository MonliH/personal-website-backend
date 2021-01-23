use std::borrow::Cow;
use std::env;
use std::future::Future;

use actix_web::HttpResponse;

const KEY_ERROR: &str = "Could not find ADMIN_KEY variable, did you forget to set it?";

fn validate_key(username: &Cow<'static, str>, pwd: &Cow<'static, str>) -> bool {
    username == &env::var("ADMIN_USERNAME").expect(KEY_ERROR)
        && pwd == &env::var("ADMIN_KEY").expect(KEY_ERROR)
}

fn bad_key() -> HttpResponse {
    HttpResponse::Forbidden().body("incorrect key")
}

pub async fn protected<T, C>(
    user: &Cow<'static, str>,
    password: Option<&Cow<'static, str>>,
    logic: C,
) -> HttpResponse
where
    T: Future<Output = HttpResponse>,
    C: Fn() -> T,
{
    match password {
        Some(pwd) if validate_key(user, pwd) => logic().await,
        _ => bad_key(),
    }
}
