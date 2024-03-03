//! The [PlayOnBSD](https://github.com/playonbsd/OpenBSD-Games-Database)
//! database is a human readable database listing commercial games that
//! can be played on [OpenBSD](https://openbsd.org).
//! Currently, each game is represented by **17** lines (one for each field), **in the following order**:
//! 1. *Game*: string, leading "A " or "The " treated specially for alphabetic ordering
//! 2. *Cover*: path to cover art image file (`.png`, `.jpg`)
//! 3. *Engine*: string of valid engine entry
//! 4. *Setup*: string (package, command, text)
//! 5. *Runtime*: string; should correspond to an executable in packages
//! 6. *Store*: strings of URLs, whitespace-separated
//! 7. *Hints*: string
//! 8. *Genre*: strings, comma-separated
//! 9. *Tags*: strings, comma-separated
//! 10. *Year*: integer (release year)
//! 11. *Dev*: string (developer), comma-separated
//! 12. *Pub*: string (publisher), comma-separated
//! 13. *Version*: version number/string
//! 14. *Status*: string of valid status with date when tested on -current in parentheses
//! 15. *Added*: date (ISO 8601 format) when the entry was added (EPOCH when the information is not available)
//! 16. *Updated*: date (ISO 8601 format) when the entry was last updated
//! 17. *IgdbId*: id of the game in the [IGDB](https://www.igdb.com) database
//!
//! The **libpobsd** provide a [`Parser`] to parse the PlayOnBSD database and a [`GameDataBase`] to
//! query the PlayOnBSD database.
//!
//! ## Examples
//! Loading the games (represented by the [`Game`] struct) from the database:
//! ```no_run
//! use libpobsd::{Parser, ParserResult};
//! let games = match Parser::default()
//!            .load_from_file("openbsd-games.db")
//!            .expect("Failed to load database") {
//!     ParserResult::WithoutError(games) => games,
//!     ParserResult::WithError(games, _) => games,
//! };
//! ```
//!
//! Loading the games into the [`GameDataBase`]:
//! ```no_run
//! use libpobsd::{Parser, ParserResult, GameDataBase};
//! let games = match Parser::default()
//!            .load_from_file("openbsd-games.db")
//!            .expect("Failed to load database") {
//!     ParserResult::WithoutError(games) => games,
//!     ParserResult::WithError(games, _) => games,
//! };
//! let db = GameDataBase::new(games);
//! ```
//!
//! Search games by name:
//! ```no_run
//! # use libpobsd::{Parser, ParserResult, GameDataBase, db::SearchType};
//! # let games = match Parser::default()
//! #            .load_from_file("openbsd-games.db")
//! #            .expect("Failed to load database") {
//! #     ParserResult::WithoutError(games) => games,
//! #     ParserResult::WithError(games, _) => games,
//! # };
//! let db = GameDataBase::new(games);
//! let st = SearchType::CaseSensitive;
//! let games = db.search_game_by_name("Barrow", &st);
//! ```
//!
//! Filter a query result (represented by the [`QueryResult`] struct)
//! by year:
//! ```no_run
//! # use libpobsd::{Parser, ParserResult, GameDataBase, db::SearchType};
//! # let games = match Parser::default()
//! #            .load_from_file("openbsd-games.db")
//! #            .expect("Failed to load database") {
//! #     ParserResult::WithoutError(games) => games,
//! #     ParserResult::WithError(games, _) => games,
//! # };
//! let db = GameDataBase::new(games);
//! let st = SearchType::CaseSensitive;
//! let games = db.search_game_by_name("Barrow", &st);
//! let games = games.get_game_by_year("2018", &st);
//! ```
//!
//! List the games of a query result:
//! ```no_run
//! # use libpobsd::{Parser, ParserResult, GameDataBase, db::SearchType};
//! # let games = match Parser::default()
//! #            .load_from_file("openbsd-games.db")
//! #            .expect("Failed to load database") {
//! #     ParserResult::WithoutError(games) => games,
//! #     ParserResult::WithError(games, _) => games,
//! # };
//! let db = GameDataBase::new(games);
//! let st = SearchType::CaseSensitive;
//! let games = db.search_game_by_name("Barrow", &st);
//! for game in games.into_inner() {
//!     println!("Game: {}", game.name);
//! }
//! ```
//! More examples are available in each module documentation.

pub mod db;
pub mod parser;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub use crate::db::game_filer::GameFilter;
pub use crate::db::GameDataBase;
pub use crate::db::QueryResult;
pub use crate::db::SearchType;
pub use crate::parser::Game;
pub use crate::parser::Parser;
pub use crate::parser::ParserResult;
pub use crate::parser::ParsingMode;
