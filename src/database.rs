//! Provide a representation of the PlayOnBSD database than can be
//! queried using a set of predefined methods.
use paste::paste;
use pobsd_parser::Game;
use std::collections::HashMap;

macro_rules! add_game_to {
    ($field:ident) => {
        paste! {
            fn [<add_game_to_ $field>](&mut self, item: &str, game_id: u32) {
                match self.[<$field>].get_mut(item) {
                    Some(item) => item.push(game_id),
                    None => {
                        let _ = self.[<$field>].insert(item.into(), vec![game_id]);
                    }
                }
            }
        }
    };
}

macro_rules! push {
    ($game:ident, $field:ident, $db:ident ) => {
        paste! {
            if let Some(item) = &$game.$field {
                $db.[<add_game_to_ $field s>](&item, $game.uid);
            }
        }
    };
    ($game:ident, array $field:ident, $db:ident) => {
        paste! {
            if let Some(items) = &$game.$field {
                for item in items {
                    $db.[<add_game_to_ $field>](&item, $game.uid);
                }
            }
        }
    };
}

/// Representation of the PlayOnBSD database that provides
/// a set of methods to query and filter the games.
#[derive(Default)]
pub struct GameDataBase {
    /// HashMap using the game uid as key and the corresponding game as value
    pub(crate) games: HashMap<u32, Game>,
    /// HashMap using the engine name as key and vector of game uid corresponding to said engine as value
    pub(crate) engines: HashMap<String, Vec<u32>>,
    /// HashMap using the runtime name as key and vector of game uid corresponding to said engine as value
    pub(crate) runtimes: HashMap<String, Vec<u32>>,
    /// HashMap using the genre name as key and vector of game uid corresponding to said engine as value
    pub(crate) genres: HashMap<String, Vec<u32>>,
    /// HashMap using the tag name as key and vector of game uid corresponding to said engine as value
    pub(crate) tags: HashMap<String, Vec<u32>>,
    /// HashMap using the year as key and vector of game uid corresponding to said engine as value
    pub(crate) years: HashMap<String, Vec<u32>>,
    /// HashMap using the dev name as key and vector of game uid corresponding to said engine as value
    pub(crate) devs: HashMap<String, Vec<u32>>,
    /// HashMap using the pub name as key and vector of game uid corresponding to said engine as value
    pub(crate) publis: HashMap<String, Vec<u32>>,
}

impl GameDataBase {
    /// Create a database for the given vector of games
    pub fn new(games: Vec<Game>) -> Self {
        let mut db = GameDataBase::default();
        for game in games {
            db.load_game(game);
        }
        db
    }
    /// Load the given game in the database
    pub fn load_game(&mut self, game: Game) {
        let uid = game.uid;
        self.add_game(game);
        let game = self.games.get(&uid).unwrap().clone();

        push!(game, engine, self);
        push!(game, runtime, self);
        push!(game, array genres, self);
        push!(game, array tags, self);
        push!(game, year, self);
        push!(game, dev, self);
        push!(game, publi, self);
    }
    fn add_game(&mut self, game: Game) {
        self.games.insert(game.uid, game);
    }
    add_game_to!(tags);
    add_game_to!(engines);
    add_game_to!(runtimes);
    add_game_to!(genres);
    add_game_to!(years);
    add_game_to!(devs);
    add_game_to!(publis);
}
