-- Add migration script here

COPY words (id, word)
FROM '/Users/dionemorales/Development/dwordle-backend/wordle.csv' delimiter ',';

