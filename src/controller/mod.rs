extern crate rocket;

pub mod hello_controller;

use std::vec::Vec;
use rocket::Route;
pub fn routes() -> Vec<Route> {
    routes![hello_controller::get_hello]
}