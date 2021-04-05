#![feature(plugin, const_fn, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

// #![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_derive;
extern crate serde_json;

use crate::routes::*;
mod db_service;
mod routes;

fn main() {
    rocket::ignite()
        .mount("/", routes![get_entity, create_entity])
        .launch();
}
