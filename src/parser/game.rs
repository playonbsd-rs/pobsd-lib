//! Provides a representations of the game in the PlayOnBSD database.
use super::store_links::StoreLinks;
use std::cmp::{Ordering, PartialOrd};
use std::fmt;

/// Represents a game from the database.
///
/// It also includes an additional [`Game::uid`] field
/// derived from the name of the game as well as the date to
/// which the game was added to the database. It therefore
/// provides an unique identifier under the assumption that no
/// game with the same name will be added the same dat into
/// the databas.
///
/// The name of some fields differs from the one used
/// in the database itself: Genre and Store are plural
/// since there can be more than one item for each
/// and Pub translate to publi since pub is a reserved
/// keyword in Rust.
///
/// All fields are optional strings or vectors of strings
/// except for the name of the game which is mandatory.
/// The parser does not try to be smart with dates and
/// just store them as string.
///
/// ### Display
/// The [`Game`] struct implement the [`core::fmt::Display`] trait
/// and will be displayed as it would appear in the
/// PlayOnBSD database.
///
/// ### PartialOrd
/// The [`Game`] struct implements the [`core::cmp::PartialOrd`] trait
/// and [`Game`] objects are ordered according to their name (without The or A).
#[derive(Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Game {
    /// An unique identifier generated from the name and added fields
    pub uid: u32,
    /// The name of the game.
    pub name: String,
    /// The cover of the game.
    pub cover: Option<String>,
    /// The engine used by the game.
    pub engine: Option<String>,
    /// Step(s) to setup the game.
    pub setup: Option<String>,
    /// The executable in the package.
    pub runtime: Option<String>,
    /// A vector with store urls.
    pub stores: Option<StoreLinks>,
    /// Hints (as the name imply).
    pub hints: Option<String>,
    /// A vector of genres associated with the game.
    pub genres: Option<Vec<String>>,
    /// A vector of tags associated with the game.
    pub tags: Option<Vec<String>>,
    /// Released year (can be text such as "early access".
    pub year: Option<String>,
    /// Developer.
    #[serde(rename = "dev")]
    pub devs: Option<Vec<String>>,
    /// Publisher.
    #[serde(rename = "pub")]
    pub publis: Option<Vec<String>>,
    /// Version of the game.
    pub version: Option<String>,
    /// When tested on -current.
    pub status: Option<String>,
    /// When added
    pub added: Option<String>,
    /// When updated
    pub updated: Option<String>,
    /// The IGDB Id of the game
    pub igdb_id: Option<String>,
}

impl<'a> Game {
    pub fn new() -> Self {
        Self::default()
    }
    fn get_ordering_name(&'a self) -> String {
        let name = self.name.to_lowercase();
        let name = if name.starts_with("the ") {
            name.strip_prefix("the ").unwrap().to_string()
        } else if name.starts_with("a ") {
            name.strip_prefix("a ").unwrap().to_string()
        } else {
            name
        };
        name
    }
    pub fn name_contains(&self, value: &str) -> bool {
        self.name.contains(value)
    }
    pub fn engine_contains(&self, value: &str) -> bool {
        self.engine.as_ref().is_some_and(|v| v.contains(value))
    }
    pub fn runtime_contains(&self, value: &str) -> bool {
        self.runtime.as_ref().is_some_and(|v| v.contains(value))
    }
    pub fn genres_contains(&self, value: &str) -> bool {
        !self.genres.as_ref().is_some_and(|v| {
            v.iter()
                .filter(|x| x.contains(value))
                .collect::<Vec<&String>>()
                .is_empty()
        })
    }
    pub fn tags_contains(&self, value: &str) -> bool {
        !self.tags.as_ref().is_some_and(|v| {
            v.iter()
                .filter(|x| x.contains(value))
                .collect::<Vec<&String>>()
                .is_empty()
        })
    }
    pub fn devs_contains(&self, value: &str) -> bool {
        !self.devs.as_ref().is_some_and(|v| {
            v.iter()
                .filter(|x| x.contains(value))
                .collect::<Vec<&String>>()
                .is_empty()
        })
    }

    pub fn publis_contains(&self, value: &str) -> bool {
        !self.publis.as_ref().is_some_and(|v| {
            v.iter()
                .filter(|x| x.contains(value))
                .collect::<Vec<&String>>()
                .is_empty()
        })
    }
    pub fn year_contains(&self, value: &str) -> bool {
        self.year.as_ref().is_some_and(|v| v.contains(value))
    }
    pub fn status_contains(&self, value: &str) -> bool {
        self.status.as_ref().is_some_and(|v| v.contains(value))
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Game) -> Option<Ordering> {
        self.get_ordering_name()
            .partial_cmp(&other.get_ordering_name())
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

/// Display the game as it would appears in the database.
/// See <https://github.com/playonbsd/OpenBSD-Games-Database>
/// for details.
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let game = format!("Game\t{}", self.name);
        let cover = match &self.cover {
            Some(cover) => format!("Cover\t{}", cover),
            None => "Cover".to_string(),
        };
        let engine = match &self.engine {
            Some(engine) => format!("Engine\t{}", engine),
            None => "Engine".to_string(),
        };
        let setup = match &self.setup {
            Some(setup) => format!("Setup\t{}", setup),
            None => "Setup".to_string(),
        };
        let runtime = match &self.runtime {
            Some(runtime) => format!("Runtime\t{}", runtime),
            None => "Runtime".to_string(),
        };
        let stores = match &self.stores {
            Some(stores) => format!(
                "Store\t{}",
                stores
                    .inner_ref()
                    .iter()
                    .map(|a| a.url.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            None => "Store".to_string(),
        };
        let hints = match &self.hints {
            Some(hints) => format!("Hints\t{}", hints),
            None => "Hints".to_string(),
        };
        let genres = match &self.genres {
            Some(genres) => format!("Genre\t{}", genres.join(", ")),
            None => "Genre".to_string(),
        };
        let tags = match &self.tags {
            Some(tags) => format!("Tags\t{}", tags.join(", ")),
            None => "Tags".to_string(),
        };
        let year = match &self.year {
            Some(year) => format!("Year\t{}", year),
            None => "Year".to_string(),
        };
        let dev = match &self.devs {
            Some(devs) => format!("Dev\t{}", devs.join(", ")),
            None => "Dev".to_string(),
        };
        let publi = match &self.publis {
            Some(publis) => format!("Pub\t{}", publis.join(", ")),
            None => "Pub".to_string(),
        };
        let version = match &self.version {
            Some(version) => format!("Version\t{}", version),
            None => "Version".to_string(),
        };
        let status = match &self.status {
            Some(status) => format!("Status\t{}", status),
            None => "Status".to_string(),
        };
        let added = match &self.added {
            Some(added) => format!("Added\t{}", added),
            None => "Added".to_string(),
        };
        let updated = match &self.updated {
            Some(updated) => format!("Updated\t{}", updated),
            None => "Updated".to_string(),
        };
        let igdb_id = match &self.igdb_id {
            Some(runtime) => format!("IgdbId\t{}", runtime),
            None => "IgdbId".to_string(),
        };
        write!(
            f,
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            game,
            cover,
            engine,
            setup,
            runtime,
            stores,
            hints,
            genres,
            tags,
            year,
            dev,
            publi,
            version,
            status,
            added,
            updated,
            igdb_id,
        )
    }
}

/* ------------------------- TESTS --------------------------*/

#[cfg(test)]
mod game_tests {
    use crate::parser::store_links::StoreLink;

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
        game.status = Some("game status".to_string());
        game.added = Some("2012-12-03".to_string());
        game.updated = Some("2014-12-03".to_string());
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
Added
Updated
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
            status: None,
            added: None,
            updated: None,
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
Status\tfine
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
            status: Some("fine".to_string()),
            added: Some("1970-01-01".to_string()),
            updated: Some("1970-01-02".to_string()),
            igdb_id: Some("1234".to_string()),
        };
        assert_eq!(format!("{}", game), game_str);
    }
    #[test]
    fn test_name_contains() {
        let game = create_game();
        assert!(game.name_contains("game"));
        assert!(game.name_contains("name"));
        assert!(game.name_contains("game name"));
        assert!(!game.name_contains("not name"));
    }
    #[test]
    fn test_name_contains_is_case_sensitive() {
        let game = create_game();
        assert!(game.name_contains("name"));
        assert!(!game.name_contains("Name"));
    }
    #[test]
    fn test_engine_contains() {
        let game = create_game();
        assert!(game.engine_contains("game"));
        assert!(game.engine_contains("engine"));
        assert!(game.engine_contains("game engine"));
        assert!(!game.engine_contains("not engine"));
    }
    #[test]
    fn test_engine_contains_is_case_sensitive() {
        let game = create_game();
        assert!(game.engine_contains("engine"));
        assert!(!game.engine_contains("Engine"));
    }
    #[test]
    fn test_runtime_contains() {
        let game = create_game();
        assert!(game.runtime_contains("game"));
        assert!(game.runtime_contains("runtime"));
        assert!(game.runtime_contains("game runtime"));
        assert!(!game.runtime_contains("not runtime"));
    }
    #[test]
    fn test_runtime_contains_is_case_sensitive() {
        let game = create_game();
        assert!(game.runtime_contains("runtime"));
        assert!(!game.runtime_contains("Runtime"));
    }
    #[test]
    fn test_year_contains() {
        let game = create_game();
        assert!(game.year_contains("1980"));
        assert!(!game.year_contains("2000"));
    }
    #[test]
    fn test_year_contains_is_case_sensitive() {
        let mut game = create_game();
        game.year = Some("early access".into());
        assert!(game.year_contains("early"));
        assert!(!game.year_contains("Early"));
    }
    #[test]
    fn test_status_contains() {
        let game = create_game();
        assert!(game.status_contains("game"));
        assert!(game.status_contains("status"));
        assert!(game.status_contains("game status"));
        assert!(!game.status_contains("good"));
    }
    #[test]
    fn test_status_contains_is_case_sensitive() {
        let game = create_game();
        assert!(game.status_contains("status"));
        assert!(!game.status_contains("Status"));
    }
    #[test]
    fn test_genres_contains() {
        let game = create_game();
        assert!(game.genres_contains("genre1"));
        assert!(game.genres_contains("genre2"));
        assert!(game.genres_contains("genre"));
        assert!(!game.status_contains("coucou"));
    }
    #[test]
    fn test_genres_contains_is_case_sensitive() {
        let game = create_game();
        assert!(game.genres_contains("genre"));
        assert!(!game.genres_contains("Genre"));
    }
    #[test]
    fn test_tags_contains() {
        let game = create_game();
        assert!(game.tags_contains("tag1"));
        assert!(game.tags_contains("tag2"));
        assert!(game.tags_contains("tag"));
        assert!(!game.tags_contains("coucou"));
    }
    #[test]
    fn test_tags_contains_is_case_sensitive() {
        let game = create_game();
        assert!(game.tags_contains("tag"));
        assert!(!game.tags_contains("Tag"));
    }
    #[test]
    fn test_devs_contains() {
        let game = create_game();
        assert!(game.devs_contains("game"));
        assert!(game.devs_contains("dev"));
        assert!(game.devs_contains("game dev"));
        assert!(!game.devs_contains("coucou"));
    }
    #[test]
    fn test_devs_contains_is_case_sensitive() {
        let game = create_game();
        assert!(game.devs_contains("game"));
        assert!(!game.devs_contains("Game"));
    }
    #[test]
    fn test_publis_contains() {
        let game = create_game();
        assert!(game.publis_contains("game"));
        assert!(game.publis_contains("publi"));
        assert!(game.publis_contains("game publi"));
        assert!(!game.publis_contains("coucou"));
    }
    #[test]
    fn test_publis_contains_is_case_sensitive() {
        let game = create_game();
        assert!(game.publis_contains("game"));
        assert!(!game.publis_contains("Game"));
    }
}
