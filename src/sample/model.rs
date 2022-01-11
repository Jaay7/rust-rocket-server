#![allow(proc_macro_derive_resolution_fallback)]

use crate::schema::hotel;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "hotel"]
pub struct Hotel {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub category: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub phone: String,
    pub website: String,
    pub email: String,
    pub price: i32,
    pub rating: String,
    pub review_count: String,
    pub latitude: String,
    pub longitude: String,
    pub rooms: i32,
    pub image: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "hotel"]
pub struct NewHotel {
    pub name: String,
    pub description: String,
    pub category: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub phone: String,
    pub website: String,
    pub email: String,
    pub price: i32,
    pub rating: String,
    pub review_count: String,
    pub latitude: String,
    pub longitude: String,
    pub rooms: i32,
    pub image: String,
}