#![feature(decl_macro)]

use log::info;
use rocket::{get, routes};
use std::{env, thread};

#[get("/quit")]
fn quit() {
    std::process::exit(0);
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![quit])
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("ROCKET_ENV", "dev");

    let rocket = thread::Builder::new()
        .name("rocket".into())
        .spawn(move || {
            let err = rocket().launch();
            panic!("Launch rocket failed: {}", err);
        })
        .expect("Failed to launch Rocket");

    let _ = rocket.join();
    info!("pRuntime quited");
}
