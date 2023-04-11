[![check](https://github.com/playonbsd-rs/pobsd-lib/actions/workflows/check.yml/badge.svg)](https://github.com/playonbsd-rs/pobsd-lib/actions/workflows/check.yml)
[![test](https://github.com/playonbsd-rs/pobsd-lib/actions/workflows/test.yml/badge.svg)](https://github.com/playonbsd-rs/pobsd-lib/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/playonbsd-rs/pobsd-lib/branch/main/graph/badge.svg?token=zIWifzUoN9)](https://codecov.io/gh/playonbsd-rs/pobsd-lib)
[![Crates.io (latest)](https://img.shields.io/crates/v/pobsd-db?style=flat)](https://crates.io/crates/libpobsd)
[![Docs.rs](https://img.shields.io/docsrs/pobsd-db)](https://docs.rs/libpobsd)

## libpobsd

The [PlayOnBSD](https://github.com/playonbsd/OpenBSD-Games-Database)
database is a human readable database listing commercial games that
can be played on [OpenBSD](https://openbsd.org).

The **libpobsd** provides a `Parser` to parse the PlayOnBSD database and a `GameDataBase` to
query the PlayOnBSD database.

### Examples
Loading the games (represented by the `Game` struct) from the database:
```rust
use libpobsd::{Parser, ParserResult};
let games = match Parser::default()
           .load_from_file("openbsd-games.db")
           .expect("Failed to load database") {
    ParserResult::WithoutError(games) => games,
    ParserResult::WithError(games, _) => games,
};
```
Loading the games into the `GameDataBase`:
```rust
use libpobsd::{Parser, ParserResult, GameDataBase};
let games = match Parser::default()
           .load_from_file("openbsd-games.db")
           .expect("Failed to load database") {
    ParserResult::WithoutError(games) => games,
    ParserResult::WithError(games, _) => games,
};
let db = GameDataBase::new(games);
```
Search games by name:
```rust
use libpobsd::{Parser, ParserResult, GameDataBase};
let games = match Parser::default()
           .load_from_file("openbsd-games.db")
           .expect("Failed to load database") {
    ParserResult::WithoutError(games) => games,
    ParserResult::WithError(games, _) => games,
};
let db = GameDataBase::new(games);
let games = db.search_game_by_name("Barrow");
```
Filter a query result (represented by the `QueryResult` struct)
by year:
```rust
use libpobsd::{Parser, ParserResult, GameDataBase};
let games = match Parser::default()
           .load_from_file("openbsd-games.db")
           .expect("Failed to load database") {
    ParserResult::WithoutError(games) => games,
    ParserResult::WithError(games, _) => games,
};
let db = GameDataBase::new(games);
let games = db.search_game_by_name("Barrow");
let games = games.get_game_by_year("2018");
```
List the games of a query result:
```rust
use libpobsd::{Parser, ParserResult, GameDataBase};
let games = match Parser::default()
           .load_from_file("openbsd-games.db")
           .expect("Failed to load database") {
    ParserResult::WithoutError(games) => games,
    ParserResult::WithError(games, _) => games,
};
let db = GameDataBase::new(games);
let games = db.search_game_by_name("Barrow");
for game in games.into_inner() {
    println!("Game: {}", game.name);
}
```
More examples are available in the documentation of each module.
