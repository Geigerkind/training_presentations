#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate mysql;

mod domain;
mod tools;
mod transfer;

fn main() {
    let hero_manager = domain::HeroManager::default().init();
    rocket::ignite()
        .manage(hero_manager)
        .mount("/API/",
               routes![
                    transfer::get_heroes::get_heroes
               ])
        .launch();
}
