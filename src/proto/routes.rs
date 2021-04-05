// use std::borrow::BorrowMut;

use super::requests::*;
use super::Db;
use rocket::State;
// pub use super::ServerState;
use rocket_contrib::json::Json;

// PUT is idempotent, repeated calls return same value
// whereas POST is not idempotent

#[openapi]
#[post("/event_manager/status", format = "json", data = "<req>")]
pub fn status(
    req: Json<EventManagerStatusRequest>,
    db: State<Db>,
) -> Json<EventManagerStatusResponse> {
    let mut db = db.lock().unwrap();
    println!("{}", req.query);
    db.call_count += 1;
    let mut resp = EventManagerStatusResponse::default();
    resp.request_status = true;
    Json(resp)
}
