-- Your SQL goes here
CREATE TABLE hotel (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  category VARCHAR NOT NULL,
  address VARCHAR NOT NULL,
  city VARCHAR NOT NULL,
  state VARCHAR NOT NULL,
  zip VARCHAR NOT NULL,
  phone VARCHAR NOT NULL,
  website VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  price VARCHAR NOT NULL,
  rating VARCHAR NOT NULL,
  review_count VARCHAR NOT NULL,
  latitude VARCHAR NOT NULL,
  longitude VARCHAR NOT NULL,
  rooms INT NOT NULL,
  image VARCHAR NOT NULL
);