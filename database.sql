DROP 
  TABLE IF EXISTS deck CASCADE;
DROP 
  TABLE IF EXISTS card CASCADE;
DROP 
  TABLE IF EXISTS users CASCADE;

CREATE TABLE users (
  id SERIAL PRIMARY KEY, 
  username VARCHAR(150) NOT NULL, 
  password VARCHAR(150) NOT NULL
);

CREATE TABLE deck (
  id SERIAL PRIMARY KEY, 
  deckname VARCHAR(150) NOT NULL, 
  created_by INTEGER NOT NULL, 
  FOREIGN KEY (created_by) REFERENCES users(id)
);

CREATE TABLE card (
  id SERIAL PRIMARY KEY, 
  front VARCHAR(150) NOT NULL, 
  back VARCHAR(150) NOT NULL, 
  deck_id INTEGER NOT NULL, 
  FOREIGN KEY (deck_id) REFERENCES deck(id)
);

INSERT INTO users (username, password) 
VALUES 
  ('testuser', 'testpassword');

INSERT INTO deck (deckname, created_by) 
VALUES 
  ('javascript', 1), 
  ('typescript', 1);

INSERT INTO card (front, back, deck_id) 
VALUES 
  (
    'javascript question', 'javascript answer', 
    1
  ), 
  (
    'typescript question', 'typescript answer', 
    2
  );