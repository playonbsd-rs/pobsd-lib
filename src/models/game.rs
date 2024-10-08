//! Provides a representations of the game in the PlayOnBSD database.
use crate::{
    models::{
        field::Field,
        game_status::{GameStatus, Status},
        store_links::StoreLinks,
    },
    SearchType, Store,
};

use chrono::NaiveDate;
use paste::paste;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    cmp::{Ordering, PartialOrd},
    fmt,
};

macro_rules! game_contains {
    (name) => {
        /// Returns true if the name field of a [`Game`] contains the given pattern, false otherwise.
        /// The search can be case sensitive or not depending on the [`SearchType`] variant.
        pub fn name_contains(&self, pattern: &str, search_type: &SearchType) -> bool {
            match search_type {
                SearchType::CaseSensitive => self.name.contains(pattern),
                SearchType::NotCaseSensitive => {
                    self.name.to_lowercase().contains(&pattern.to_lowercase())
                }
            }
        }
    };
    ($field:ident) => {
        paste! {
            /// Returns true if the chosen field of a [`Game`] contains the given pattern, false otherwise.
            /// The search can be case sensitive or not depending on the [`SearchType`] variant.
            pub fn [<$field _contains>](&self, pattern: &str, search_type: &SearchType) -> bool {
            match search_type {
                SearchType::CaseSensitive => self.[<$field>].as_ref().is_some_and(|v| v.contains(pattern)),
                SearchType::NotCaseSensitive => self
                    .[< $field>]
                    .as_ref()
                    .is_some_and(|v| v.to_lowercase().contains(&pattern.to_lowercase())),
                }
            }
        }
    };
    (array $field:ident) => {
        paste! {
            /// Returns true if the chosen field of a [`Game`] contains the given pattern, false otherwise.
            /// The search can be case sensitive or not depending on the [`SearchType`] variant.
            pub fn [<$field _contains>](&self, value: &str, search_type: &SearchType) -> bool {
                match search_type {
                    SearchType::CaseSensitive => match self.[<$field>].as_ref() {
                        Some(items) => {
                            !items
                            .iter().filter(|x| x.contains(value))
                            .collect::<Vec<&String>>()
                            .is_empty()
                            },
                        None => false,
                    },
                    SearchType::NotCaseSensitive => match self.[<$field>].as_ref() {
                        Some(items) => {
                            !items
                            .iter().filter(|x| x.to_lowercase().contains(&value.to_lowercase()))
                            .collect::<Vec<&String>>()
                            .is_empty()
                            },
                        None => false,
                    },
                }
            }
        }
    };
}

/// Representation of a game of the PlayOnBSD database.
///
/// It also includes an additional [`Game::uid`] field
/// derived from the name of the game as well as the date to
/// which the game was added to the database. It therefore
/// provides an unique identifier under the assumption that no
/// game with the same name will be added the same date into
/// the database.
///
/// The name of some fields differs from the one used
/// in the database itself: Genre and Store are plural
/// since there can be more than one item for each
/// and Pub translate to publi since pub is a reserved
/// keyword in Rust.
///
/// ### Display
/// The [`Game`] struct implement the [`core::fmt::Display`] trait
/// and will be displayed as it would appear in the
/// PlayOnBSD database.
///
/// ### PartialOrd
/// The [`Game`] struct implements the [`core::cmp::PartialOrd`] trait
/// and [`Game`] objects are ordered according to their name (without The or A).

#[derive(Clone, Default, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Game {
    /// Unique identifier generated from the name and added fields
    pub uid: u32,
    /// Name of the game.
    pub name: String,
    /// Cover of the game.
    pub cover: Option<String>,
    /// Engine used by the game.
    pub engine: Option<String>,
    /// Step(s) to setup the game.
    pub setup: Option<String>,
    /// Executable in the package.
    pub runtime: Option<String>,
    /// Vector with store urls.
    pub stores: Option<StoreLinks>,
    /// Hints (as the name imply).
    pub hints: Option<String>,
    /// Vector of genres associated with the game.
    pub genres: Option<Vec<String>>,
    /// Vector of tags associated with the game.
    pub tags: Option<Vec<String>>,
    /// Released year (can be text such as "early access".
    pub year: Option<String>,
    /// Developer.
    #[cfg_attr(feature = "serde", serde(rename = "dev"))]
    pub devs: Option<Vec<String>>,
    /// Publisher.
    #[cfg_attr(feature = "serde", serde(rename = "pub"))]
    pub publis: Option<Vec<String>>,
    /// Version of the game.
    pub version: Option<String>,
    /// When tested on -current.
    pub status: GameStatus,
    /// When added
    pub added: NaiveDate,
    /// When updated
    pub updated: NaiveDate,
    /// IGDB Id of the game
    pub igdb_id: Option<usize>,
}

impl<'a> Game {
    /// Create a new [`Game`]. Equivalent to Default.
    pub fn new() -> Self {
        Self::default()
    }
    fn get_ordering_name(&'a self) -> String {
        if let Some(name) = self.name.strip_prefix("the ") {
            name.to_lowercase()
        } else if let Some(name) = self.name.strip_prefix("The ") {
            name.to_lowercase()
        } else if let Some(name) = self.name.strip_prefix("a ") {
            name.to_lowercase()
        } else if let Some(name) = self.name.strip_prefix("A ") {
            name.to_lowercase()
        } else {
            self.name.to_lowercase()
        }
    }

    game_contains!(name);
    game_contains!(engine);
    game_contains!(runtime);
    game_contains!(year);

    game_contains!(array genres);
    game_contains!(array tags);
    game_contains!(array devs);
    game_contains!(array publis);

    /// Return true if the [`Status`] of the [`Game`] correspond to a given [`Status`],
    /// false otherwise. Note that the argument provided can be [`Status`] or
    /// [`crate::models::GameStatus`].
    pub fn status_is(&self, status: &impl AsRef<Status>) -> bool {
        self.status.status.eq(status.as_ref())
    }
    /// Returns the Steam id of a [`Game`] if it has any.
    pub fn get_steam_id(&self) -> Option<usize> {
        if let Some(ref stores) = self.stores {
            if stores.has_steam() {
                for store in stores.inner_ref() {
                    match store.store {
                        Store::Steam => return store.id.to_owned(),
                        _ => continue,
                    };
                }
            }
        }
        None
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Game) -> Option<Ordering> {
        Some(self.cmp(other))
    }
    fn lt(&self, other: &Game) -> bool {
        self.get_ordering_name().lt(&other.get_ordering_name())
    }
    fn le(&self, other: &Game) -> bool {
        self.get_ordering_name().le(&other.get_ordering_name())
    }
    fn gt(&self, other: &Game) -> bool {
        self.get_ordering_name().gt(&other.get_ordering_name())
    }
    fn ge(&self, other: &Game) -> bool {
        self.get_ordering_name().ge(&other.get_ordering_name())
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Game) -> Ordering {
        self.get_ordering_name().cmp(&other.get_ordering_name())
    }
}

impl AsRef<Game> for Game {
    fn as_ref(&self) -> &Game {
        self
    }
}

/// Displays the game as it would appears in the database.
/// See <https://github.com/playonbsd/OpenBSD-Games-Database>
/// for details.
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            Field::Game(Some(self.name.to_string())),
            Field::Cover(self.cover.to_owned()),
            Field::Engine(self.engine.to_owned()),
            Field::Setup(self.setup.to_owned()),
            Field::Runtime(self.runtime.to_owned()),
            Field::Store(self.stores.to_owned()),
            Field::Hints(self.hints.to_owned()),
            Field::Genres(self.genres.to_owned()),
            Field::Tags(self.tags.to_owned()),
            Field::Year(self.year.to_owned()),
            Field::Dev(self.devs.to_owned()),
            Field::Publi(self.publis.to_owned()),
            Field::Version(self.version.to_owned()),
            Field::Status(self.status.to_owned()),
            Field::Added(self.added.to_owned()),
            Field::Updated(self.updated.to_owned()),
            Field::IgdbId(self.igdb_id.to_owned()),
        )
    }
}

/* ------------------------- TESTS --------------------------*/

#[cfg(test)]
mod game_tests {
    use crate::models::store_links::StoreLink;

    use super::*;
    fn create_game() -> Game {
        let mut game = Game::default();
        let tags: Vec<String> = vec!["tag1".to_string(), "tag2".to_string()];
        let genres: Vec<String> = vec!["genre1".to_string(), "genre2".to_string()];
        let stores: Vec<String> = vec!["store1".to_string(), "store2".to_string()];
        let store_links: Vec<StoreLink> = stores.into_iter().map(|a| StoreLink::from(&a)).collect();
        let stores = StoreLinks(store_links);
        game.uid = 1221;
        game.name = "game name".to_string();
        game.cover = Some("cover.jpg".to_string());
        game.engine = Some("game engine".to_string());
        game.setup = Some("game setup".to_string());
        game.runtime = Some("game runtime".to_string());
        game.stores = Some(stores);
        game.hints = Some("game hints".to_string());
        game.genres = Some(genres);
        game.tags = Some(tags);
        game.year = Some("1980".to_string());
        game.devs = Some(vec!["game dev".to_string()]);
        game.publis = Some(vec!["game publi".to_string()]);
        game.version = Some("game version".to_string());
        game.status = GameStatus::new(Status::DoesNotRun, Some("game status".to_string()));
        game.added = NaiveDate::parse_from_str("2012-12-03", "%Y-%m-%d").unwrap();
        game.updated = NaiveDate::parse_from_str("2014-12-03", "%Y-%m-%d").unwrap();
        game
    }
    #[test]
    fn test_default_equivalent_to_new() {
        let game = Game::default();
        let game_bis = Game::new();
        assert!(game == game_bis);
    }
    #[test]
    fn test_get_ordering_name_with_a() {
        let mut game = create_game();
        game.name = "A champion".into();
        assert_eq!(game.get_ordering_name(), "champion");
        game.name = "a champion".into();
        assert_eq!(game.get_ordering_name(), "champion");
    }
    #[test]
    fn test_get_ordering_name_with_a_2() {
        let mut game = create_game();
        game.name = "Achampion".into();
        assert_eq!(game.get_ordering_name(), "achampion");
        game.name = "achampion".into();
        assert_eq!(game.get_ordering_name(), "achampion");
    }
    #[test]
    fn test_get_ordering_name_with_the() {
        let mut game = create_game();
        game.name = "The champion".into();
        assert_eq!(game.get_ordering_name(), "champion");
        game.name = "the champion".into();
        assert_eq!(game.get_ordering_name(), "champion");
    }
    #[test]
    fn test_get_ordering_name_with_the_2() {
        let mut game = create_game();
        game.name = "Thechampion".into();
        assert_eq!(game.get_ordering_name(), "thechampion");
        game.name = "thechampion".into();
        assert_eq!(game.get_ordering_name(), "thechampion");
    }
    #[test]
    fn test_ordering() {
        use std::cmp::Ordering;
        let mut game1 = create_game();
        let mut game2 = create_game();
        game1.name = "Abc".into();
        game2.name = "Def".into();
        assert!(game2.gt(&game1));
        assert!(game2.ge(&game1));
        assert!(game1.le(&game2));
        assert!(game1.lt(&game2));
        assert_eq!(game2.cmp(&game1), Ordering::Greater);
        assert_eq!(game1.cmp(&game2), Ordering::Less);
        assert_eq!(game2.cmp(&game2), Ordering::Equal);
        assert_eq!(game2.partial_cmp(&game1), Some(Ordering::Greater));
        assert_eq!(game1.partial_cmp(&game2), Some(Ordering::Less));
        assert_eq!(game2.partial_cmp(&game2), Some(Ordering::Equal));
        game1.name = "The Abc".into();
        game2.name = "def".into();
        assert!(game2.gt(&game1));
        assert!(game2.ge(&game1));
        assert!(game1.le(&game2));
        assert!(game1.lt(&game2));
        game1.name = "The Abc".into();
        game2.name = "A def".into();
        assert!(game2.gt(&game1));
        assert!(game2.ge(&game1));
        assert!(game1.le(&game2));
        assert!(game1.lt(&game2));
    }
    #[test]
    fn test_display_1() {
        let game_str = "Game\tAaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome
Cover\tAaaaaA_for_the_Awesome_Cover.jpg
Engine
Setup
Runtime\tHumblePlay
Store\thttps://www.humblebundle.com/store/aaaaaaaaaaaaaaaaaaaaaaaaa-for-the-awesome
Hints\tDemo on HumbleBundle store page
Genre
Tags
Year\t2011
Dev
Pub
Version
Status
Added\t1970-01-01
Updated\t1970-01-01
IgdbId";
        let game = Game {
            uid: 12,
            name: "AaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome".to_string(),
            cover: Some("AaaaaA_for_the_Awesome_Cover.jpg".to_string()),
            engine: None,
            setup: None,
            runtime: Some("HumblePlay".to_string()),
            stores: Some(StoreLinks(vec![StoreLink::from(
                "https://www.humblebundle.com/store/aaaaaaaaaaaaaaaaaaaaaaaaa-for-the-awesome",
            )])),
            hints: Some("Demo on HumbleBundle store page".to_string()),
            genres: None,
            tags: None,
            year: Some("2011".to_string()),
            devs: None,
            publis: None,
            version: None,
            status: GameStatus::default(),
            added: NaiveDate::default(),
            updated: NaiveDate::default(),
            igdb_id: None,
        };
        assert_eq!(format!("{}", game), game_str);
    }
    #[test]
    fn test_display_2() {
        let game_str = "Game\tAaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome
Cover
Engine\tEngine1
Setup\tSetup1
Runtime
Store
Hints
Genre\tgenre1, genre2
Tags\ttag1, tag2
Year
Dev\tdev1
Pub\tpub1
Version\tver1
Status\t0 fine
Added\t1970-01-01
Updated\t1970-01-02
IgdbId\t1234";
        let game = Game {
            uid: 12,
            name: "AaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome".to_string(),
            cover: None,
            engine: Some("Engine1".to_string()),
            setup: Some("Setup1".to_string()),
            runtime: None,
            stores: None,
            hints: None,
            genres: Some(vec!["genre1".to_string(), "genre2".to_string()]),
            tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
            year: None,
            devs: Some(vec!["dev1".to_string()]),
            publis: Some(vec!["pub1".to_string()]),
            version: Some("ver1".to_string()),
            status: GameStatus::new(Status::DoesNotRun, Some("fine".to_string())),
            added: NaiveDate::parse_from_str("1970-01-01", "%Y-%m-%d").unwrap(),
            updated: NaiveDate::parse_from_str("1970-01-02", "%Y-%m-%d").unwrap(),
            igdb_id: Some(1234),
        };
        assert_eq!(format!("{}", game), game_str);
    }
    #[test]
    fn test_name_contains() {
        let game = create_game();
        let st = SearchType::CaseSensitive;
        assert!(game.name_contains("game", &st));
        assert!(game.name_contains("name", &st));
        assert!(game.name_contains("game name", &st));
        assert!(!game.name_contains("not name", &st));
        let st = SearchType::NotCaseSensitive;
        assert!(game.name_contains("game", &st));
        assert!(game.name_contains("name", &st));
        assert!(game.name_contains("game name", &st));
        assert!(!game.name_contains("not name", &st));
    }
    #[test]
    fn test_name_contains_is_case_sensitive() {
        let game = create_game();
        let st = SearchType::CaseSensitive;
        assert!(game.name_contains("name", &st));
        assert!(!game.name_contains("Name", &st));
    }
    #[test]
    fn test_name_contains_is_not_case_sensitive() {
        let game = create_game();
        let st = SearchType::NotCaseSensitive;
        assert!(game.name_contains("name", &st));
        assert!(game.name_contains("Name", &st));
    }
    #[test]
    fn test_engine_contains() {
        let game = create_game();
        let st = SearchType::CaseSensitive;
        assert!(game.engine_contains("game", &st));
        assert!(game.engine_contains("engine", &st));
        assert!(game.engine_contains("game engine", &st));
        assert!(!game.engine_contains("not engine", &st));
        let st = SearchType::NotCaseSensitive;
        assert!(game.engine_contains("game", &st));
        assert!(game.engine_contains("engine", &st));
        assert!(game.engine_contains("game engine", &st));
        assert!(!game.engine_contains("not engine", &st));
    }
    #[test]
    fn test_engine_contains_is_case_sensitive() {
        let game = create_game();
        let st = SearchType::CaseSensitive;
        assert!(game.engine_contains("engine", &st));
        assert!(!game.engine_contains("Engine", &st));
    }
    #[test]
    fn test_engine_contains_is_not_case_sensitive() {
        let game = create_game();
        let st = SearchType::NotCaseSensitive;
        assert!(game.engine_contains("engine", &st));
        assert!(game.engine_contains("Engine", &st));
    }
    #[test]
    fn test_runtime_contains() {
        let game = create_game();
        let st = SearchType::CaseSensitive;
        assert!(game.runtime_contains("game", &st));
        assert!(game.runtime_contains("runtime", &st));
        assert!(game.runtime_contains("game runtime", &st));
        assert!(!game.runtime_contains("not runtime", &st));
        let st = SearchType::NotCaseSensitive;
        assert!(game.runtime_contains("game", &st));
        assert!(game.runtime_contains("runtime", &st));
        assert!(game.runtime_contains("game runtime", &st));
        assert!(!game.runtime_contains("not runtime", &st));
    }
    #[test]
    fn test_runtime_contains_is_case_sensitive() {
        let game = create_game();
        let st = SearchType::CaseSensitive;
        assert!(game.runtime_contains("runtime", &st));
        assert!(!game.runtime_contains("Runtime", &st));
    }
    #[test]
    fn test_runtime_contains_is_not_case_sensitive() {
        let game = create_game();
        let st = SearchType::NotCaseSensitive;
        assert!(game.runtime_contains("runtime", &st));
        assert!(game.runtime_contains("Runtime", &st));
    }
    #[test]
    fn test_year_contains() {
        let game = create_game();
        let st = SearchType::CaseSensitive;
        assert!(game.year_contains("1980", &st));
        assert!(!game.year_contains("2000", &st));
        let st = SearchType::NotCaseSensitive;
        assert!(game.year_contains("1980", &st));
        assert!(!game.year_contains("2000", &st));
    }
    #[test]
    fn test_year_contains_is_case_sensitive() {
        let mut game = create_game();
        let st = SearchType::CaseSensitive;
        game.year = Some("early access".into());
        assert!(game.year_contains("early", &st));
        assert!(!game.year_contains("Early", &st));
    }
    #[test]
    fn test_year_contains_is_not_case_sensitive() {
        let mut game = create_game();
        let st = SearchType::NotCaseSensitive;
        game.year = Some("early access".into());
        assert!(game.year_contains("early", &st));
        assert!(game.year_contains("Early", &st));
    }
    #[test]
    fn test_status_contains() {
        let game = create_game();
        let status = Status::DoesNotRun;
        assert!(game.status_is(&status));
    }
    #[test]
    fn test_genres_contains() {
        let game = create_game();
        let st = SearchType::CaseSensitive;
        assert!(game.genres_contains("genre1", &st));
        assert!(game.genres_contains("genre2", &st));
        assert!(game.genres_contains("genre", &st));
        assert!(!game.genres_contains("coucou", &st));
        let st = SearchType::NotCaseSensitive;
        assert!(game.genres_contains("genre1", &st));
        assert!(game.genres_contains("genre2", &st));
        assert!(game.genres_contains("genre", &st));
        assert!(!game.genres_contains("coucou", &st));
    }
    #[test]
    fn test_genres_contains_is_case_sensitive() {
        let game = create_game();
        let st = SearchType::CaseSensitive;
        assert!(game.genres_contains("genre", &st));
        assert!(!game.genres_contains("Genre", &st));
    }
    #[test]
    fn test_genres_contains_is_not_case_sensitive() {
        let game = create_game();
        let st = SearchType::NotCaseSensitive;
        assert!(game.genres_contains("genre", &st));
        assert!(game.genres_contains("Genre", &st));
    }
    #[test]
    fn test_tags_contains() {
        let game = create_game();
        let st = SearchType::CaseSensitive;
        assert!(game.tags_contains("tag1", &st));
        assert!(game.tags_contains("tag2", &st));
        assert!(game.tags_contains("tag", &st));
        assert!(!game.tags_contains("coucou", &st));
        let st = SearchType::NotCaseSensitive;
        assert!(game.tags_contains("tag1", &st));
        assert!(game.tags_contains("tag2", &st));
        assert!(game.tags_contains("tag", &st));
        assert!(!game.tags_contains("coucou", &st));
    }
    #[test]
    fn test_tags_contains_is_case_sensitive() {
        let game = create_game();
        let st = SearchType::CaseSensitive;
        assert!(game.tags_contains("tag", &st));
        assert!(!game.tags_contains("Tag", &st));
    }
    #[test]
    fn test_tags_contains_is_not_case_sensitive() {
        let game = create_game();
        let st = SearchType::NotCaseSensitive;
        assert!(game.tags_contains("tag", &st));
        assert!(game.tags_contains("Tag", &st));
    }
    #[test]
    fn test_devs_contains() {
        let game = create_game();
        let st = SearchType::CaseSensitive;
        assert!(game.devs_contains("game", &st));
        assert!(game.devs_contains("dev", &st));
        assert!(game.devs_contains("game dev", &st));
        assert!(!game.devs_contains("coucou", &st));
        let st = SearchType::NotCaseSensitive;
        assert!(game.devs_contains("game", &st));
        assert!(game.devs_contains("dev", &st));
        assert!(game.devs_contains("game dev", &st));
        assert!(!game.devs_contains("coucou", &st));
    }
    #[test]
    fn test_devs_contains_is_case_sensitive() {
        let game = create_game();
        let st = SearchType::CaseSensitive;
        assert!(game.devs_contains("game", &st));
        assert!(!game.devs_contains("Game", &st));
    }
    #[test]
    fn test_devs_contains_is_not_case_sensitive() {
        let game = create_game();
        let st = SearchType::NotCaseSensitive;
        assert!(game.devs_contains("game", &st));
        assert!(game.devs_contains("Game", &st));
    }
    #[test]
    fn test_publis_contains() {
        let game = create_game();
        let st = SearchType::CaseSensitive;
        assert!(game.publis_contains("game", &st));
        assert!(game.publis_contains("publi", &st));
        assert!(game.publis_contains("game publi", &st));
        assert!(!game.publis_contains("coucou", &st));
        let st = SearchType::NotCaseSensitive;
        assert!(game.publis_contains("game", &st));
        assert!(game.publis_contains("publi", &st));
        assert!(game.publis_contains("game publi", &st));
        assert!(!game.publis_contains("coucou", &st));
    }
    #[test]
    fn test_publis_contains_is_case_sensitive() {
        let game = create_game();
        let st = SearchType::CaseSensitive;
        assert!(game.publis_contains("game", &st));
        assert!(!game.publis_contains("Game", &st));
    }
    #[test]
    fn test_publis_contains_is_not_case_sensitive() {
        let game = create_game();
        let st = SearchType::NotCaseSensitive;
        assert!(game.publis_contains("game", &st));
        assert!(game.publis_contains("Game", &st));
    }
}
