use rocket::http::RawStr;

#[get("/hello")]
pub fn hello() -> &'static str {
    let nu = 33;
    "Hello, world!"
}

#[get("/hello/<name>")]
pub fn hello2(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}
