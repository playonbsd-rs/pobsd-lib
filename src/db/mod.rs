//! Povides a [`GameDataBase`], [`GameFilter`] and a [`QueryResult`]
//! each struct providing a set of methods to interrogate the PlayOnBSD
//! database in a friendly manner, without having to deal with a SQL
//! database.
//!
//! The [`GameDataBase`] is created from a vector of [`crate::Game`]
//! that can be obtained from the PlayOnBSD database using the [`crate::Parser`].
//!
//! ## Examples
//! Create a GameDataBase from the PlayOnBSD database.
//! ```no_run
//! use libpobsd::{GameDataBase, Game, Parser, ParserResult, ParsingMode, SearchType};
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
//! # use libpobsd::{GameDataBase, Game, Parser, ParserResult, ParsingMode, SearchType};
//! # let games = match Parser::new(ParsingMode::Strict)
//! #       .load_from_file("games.db")
//! #       .expect("Could not open the file")
//! #   {
//! #       ParserResult::WithoutError(games) => games,
//! #       ParserResult::WithError(games, _) => games,
//! #   };
//! # let db = GameDataBase::new(games);
//! let st = SearchType::CaseSensitive;
//! if let Some(game) = db.get_game_by_name("My Game", &st){
//!     assert_eq!(&game.name, "My Game");
//! };
//!```
//! Get all games associated to a given tag.
//! ```no_run
//! # use libpobsd::{GameDataBase, Game, Parser, ParserResult, ParsingMode};
//! # let games = match Parser::new(ParsingMode::Strict)
//! #       .load_from_file("games.db")
//! #       .expect("Could not open the file")
//! #   {
//! #       ParserResult::WithoutError(games) => games,
//! #       ParserResult::WithError(games, _) => games,
//! #   };
//! # let db = GameDataBase::new(games);
//! let game_query = db.match_games_by_tag("indie");
//! // check the first element of the query
//! if let Some(game) = game_query.get(0) {
//!     if let Some(tags) = &game.tags {
//!         assert!(tags.join(" ").contains("indie"));
//!     };
//! };
//!```
pub mod database;
pub mod game_filer;
pub(crate) mod queries;
pub mod query_result;

pub use database::GameDataBase;
pub use game_filer::GameFilter;
pub use query_result::QueryResult;

/// Representation of items such as pub, tags, etc.
pub type Item = String;

#[derive(Debug, Default, Clone)]
/// Define the type of search performed. It can be either case sensitive or not.
pub enum SearchType {
    /// Correspond to a case sensitive search
    CaseSensitive,
    #[default]
    /// Correspond to a case insensitive search. It is the default.
    NotCaseSensitive,
}
