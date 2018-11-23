#![feature(decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback)]

extern crate rocket;
#[macro_use] extern crate juniper;
extern crate juniper_rocket;
#[macro_use] extern crate diesel;

pub mod schema;
pub mod models;
