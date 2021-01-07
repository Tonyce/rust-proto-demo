#[macro_use]
extern crate prost_derive;

pub mod route;
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/snazzy.items.rs"));
}