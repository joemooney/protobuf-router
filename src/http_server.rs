// use crate::db;
use crate::db::ServerState;
use crate::proto;
use crate::Opt;
use rocket::Rocket;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use std::env;
use std::sync::Mutex;

pub fn build_app(opt: Opt) -> Rocket {
    env::set_var("ROCKET_PORT", opt.port.to_string());
    rocket::ignite()
        .manage(Mutex::new(ServerState::new()))
        .mount("/", routes_with_openapi![proto::status])
        .mount(
            "/docs/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}
