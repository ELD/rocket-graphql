-- Your SQL goes here
CREATE TYPE episode AS ENUM ('NewHope', 'Empire', 'Jedi');
CREATE TABLE IF NOT EXISTS humans (
  id SERIAL NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  appears_in episode[] NOT NULL,
  home_planet VARCHAR NOT NULL
);
