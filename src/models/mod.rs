//! Provides a [`Parser`] as well as few related items (such as [`Game`])
//! for the PlayOnBSD Database.
//!
//! In particular, this module provides:
//! * A [`Parser`] struct handling the parsing
//! * A [`ParsingMode`] enum to choose the parsing mode betweena strict and a relax mode
//! * A [`ParserResult`] struct to handle parsing with and without error
//! * A [`Game`] struct representing a game of a database
//! * A [`StoreLinks`] struct, a [`StoreLink`] structs and [`Store`] enum to represent store links for each game
//!
//! ### Examples
//! Here is a first example loading a file in relaxed mode (by default).
//! ```no_run
//! use libpobsd::{Parser, ParserResult};
//!
//! // Create a parser
//! let parser = Parser::default();
//! // Load the database
//! let parser_result = parser.load_from_file("/path/to/games.db")
//!        .expect("Problem trying to open the file");
//! let games = match parser_result {
//!        ParserResult::WithoutError(games) => games,
//!        ParserResult::WithError(games, _) => games,
//!    };
//! ```
//! The parser can also use a strict mode in which it will stop when encountering
//! a parsing error and returning the games it has processed.
//! ```no_run
//! use libpobsd::{Parser, ParserResult, ParsingMode};
//!
//! // Create a parser in strict mode
//! let parser = Parser::new(ParsingMode::Strict);
//! // Load the database
//! let parser_result = parser.load_from_file("/path/to/games.db")
//!        .expect("Problem trying to open the file");
//! let games = match parser_result {
//!     ParserResult::WithoutError(games) => games,
//!     ParserResult::WithError(games, _) => games,
//! };
//! ```
//! The parser can also load from a &str or a String.
//! ```
//! use libpobsd::{Parser, ParserResult, ParsingMode};
//!
//! let games = r#"Game	AaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome
//! Cover	AaaaaA_for_the_Awesome_Cover.jpg
//! Engine
//! Setup
//! Runtime	HumblePlay
//! Store	https://www.humblebundle.com/store/aaaaaaaaaaaaaaaaaaaaaaaaa-for-the-awesome
//! Hints	Demo on HumbleBundle store page
//! Genre
//! Tags
//! Year	2011
//! Dev
//! Pub
//! Version
//! Status
//! Added	1970-01-01
//! Updated	1970-01-01
//! IgdbId	12
//! Game	The Adventures of Mr. Hat
//! Cover
//! Engine	godot
//! Setup
//! Runtime	godot
//! Store	https://store.steampowered.com/app/1869200/The_Adventures_of_Mr_Hat/
//! Hints
//! Genre	Puzzle Platformer
//! Tags	indie
//! Year
//! Dev	AX-GAME
//! Pub	Fun Quarter
//! Version	Early Access
//! Status	runs (2022-05-13)
//! Added	2022-05-13
//! Updated	2022-05-13
//! IgdbId	13"#;
//!
//! let parser = Parser::default();
//! let games = match parser.load_from_string(games) {
//!     ParserResult::WithoutError(games) => games,
//!     // Should not panic since the data are fine
//!     ParserResult::WithError(_, _) => panic!(),
//! };
//!
//! ```
#[macro_use]
pub(crate) mod parser_macros;
pub(crate) mod field;
pub mod game;
pub mod game_status;
pub mod parser;
pub(crate) mod split_line;
pub mod store_links;

pub use self::game::Game;
pub use self::parser::Parser;
pub use self::parser::ParserResult;
pub use self::parser::ParsingMode;
pub use self::store_links::Store;
pub use self::store_links::StoreLink;
pub use self::store_links::StoreLinks;
