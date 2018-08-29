#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

mod controller;

fn main() {
    println!("Hello, world!");
    rocket::ignite().mount("/", controller::routes()).launch();
}
