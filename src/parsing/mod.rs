//! Provides a simplistic [`Parser`] that converts
//! the [PlayOnBSD Database](https://github.com/playonbsd/OpenBSD-Games-Database)
//! (either provided as a string or as a file) into a vector of [`Game`].
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
//! The parser can also load from a [`&str`] or a [`String`].
//! ```
//! use libpobsd::{Parser, ParserResult, ParsingMode, Game};
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
//! let game1: &Game = games.get(1).unwrap();
//! assert_eq!(Some(String::from("godot")), game1.engine);
//!
//! ```
#[macro_use]
pub(crate) mod parser_macros;

use crate::models::field::Field;
use crate::Game;

use hash32::{FnvHasher, Hasher};
use std::fs;
use std::hash::Hash;
use std::path::Path;

enum ParserState {
    Game,
    Cover,
    Engine,
    Setup,
    Runtime,
    Store,
    Hints,
    Genre,
    Tags,
    Year,
    Dev,
    Pub,
    Version,
    Status,
    Added,
    Updated,
    IgdbId,
    Error,
    Recovering,
}

/// Represent the two parsing modes supported by [`Parser`].
pub enum ParsingMode {
    /// In **strict mode**, the parsing will stop if a parsing error occurs
    /// returning the games processed before the error as well as the line
    /// in the input (file or string) where the error occurred.
    Strict,
    /// In **relaxed mode**, the parsing will continue even after an error
    /// is encountered, the parsing resuming when reaching the next game
    /// after the parsing error, and returning all the games that have been
    /// parsed as well as the line numbers that were ignored due to parsing
    /// errors.
    Relaxed,
}

/// Represent the result of the parsing. When in in strict mode,
/// only the games parsed before a parsing error occurred will
/// be returned. In relaxed mode, the parser will do its best
/// to continue parsing games.
pub enum ParserResult {
    /// Result of the parsing when an error occurred. It holds a vector
    /// of [`Game`] parsed from the database and a vector of the lines where
    /// errors occurred.
    WithError(Vec<Game>, Vec<usize>),
    /// Result of the parsing when no error occurred. It holds a vector
    /// of [`Game`] parsed from the database.
    WithoutError(Vec<Game>),
}

impl From<ParserResult> for Vec<Game> {
    fn from(val: ParserResult) -> Self {
        match val {
            ParserResult::WithError(games, _) => games,
            ParserResult::WithoutError(games) => games,
        }
    }
}
/// [`Parser`] parses the PlayOnBSD database provided as a [`&str`] or from
/// a file and returns a [`ParserResult`] holding a vector of [`Game`] contained
/// in the PlayOnBSD database.
pub struct Parser {
    state: ParserState,
    games: Vec<Game>,
    current_line: usize,
    error_lines: Vec<usize>,
    mode: ParsingMode,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            state: ParserState::Game,
            games: Vec::new(),
            current_line: 0,
            error_lines: Vec::new(),
            mode: ParsingMode::Relaxed,
        }
    }
}
impl Parser {
    /// Crate a [`Parser`] set to the given parsing mode.
    pub fn new(mode: ParsingMode) -> Self {
        Self {
            state: ParserState::Game,
            games: Vec::new(),
            current_line: 0,
            error_lines: Vec::new(),
            mode,
        }
    }
    /// Load the database from a file.
    pub fn load_from_file(self, file: impl AsRef<Path>) -> Result<ParserResult, std::io::Error> {
        let file: &Path = file.as_ref();
        if file.is_file() {
            let data = fs::read_to_string(file)?;
            Ok(self.load_from_string(&data))
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "This is not a file",
            ))
        }
    }
    /// Load the database from a string.
    pub fn load_from_string(mut self, data: &str) -> ParserResult {
        for line in data.lines() {
            self.current_line += 1;
            self.parse(line);
            if let ParserState::Error = self.state {
                self.error_lines.push(self.current_line);
                if let ParsingMode::Strict = self.mode {
                    break;
                }
            };
        }
        for game in &mut self.games {
            let mut fnv = FnvHasher::default();
            // This is ugly but for compatibility
            // uid should not change while updating
            // libpobsd
            let added = game.added.format("%Y-%m-%d").to_string();
            Some(added).hash(&mut fnv);
            game.name.hash(&mut fnv);
            game.uid = fnv.finish32();
        }
        match self.error_lines.is_empty() {
            false => ParserResult::WithError(self.games, self.error_lines),
            true => ParserResult::WithoutError(self.games),
        }
    }
    impl_parse![ParserState::Game, Field::Game, name, ParserState::Cover;
         (ParserState::Cover, Field::Cover, cover, ParserState::Engine);
         (ParserState::Engine, Field::Engine, engine, ParserState::Setup);
         (ParserState::Setup, Field::Setup, setup, ParserState::Runtime);
         (ParserState::Runtime, Field::Runtime, runtime, ParserState::Store);
         (ParserState::Store, Field::Store, stores, ParserState::Hints);
         (ParserState::Hints, Field::Hints, hints, ParserState::Genre);
         (ParserState::Genre, Field::Genres, genres, ParserState::Tags);
         (ParserState::Tags, Field::Tags, tags, ParserState::Year);
         (ParserState::Year, Field::Year, year, ParserState::Dev);
         (ParserState::Dev, Field::Dev, devs, ParserState::Pub);
         (ParserState::Pub, Field::Publi, publis, ParserState::Version);
         (ParserState::Version, Field::Version, version, ParserState::Status);
         (ParserState::Status, Field::Status, status, ParserState::Added);
         (ParserState::Added, Field::Added, added, ParserState::Updated);
         (ParserState::Updated, Field::Updated, updated, ParserState::IgdbId);
         (ParserState::IgdbId, Field::IgdbId, igdb_id, ParserState::Game)
    ];
}
#[cfg(test)]
mod game_tests {
    use super::*;
    #[test]
    fn test_from_parse_result_without_error_to_vec() {
        let game = Game::new();
        let game_bis = Game::new();
        let games1 = vec![game, game_bis];
        let games2 = games1.clone();
        let parse_result = ParserResult::WithoutError(games2);
        let games_test: Vec<Game> = parse_result.into();
        assert_eq!(games1, games_test);
    }
    #[test]
    fn test_from_parse_result_with_error_to_vec() {
        let game = Game::new();
        let game_bis = Game::new();
        let games1 = vec![game, game_bis];
        let games2 = games1.clone();
        let parse_result = ParserResult::WithError(games2, vec![]);
        let games_test: Vec<Game> = parse_result.into();
        assert_eq!(games1, games_test);
    }
    #[test]
    fn load_from_file_fail() {
        let re = match Parser::default().load_from_file("nothere") {
            Ok(_) => panic!(),
            Err(e) => e,
        };
        let error_type = std::io::ErrorKind::InvalidInput;
        assert_eq!(re.kind(), error_type);
    }
}
