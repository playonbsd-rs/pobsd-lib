use crate::db::Item;
use crate::models::Store;
use crate::{Game, GameDataBase, GameFilter, QueryResult, SearchType};

use paste::paste;

macro_rules! match_games_by {
    ($field:ident) => {
        paste! {
            /// Returns the games for which the searched field exactly matches the given value.
            pub fn [<match_games_by_ $field>](&self, field: &str) -> QueryResult<&Game> {
                match self.[<$field s>].get(field) {
                    Some(game_ids) => {
                        let mut games: Vec<&Game> = Vec::new();
                        for game_id in game_ids {
                            if let Some(game) = self.games.get(game_id) {
                                games.push(game)
                            }
                        }
                        QueryResult::new(games)
                    }
                    None => QueryResult::new(vec![]),
                }
            }
        }
    };
}

macro_rules! search_games_by {
    ($field:ident) => {
        paste! {
            /// Returns the games for which the chosen field contains the given value.
            /// It can be case sensitive or insensitive depending on the
            /// [`SearchType`] variant.
            pub fn [<search_games_by_ $field>](&self, pattern: &str, search_type: &SearchType) -> QueryResult<&Game> {
                let games = GameFilter::default()
                        .[<set_ $field>](pattern)
                        .filter_games(self.games.values().collect(), search_type);
                QueryResult::new(games)
            }
        }
    };
}

macro_rules! get_all {
    ($field:ident) => {
        paste! {
            /// Returns all the items for the chosen field.
            pub fn [<get_all_ $field>](&self) -> QueryResult<&Item> {
                let items: Vec<&Item> = self.$field.keys().collect();
                QueryResult::new(items)
            }
        }
    };
}

macro_rules! get_all_with_ids {
    ($field:ident) => {
        paste! {
            /// Returns all the items for the chosen field as well as the game ids
            /// associated to each item.
            pub fn [<get_all_ $field _with_ids>](&self) -> Vec<(String, Vec<u32>)> {
                let mut items: Vec<(String, Vec<u32>)> = self.$field.iter().map(|a| (a.0.clone(), a.1.clone())).collect();
                items.sort_by(|a,b| a.0.cmp(&b.0));
                items
            }
        }
    };
}

impl GameDataBase {
    /// Returns the game with the given id.
    pub fn get_game_by_id(&self, game_id: u32) -> Option<&Game> {
        self.games.get(&game_id)
    }
    /// Returns the first game found which names contains the given name.
    /// It can be case sensitive or insensitive depending on the
    /// [`SearchType`] variant.
    pub fn get_game_by_name(&self, name: &str, search_type: &SearchType) -> Option<&Game> {
        let mut filter = GameFilter::default();
        filter.set_name(name);
        self.games
            .values()
            .find(|game| filter.check_game(game, search_type))
    }
    /// Returns the game with the given steam_id.
    pub fn get_game_by_steam_id(&self, steam_id: usize) -> Option<&Game> {
        for game in self.games.values() {
            if let Some(stores) = &game.stores {
                for store in stores.inner_ref() {
                    if store.store.eq(&Store::Steam) && store.id.eq(&Some(steam_id)) {
                        return Some(game);
                    }
                }
            }
        }
        None
    }

    /// Returns all games matching the given vector of game ids.
    pub fn match_games_by_ids(&self, game_ids: Vec<u32>) -> QueryResult<&Game> {
        let mut games: Vec<&Game> = Vec::new();
        for game_id in game_ids {
            if let Some(game) = self.get_game_by_id(game_id) {
                games.push(game);
            }
        }
        QueryResult::new(games)
    }
    match_games_by!(tag);
    match_games_by!(year);
    match_games_by!(engine);
    match_games_by!(runtime);
    match_games_by!(genre);
    match_games_by!(dev);
    match_games_by!(publi);

    search_games_by!(name);
    search_games_by!(tag);
    search_games_by!(year);
    search_games_by!(engine);
    search_games_by!(runtime);
    search_games_by!(genre);
    search_games_by!(dev);
    search_games_by!(publi);

    /// Returns the games filtered using the [`GameFilter`].
    pub fn search_game_by_filter(
        &self,
        search_type: &SearchType,
        filter: &GameFilter,
    ) -> QueryResult<&Game> {
        let games = filter.filter_games(self.games.values().collect(), search_type);
        QueryResult::new(games)
    }

    /// Returns all games as a QueryResult.
    pub fn get_all_games(&self) -> QueryResult<&Game> {
        let mut games: Vec<&Game> = self.games.values().collect();
        games.sort();
        QueryResult::new(games)
    }
    get_all!(tags);
    get_all!(engines);
    get_all!(runtimes);
    get_all!(genres);
    get_all!(years);
    get_all!(devs);
    get_all!(publis);

    get_all_with_ids!(tags);
    get_all_with_ids!(engines);
    get_all_with_ids!(runtimes);
    get_all_with_ids!(genres);
    get_all_with_ids!(years);
    get_all_with_ids!(devs);
    get_all_with_ids!(publis);
}
