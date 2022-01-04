use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::sample;
use crate::sample::model::Hotel;
use crate::sample::model::NewHotel;

#[get("/")]
pub fn all_hotels(connection: DbConn) -> Result<Json<Vec<Hotel>>, Status> {
  sample::repository::show_hotels(&connection)
    .map(|hotel| Json(hotel))
    .map_err(|error| error_status(error))
}

#[post("/", format="application/json", data="<new_hotel>")]
pub fn create_hotel(new_hotel: Json<NewHotel>, connection: DbConn) -> Result<status::Created<Json<Hotel>>, Status> {
  sample::repository::create_hotel(new_hotel.into_inner(), &connection)
    .map(|hotel| hotel_created(hotel))
    .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn get_hotel(id: i32, connection: DbConn) -> Result<Json<Hotel>, Status> {
  sample::repository::get_hotel(id, &connection)
    .map(|hotel| Json(hotel))
    .map_err(|error| error_status(error))
}

#[put("/<id>", format="application/json", data="<hotel>")]
pub fn update_hotel(id: i32, hotel: Json<Hotel>, connection: DbConn) -> Result<Json<Hotel>, Status> {
  sample::repository::update_hotel(id, hotel.into_inner(), &connection)
    .map(|hotel| Json(hotel))
    .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete_hotel(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
  sample::repository::delete_hotel(id, &connection)
    .map(|_| status::NoContent)
    .map_err(|error| error_status(error))
}

#[get("/rating/<rating>")]
pub fn filter_rating(rating: String, connection: DbConn) -> Result<Json<Vec<Hotel>>, Status> {
  sample::repository::filter_rating(rating, &connection)
    .map(|hotel| Json(hotel))
    .map_err(|error| error_status(error))
}

fn hotel_created(hotel: Hotel) -> status::Created<Json<Hotel>> {
  status::Created(
    format!("localhost:{port}/hotel/{id}", port = port(), id = hotel.id).to_string(),
    Some(Json(hotel)),
  )
}

fn host() -> String {
  env::var("ROCKET_ADDRESS").unwrap_or_else(|_| "localhost".to_string())
}

fn port() -> String {
  env::var("ROCKET_PORT").expect("port not given")
}

fn error_status(error: Error) -> Status {
  match error {
    Error::NotFound => Status::NotFound,
    _ => Status::InternalServerError,
  }
}