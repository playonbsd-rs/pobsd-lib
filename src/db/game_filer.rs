use super::SearchType;
use crate::Game;

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

impl<'a, 'b> GameFilter {
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
    pub fn check_game(&self, game: &Game, search_type: &SearchType) -> bool {
        let check_name = match &self.name {
            Some(name) => game.name_contains(name, search_type),
            None => true,
        };
        let check_engine = match &self.engine {
            Some(engine) => game.engine_contains(engine, search_type),
            None => true,
        };
        let check_runtime = match &self.runtime {
            Some(runtime) => game.runtime_contains(runtime, search_type),
            None => true,
        };
        let check_genre = match &self.genre {
            Some(genre) => game.genres_contains(genre, search_type),
            None => true,
        };
        let check_tag = match &self.tag {
            Some(tag) => game.tags_contains(tag, search_type),
            None => true,
        };
        let check_year = match &self.year {
            Some(year) => game.year_contains(year, search_type),
            None => true,
        };
        let check_dev = match &self.dev {
            Some(dev) => game.devs_contains(dev, search_type),
            None => true,
        };
        let check_publi = match &self.publi {
            Some(publi) => game.publis_contains(publi, search_type),
            None => true,
        };
        let check_status = match &self.status {
            Some(status) => game.status_contains(status, search_type),
            None => true,
        };
        check_name
            && check_engine
            && check_runtime
            && check_genre
            && check_tag
            && check_year
            && check_dev
            && check_publi
            && check_status
    }
    pub fn filter_games(&'a self, games: Vec<&'b Game>, search_type: &SearchType) -> Vec<&'b Game> {
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
    fn test_filter_game_name() {
        let game = create_game();
        let mut filter = GameFilter::default();
        filter.name = Some("Game name".into());
        assert!(filter.check_game(&game, &SearchType::NotCaseSensitive));
        assert!(!filter.check_game(&game, &SearchType::CaseSensitive));
    }
    #[test]
    fn test_filter_game_engine() {
        let game = create_game();
        let mut filter = GameFilter::default();
        filter.engine = Some("Engine".into());
        assert!(filter.check_game(&game, &SearchType::NotCaseSensitive));
        assert!(!filter.check_game(&game, &SearchType::CaseSensitive));
    }
    #[test]
    fn test_filter_game_runtime() {
        let game = create_game();
        let mut filter = GameFilter::default();
        filter.runtime = Some("Runtime".into());
        assert!(filter.check_game(&game, &SearchType::NotCaseSensitive));
        assert!(!filter.check_game(&game, &SearchType::CaseSensitive));
    }
    #[test]
    fn test_filter_game_genre() {
        let game = create_game();
        let mut filter = GameFilter::default();
        filter.genre = Some("Genre1".into());
        assert!(filter.check_game(&game, &SearchType::NotCaseSensitive));
        assert!(!filter.check_game(&game, &SearchType::CaseSensitive));
    }
    #[test]
    fn test_filter_game_tag() {
        let game = create_game();
        let mut filter = GameFilter::default();
        filter.tag = Some("Tag1".into());
        assert!(filter.check_game(&game, &SearchType::NotCaseSensitive));
        assert!(!filter.check_game(&game, &SearchType::CaseSensitive));
    }
    #[test]
    fn test_filter_game_year() {
        let game = create_game();
        let mut filter = GameFilter::default();
        filter.year = Some("1980".into());
        assert!(filter.check_game(&game, &SearchType::NotCaseSensitive));
        assert!(filter.check_game(&game, &SearchType::CaseSensitive));
    }
    #[test]
    fn test_filter_game_dev() {
        let game = create_game();
        let mut filter = GameFilter::default();
        filter.dev = Some("Game dev".into());
        assert!(filter.check_game(&game, &SearchType::NotCaseSensitive));
        assert!(!filter.check_game(&game, &SearchType::CaseSensitive));
    }
    #[test]
    fn test_filter_game_publi() {
        let game = create_game();
        let mut filter = GameFilter::default();
        filter.publi = Some("Game publi".into());
        assert!(filter.check_game(&game, &SearchType::NotCaseSensitive));
        assert!(!filter.check_game(&game, &SearchType::CaseSensitive));
    }
    #[test]
    fn test_filter_game_status() {
        let game = create_game();
        let mut filter = GameFilter::default();
        filter.status = Some("Game status".into());
        assert!(filter.check_game(&game, &SearchType::NotCaseSensitive));
        assert!(!filter.check_game(&game, &SearchType::CaseSensitive));
    }
    #[test]
    fn test_filter_game_status_and_publis() {
        let game = create_game();
        let mut filter = GameFilter::default();
        filter.status = Some("Game status".into());
        filter.publi = Some("Game publi".into());
        assert!(filter.check_game(&game, &SearchType::NotCaseSensitive));
        assert!(!filter.check_game(&game, &SearchType::CaseSensitive));
    }
}
