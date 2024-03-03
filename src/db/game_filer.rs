use super::SearchType;
use crate::Game;
use paste::paste;

macro_rules! gf_setter {
    ($field:ident) => {
        paste! {
            pub fn [<set_ $field>](&mut self, value: &str) -> &mut Self {
                self.$field = Some(value.into());
                self
            }
        }
    };
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct GameFilter {
    /// The name of the game.
    pub name: Option<String>,
    /// The engine used by the game.
    pub engine: Option<String>,
    /// The executable in the package.
    pub runtime: Option<String>,
    /// A vector of genres associated with the game.
    pub genre: Option<String>,
    /// A vector of tags associated with the game.
    pub tag: Option<String>,
    /// Released year (can be text such as "early access".
    pub year: Option<String>,
    /// Developer.
    pub dev: Option<String>,
    /// Publisher.
    pub publi: Option<String>,
    /// When tested on -current.
    pub status: Option<String>,
}

impl GameFilter {
    pub fn new(
        name: Option<String>,
        engine: Option<String>,
        runtime: Option<String>,
        genre: Option<String>,
        tag: Option<String>,
        year: Option<String>,
        dev: Option<String>,
        publi: Option<String>,
        status: Option<String>,
    ) -> Self {
        Self {
            name,
            engine,
            runtime,
            genre,
            tag,
            year,
            dev,
            publi,
            status,
        }
    }
    gf_setter!(name);
    gf_setter!(engine);
    gf_setter!(runtime);
    gf_setter!(genre);
    gf_setter!(tag);
    gf_setter!(year);
    gf_setter!(dev);
    gf_setter!(publi);
    gf_setter!(status);

    pub fn check_game<T: AsRef<Game>>(
        &self,
        game: T,
        //game: impl AsRef<Game>,
        search_type: &SearchType,
    ) -> bool {
        let check_name = match &self.name {
            Some(name) => game.as_ref().name_contains(name, search_type),
            None => false,
        };
        let check_engine = match &self.engine {
            Some(engine) => game.as_ref().engine_contains(engine, search_type),
            None => false,
        };
        let check_runtime = match &self.runtime {
            Some(runtime) => game.as_ref().runtime_contains(runtime, search_type),
            None => false,
        };
        let check_genre = match &self.genre {
            Some(genre) => game.as_ref().genres_contains(genre, search_type),
            None => false,
        };
        let check_tag = match &self.tag {
            Some(tag) => game.as_ref().tags_contains(tag, search_type),
            None => false,
        };
        let check_year = match &self.year {
            Some(year) => game.as_ref().year_contains(year, search_type),
            None => false,
        };
        let check_dev = match &self.dev {
            Some(dev) => game.as_ref().devs_contains(dev, search_type),
            None => false,
        };
        let check_publi = match &self.publi {
            Some(publi) => game.as_ref().publis_contains(publi, search_type),
            None => false,
        };
        let check_status = match &self.status {
            Some(status) => game.as_ref().status_contains(status, search_type),
            None => false,
        };
        check_name
            || check_engine
            || check_runtime
            || check_genre
            || check_tag
            || check_year
            || check_dev
            || check_publi
            || check_status
    }
    pub fn filter_games<T: AsRef<Game>>(&self, games: Vec<T>, search_type: &SearchType) -> Vec<T> {
        games
            .into_iter()
            .filter(|x| self.check_game(x, search_type))
            .collect()
    }
}

#[cfg(test)]
mod game_tests {
    use crate::parser::store_links::{StoreLink, StoreLinks};

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
    fn test_check_game_name() {
        let game = create_game();
        let mut filter = GameFilter::default();
        filter.set_name("Game name");
        assert!(filter.check_game(&game, &SearchType::NotCaseSensitive));
        assert!(!filter.check_game(&game, &SearchType::CaseSensitive));
    }
    #[test]
    fn test_check_game_engine() {
        let game = create_game();
        let mut filter = GameFilter::default();
        filter.set_engine("Engine");
        assert!(filter.check_game(&game, &SearchType::NotCaseSensitive));
        assert!(!filter.check_game(&game, &SearchType::CaseSensitive));
    }
    #[test]
    fn test_check_game_runtime() {
        let game = create_game();
        let mut filter = GameFilter::default();
        filter.set_runtime("Runtime");
        assert!(filter.check_game(&game, &SearchType::NotCaseSensitive));
        assert!(!filter.check_game(&game, &SearchType::CaseSensitive));
    }
    #[test]
    fn test_check_game_genre() {
        let game = create_game();
        let mut filter = GameFilter::default();
        filter.set_genre("Genre1");
        assert!(filter.check_game(&game, &SearchType::NotCaseSensitive));
        assert!(!filter.check_game(&game, &SearchType::CaseSensitive));
    }
    #[test]
    fn test_check_game_tag() {
        let game = create_game();
        let mut filter = GameFilter::default();
        filter.set_tag("Tag1");
        assert!(filter.check_game(&game, &SearchType::NotCaseSensitive));
        assert!(!filter.check_game(&game, &SearchType::CaseSensitive));
    }
    #[test]
    fn test_check_game_year() {
        let game = create_game();
        let mut filter = GameFilter::default();
        filter.set_year("1980");
        assert!(filter.check_game(&game, &SearchType::NotCaseSensitive));
        assert!(filter.check_game(&game, &SearchType::CaseSensitive));
    }
    #[test]
    fn test_check_game_dev() {
        let game = create_game();
        let mut filter = GameFilter::default();
        filter.set_dev("Game dev");
        assert!(filter.check_game(&game, &SearchType::NotCaseSensitive));
        assert!(!filter.check_game(&game, &SearchType::CaseSensitive));
    }
    #[test]
    fn test_check_game_publi() {
        let game = create_game();
        let mut filter = GameFilter::default();
        filter.set_publi("Game publi");
        assert!(filter.check_game(&game, &SearchType::NotCaseSensitive));
        assert!(!filter.check_game(&game, &SearchType::CaseSensitive));
    }
    #[test]
    fn test_check_game_status() {
        let game = create_game();
        let mut filter = GameFilter::default();
        filter.set_status("Game status");
        assert!(filter.check_game(&game, &SearchType::NotCaseSensitive));
        assert!(!filter.check_game(&game, &SearchType::CaseSensitive));
    }
    #[test]
    fn test_check_game_status_and_publis() {
        let game = create_game();
        let mut filter = GameFilter::default();
        filter.set_status("Game status").set_publi("Game publi");
        assert!(filter.check_game(&game, &SearchType::NotCaseSensitive));
        assert!(!filter.check_game(&game, &SearchType::CaseSensitive));
    }
    #[test]
    fn test_filter_game_status_and_publis() {
        let mut game1 = create_game();
        let mut game2 = game1.clone();
        game1.name = "Game1".into();
        game2.name = "Game2".into();
        let game1bis = game1.clone();
        let games: Vec<Game> = vec![game1, game2];
        let games_ref: Vec<&Game> = games.iter().collect();
        let games_filtered: Vec<Game> = vec![game1bis];
        let games_filtered_ref: Vec<&Game> = games_filtered.iter().collect();
        let mut filter = GameFilter::default();
        filter.set_name("Game1");
        let gf_ref = filter.filter_games(games_ref, &SearchType::CaseSensitive);
        assert_eq!(gf_ref, games_filtered_ref);
        let gf = filter.filter_games(games, &SearchType::CaseSensitive);
        assert_eq!(gf, games_filtered);
    }
}
