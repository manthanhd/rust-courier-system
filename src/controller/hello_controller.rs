#[get("/hello/<name>/<age>")]
pub fn get_hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}