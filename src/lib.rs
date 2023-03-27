//! This library provides a set of methods to interogate the PlayOnBSD
//! database in a friendly manner, without having to deal with a SQL
//! database.
//! ## Exemples
//! Create a GameDataBase from the PlayOnBSD database.
//! ```no_run
//! extern crate pobsd_db;
//! extern crate pobsd_parser;
//! use pobsd_db::GameDataBase;
//! use pobsd_parser::{Game, Parser, ParserResult, ParsingMode};
//! // loading the games from the database
//! let games = match Parser::new(ParsingMode::Strict)
//!        .load_from_file("games.db")
//!        .expect("Could not open the file")
//!    {
//!        ParserResult::WithoutError(games) => games,
//!        ParserResult::WithError(games, _) => games,
//!    };
//! GameDataBase::new(games);
//!```
//! Get a game by name.
//! ```no_run
//! # extern crate pobsd_db;
//! # extern crate pobsd_parser;
//! # use pobsd_db::GameDataBase;
//! # use pobsd_parser::{Game, Parser, ParserResult, ParsingMode};
//! # let games = match Parser::new(ParsingMode::Strict)
//! #       .load_from_file("games.db")
//! #       .expect("Could not open the file")
//! #   {
//! #       ParserResult::WithoutError(games) => games,
//! #       ParserResult::WithError(games, _) => games,
//! #   };
//! # let db = GameDataBase::new(games);
//! if let Some(game) = db.get_game_by_name("My Game"){
//!     assert_eq!(&game.name, "My Game");
//! };
//!```
//! Get all games associated to a givent tag.
//! ```no_run
//! # extern crate pobsd_db;
//! # extern crate pobsd_parser;
//! # use pobsd_db::GameDataBase;
//! # use pobsd_parser::{Game, Parser, ParserResult, ParsingMode};
//! # let games = match Parser::new(ParsingMode::Strict)
//! #       .load_from_file("games.db")
//! #       .expect("Could not open the file")
//! #   {
//! #       ParserResult::WithoutError(games) => games,
//! #       ParserResult::WithError(games, _) => games,
//! #   };
//! # let db = GameDataBase::new(games);
//! let game_query = db.get_game_by_tag("indie");
//! // check the first element of the query
//! if let Some(game) = game_query.get(0) {
//!     if let Some(tags) = &game.tags {
//!         assert!(tags.join(" ").contains("indie"));
//!     };
//! };
//!```
pub mod database;
pub(crate) mod queries;
pub mod query_result;

pub use database::GameDataBase;
pub use query_result::QueryResult;

/// Representation of items such as pub, tags, etc.
pub type Item = String;
