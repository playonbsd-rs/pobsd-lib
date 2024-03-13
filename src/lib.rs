#![warn(missing_docs)]
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
//! 14. *Status*: numerical status with date when tested on -current in parentheses (doesn't
//!  apply to upstream bugs that have nothing to do with the OpenBSD platform); note highest
//!  numerical description reached applies
//! * 0 = doesn't run
//! * 1 = game launches (not enough information to comment meaningfully on status beyond launching the game)
//! * 2 = major bugs: potentially game-breaking, making finishing the game impossible or a chore;
//! noticeably degrading the enjoyment compared to running the game on other platforms
//! * 3 = medium-impact bugs: noticeable, but not game-breaking
//! * 4 = minor bugs: barely noticeable, or not relevant to core game
//! * 5 = completable: game can be played through until the credits roll, without major bugs (category 2);
//! doesn't (necessarily) include optional side content, DLC, optional multiplayer, achievements etc.
//! * 6 = 100%: the complete game including optional content like DLC, side quests, multiplayer can be enjoyed
//! 15. *Added*: date (ISO 8601 format) when the entry was added (EPOCH when the information is not available)
//! 16. *Updated*: date (ISO 8601 format) when the entry was last updated
//! 17. *IgdbId*: id of the game in the [IGDB](https://www.igdb.com) database
//!
//! The **libpobsd** provide a [`Parser`] to parse the PlayOnBSD database and a [`GameDataBase`] to
//! query the PlayOnBSD database. The result of a [`GameDataBase`] query are returned as a [`QueryResult`]
//! of [`Item`] or [`Game`] depending on the nature of the query. [`Game`] collections can also
//! be filtered using a [`GameFilter`].
//!
//! ## Examples
//! Loading the games listed in the PlayOnBSD database in a vector:
//! ```no_run
//! use libpobsd::{Parser, ParserResult, Game};
//!
//! let games: Vec<Game> = match Parser::default()
//!            .load_from_file("openbsd-games.db")
//!            .expect("Failed to load database") {
//!     ParserResult::WithoutError(games) => games,
//!     ParserResult::WithError(games, _) => games,
//! };
//! ```
//!
//! Loading the games listed in the PlayOnBSD database
//! into the [`GameDataBase`] without dealing with parsing
//! errors if any:
//! ```no_run
//! use libpobsd::{Parser, ParserResult, GameDataBase, Game};
//! let games: Vec<Game> = match Parser::default()
//!            .load_from_file("openbsd-games.db")
//!            .expect("Failed to load database") {
//!     ParserResult::WithoutError(games) => games,
//!     ParserResult::WithError(games, _) => games,
//! };
//! let db = GameDataBase::new(games);
//! ```
//!
//! Perform a non case sensitive search of games by name using
//! the [`GameDataBase`], the query result being return in a form
//! of a [`QueryResult`]:
//! ```no_run
//! # use libpobsd::{Parser, ParserResult, GameDataBase, SearchType, QueryResult, Game};
//! # let games = match Parser::default()
//! #            .load_from_file("openbsd-games.db")
//! #            .expect("Failed to load database") {
//! #     ParserResult::WithoutError(games) => games,
//! #     ParserResult::WithError(games, _) => games,
//! # };
//! let db = GameDataBase::new(games);
//! let st = SearchType::CaseSensitive;
//! let games: QueryResult<&Game> = db.search_games_by_name("Barrow", &st);
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
//! let games = db.search_games_by_name("Barrow", &st);
//! let games = games.filter_games_by_year("2018", &st);
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
//! let games = db.search_games_by_name("Barrow", &st);
//! for game in games.into_inner() {
//!     println!("Game: {}", game.name);
//! }
//! ```
//! More examples are available in each module documentation.

pub mod db;
#[allow(clippy::tabs_in_doc_comments)]
pub mod models;
pub mod parsing;

pub use crate::db::game_filer::GameFilter;
pub use crate::db::GameDataBase;
pub use crate::db::Item;
pub use crate::db::QueryResult;
pub use crate::db::SearchType;
pub use crate::models::Game;
pub use crate::models::GameStatus;
pub use crate::models::Status;
pub use crate::models::Store;
pub use crate::models::StoreLink;
pub use crate::models::StoreLinks;
pub use crate::parsing::Parser;
pub use crate::parsing::ParserResult;
pub use crate::parsing::ParsingMode;
