DROP 
  TABLE IF EXISTS deck CASCADE;
DROP 
  TABLE IF EXISTS card CASCADE;

CREATE TABLE deck (
  id SERIAL PRIMARY KEY, 
  deckname VARCHAR(150) NOT NULL
);

CREATE TABLE card (
  id SERIAL PRIMARY KEY, 
  front VARCHAR(150) NOT NULL, 
  back VARCHAR(150) NOT NULL, 
  deck_id INTEGER NOT NULL, 
  FOREIGN KEY (deck_id) REFERENCES deck(id)
);

INSERT INTO deck (deckname) 
VALUES 
  ('javascript'), 
  ('typescript');

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