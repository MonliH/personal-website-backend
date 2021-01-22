use std::borrow::Cow;
use std::env;

use actix_web::HttpResponse;

const KEY_ERROR: &str = "Could not find ADMIN_KEY variable, did you forget to set it?";
pub fn validate_key(username: &Cow<'static, str>, pwd: &Cow<'static, str>) -> bool {
    username == &env::var("ADMIN_USERNAME").expect(KEY_ERROR)
        && pwd == &env::var("ADMIN_KEY").expect(KEY_ERROR)
}

pub fn bad_key() -> HttpResponse {
    HttpResponse::Forbidden().body("incorrect key")
}
