# dwordle backend

Backend for the [dwordle](https://github.com/DioneJM/dwordle) project

TODO:
- Uniquely identify users
  - as frictionless as possible to create?
  - does it have to persist between devices? or per device is alright?
- manage leaderboard
  - manage community leaderboards?

## Development
To get started locally:
1. Initialise the DB
2. `cargo run`


### Initialising the DB
1. Update the [migration script](migrations/20220406201351_insert_words.sql) to insert the word list with the absolute path of the file
   - The directory path can be found by running command `pwd` in the project root and appending `'wordle.csv'`
2. Start docker daemon
3. In project root run: `./scripts/init_db.sh`