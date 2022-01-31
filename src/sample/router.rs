use rocket;

use crate::connection;
use crate::sample;

pub fn create_routes() {
  rocket::ignite()
    .manage(connection::init_pool())
    .mount("/hotel", routes![
      sample::handler::all_hotels,
      sample::handler::create_hotel,
      sample::handler::get_hotel,
      sample::handler::update_hotel,
      sample::handler::delete_hotel,
      sample::handler::filter_rating,
      sample::handler::price_range,
      sample::handler::filter_rooms
    ])
    .launch();
}