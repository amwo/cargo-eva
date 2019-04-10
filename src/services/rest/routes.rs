#[get("/user")]
pub fn index() -> &'static str {
    "Hello, world!"
}
