#![feature(plugin, const_fn, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

// #![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::routes::*;

mod routes;

fn main() {
    rocket::ignite().mount("/", routes![hello2]).launch();
}
