#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::sample::model::Hotel;
use crate::sample::model::NewHotel;

use crate::schema::hotel;
use crate::schema::hotel::dsl::*;

pub fn create_hotel(new_hotel: NewHotel, conn: &PgConnection) -> QueryResult<Hotel> {
    diesel::insert_into(hotel::table)
        .values(&new_hotel)
        .get_result(conn)
}

pub fn show_hotels(connection: &PgConnection) -> QueryResult<Vec<Hotel>> {
  hotel.limit(5)
    .load::<Hotel>(&*connection)
}

pub fn get_hotel(hotel_id: i32, connection: &PgConnection) -> QueryResult<Hotel> {
  hotel::table.find(hotel_id).get_result::<Hotel>(connection)
}

pub fn update_hotel(hotel_id: i32, hotel_u: Hotel, connection: &PgConnection) -> QueryResult<Hotel> {
  diesel::update(hotel::table.find(hotel_id))
    .set(&hotel_u)
    .get_result(connection)
}

pub fn delete_hotel(hotel_id: i32, connection: &PgConnection) -> QueryResult<usize> {
  diesel::delete(hotel::table.find(hotel_id))
    .execute(connection)
}

pub fn filter_rating(hotel_rating: String, connection: &PgConnection) -> QueryResult<Vec<Hotel>> {
  hotel.filter(rating.eq(hotel_rating))
    .limit(5)
    .load::<Hotel>(&*connection)
}

pub fn price_range(range1: i32, range2: i32, connection: &PgConnection) -> QueryResult<Vec<Hotel>> {
  hotel.filter(price.ge(range1))
    .filter(price.le(range2))
    .limit(5)
    .load::<Hotel>(&*connection)
}