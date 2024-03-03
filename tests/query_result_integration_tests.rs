extern crate libpobsd;
use libpobsd::db::{GameDataBase, SearchType};
use libpobsd::parser::{Game, Parser, ParserResult, ParsingMode};

// HELPER FUNCTIONS

// helper function to return the games with both
// correct and faulty database in relaxed mode
fn get_games_strict(file: &str) -> Vec<Game> {
    match Parser::new(ParsingMode::Strict)
        .load_from_file(file)
        .expect("Could not open the file")
    {
        ParserResult::WithoutError(games) => games,
        ParserResult::WithError(games, _) => games,
    }
}

fn get_db_strict() -> GameDataBase {
    let games = get_games_strict("tests/data/test-games.db");
    GameDataBase::new(games)
}

//-------------------
// GET
//-------------------

// Test get_by_name method
#[test]
fn test_get_game_by_name_game_exists() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let game = qr
        .get_game_by_name("Airships: Conquer the Skies")
        .expect("Game with id 1595434339 exists");
    assert_eq!(game.uid, 1595434339);
}
#[test]
fn test_get_game_by_name_game_does_not_exist() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let game = qr.get_game_by_name("I do not exist");
    assert_eq!(game, None);
}

// Test get_game_by_tag
#[test]
fn test_get_game_by_tag_tag_exists() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::CaseSensitive;
    let game_query = qr.clone().get_game_by_tag("indie", &st);
    assert_eq!(game_query.items.len(), 3);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "The Adventures of Mr. Hat".to_string());
    let game = game_query.items.get(1).unwrap();
    assert_eq!(game.name, "The Adventures of Shuggy".to_string());
    let game = game_query.items.get(2).unwrap();
    assert_eq!(game.name, "Aeternum".to_string());
    let st = SearchType::NotCaseSensitive;
    let game_query = qr.get_game_by_tag("indie", &st);
    assert_eq!(game_query.items.len(), 3);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "The Adventures of Mr. Hat".to_string());
    let game = game_query.items.get(1).unwrap();
    assert_eq!(game.name, "The Adventures of Shuggy".to_string());
    let game = game_query.items.get(2).unwrap();
    assert_eq!(game.name, "Aeternum".to_string());
}
#[test]
fn test_get_game_by_tag_tag_exists_case_sensitive() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::CaseSensitive;
    let game_query = qr.clone().get_game_by_tag("inDie", &st);
    assert_eq!(game_query.items.len(), 0);
}
#[test]
fn test_get_game_by_tag_tag_exists_not_case_sensitive() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::NotCaseSensitive;
    let game_query = qr.clone().get_game_by_tag("inDie", &st);
    assert_eq!(game_query.items.len(), 3);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "The Adventures of Mr. Hat".to_string());
    let game = game_query.items.get(1).unwrap();
    assert_eq!(game.name, "The Adventures of Shuggy".to_string());
    let game = game_query.items.get(2).unwrap();
    assert_eq!(game.name, "Aeternum".to_string());
}
#[test]
fn test_get_game_by_tag_tag_does_not_exist() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::CaseSensitive;
    let game_query = qr.clone().get_game_by_tag("I do not exist", &st);
    assert_eq!(game_query.items.len(), 0);
    let st = SearchType::NotCaseSensitive;
    let game_query = qr.get_game_by_tag("I do not exist", &st);
    assert_eq!(game_query.items.len(), 0);
}

// Test get_game_by_year
#[test]
fn test_get_by_year_year_exists() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::CaseSensitive;
    let game_query = qr.clone().get_game_by_year("2011", &st);
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(
        game.name,
        "AaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome".to_string()
    );
    let st = SearchType::NotCaseSensitive;
    let game_query = qr.get_game_by_year("2011", &st);
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(
        game.name,
        "AaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome".to_string()
    );
}
#[test]
fn test_get_by_year_year_does_not_exist() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::CaseSensitive;
    let game_query = qr.clone().get_game_by_year("2811", &st);
    assert_eq!(game_query.items.len(), 0);
    let st = SearchType::NotCaseSensitive;
    let game_query = qr.get_game_by_year("2811", &st);
    assert_eq!(game_query.items.len(), 0);
}

// Test get_game_by_engine
#[test]
fn test_get_game_by_engine_engine_exists() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::CaseSensitive;
    let game_query = qr.clone().get_game_by_engine("godot", &st);
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "The Adventures of Mr. Hat".to_string());
    let st = SearchType::NotCaseSensitive;
    let game_query = qr.get_game_by_engine("godot", &st);
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "The Adventures of Mr. Hat".to_string());
}
#[test]
fn test_get_game_by_engine_engine_does_not_exist() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::CaseSensitive;
    let game_query = qr.clone().get_game_by_engine("I do not exist", &st);
    assert_eq!(game_query.items.len(), 0);
    let st = SearchType::NotCaseSensitive;
    let game_query = qr.get_game_by_engine("I do not exist", &st);
    assert_eq!(game_query.items.len(), 0);
}

// Test get_game_by_runtime
#[test]
fn test_get_game_by_runtime_runtime_exists() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::CaseSensitive;
    let game_query = qr.clone().get_game_by_runtime("lwjgl", &st);
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Airships: Conquer the Skies".to_string());
    let st = SearchType::NotCaseSensitive;
    let game_query = qr.get_game_by_runtime("lwjgl", &st);
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Airships: Conquer the Skies".to_string());
}
#[test]
fn test_get_game_by_runtime_runtime_does_not_exist() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::CaseSensitive;
    let game_query = qr.clone().get_game_by_runtime("I do not exist", &st);
    assert_eq!(game_query.items.len(), 0);
    let st = SearchType::NotCaseSensitive;
    let game_query = qr.get_game_by_runtime("I do not exist", &st);
    assert_eq!(game_query.items.len(), 0);
}

// Test get_game_by_genre
#[test]
fn test_get_game_by_genre_genre_exists() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::CaseSensitive;
    let game_query = qr.clone().get_game_by_genre("shmup", &st);
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Aeternum".to_string());
    let st = SearchType::NotCaseSensitive;
    let game_query = qr.get_game_by_genre("shmup", &st);
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Aeternum".to_string());
}
#[test]
fn test_get_game_by_genre_genre_exists_case_sensitive() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::CaseSensitive;
    let game_query = qr.get_game_by_genre("sHmup", &st);
    assert_eq!(game_query.items.len(), 0);
}
#[test]
fn test_get_game_by_genre_genre_exists_not_case_sensitive() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::NotCaseSensitive;
    let game_query = qr.get_game_by_genre("sHmup", &st);
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Aeternum".to_string());
}
#[test]
fn test_get_game_by_genre_genre_does_not_exist() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::CaseSensitive;
    let game_query = qr.clone().get_game_by_genre("I do not exist", &st);
    assert_eq!(game_query.items.len(), 0);
    let st = SearchType::CaseSensitive;
    let game_query = qr.get_game_by_genre("I do not exist", &st);
    assert_eq!(game_query.items.len(), 0);
}

// Test get_game_by_dev
#[test]
fn test_get_game_by_dev_dev_exists() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::CaseSensitive;
    let game_query = qr.clone().get_game_by_dev("Creaky Lantern Games", &st);
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Aeternum".to_string());
    let st = SearchType::NotCaseSensitive;
    let game_query = qr.get_game_by_dev("Creaky Lantern Games", &st);
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Aeternum".to_string());
}
#[test]
fn test_get_game_by_dev_dev_exists_case_sensitive() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::CaseSensitive;
    let game_query = qr.clone().get_game_by_dev("creaky Lantern Games", &st);
    assert_eq!(game_query.items.len(), 0);
}
#[test]
fn test_get_game_by_dev_dev_exists_not_case_sentitive() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::NotCaseSensitive;
    let game_query = qr.clone().get_game_by_dev("creaky Lantern Games", &st);
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Aeternum".to_string());
}
#[test]
fn test_get_game_by_dev_dev_does_not_exist() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::CaseSensitive;
    let game_query = qr.clone().get_game_by_dev("I do not exist", &st);
    assert_eq!(game_query.items.len(), 0);
    let st = SearchType::NotCaseSensitive;
    let game_query = qr.get_game_by_dev("I do not exist", &st);
    assert_eq!(game_query.items.len(), 0);
}

// Test get_game_by_publi
#[test]
fn test_get_game_by_publi_publi_exists() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::CaseSensitive;
    let game_query = qr.clone().get_game_by_publi("Florent Espanet", &st);
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Alien Shepherd".to_string());
    let st = SearchType::NotCaseSensitive;
    let game_query = qr.get_game_by_publi("Florent Espanet", &st);
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Alien Shepherd".to_string());
}
#[test]
fn test_get_game_by_publi_publi_exists_case_sensitive() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::CaseSensitive;
    let game_query = qr.get_game_by_publi("florent Espanet", &st);
    assert_eq!(game_query.items.len(), 0);
}
#[test]
fn test_get_game_by_publi_publi_exists_not_case_sensitive() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::NotCaseSensitive;
    let game_query = qr.get_game_by_publi("Florent Espanet", &st);
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Alien Shepherd".to_string());
}
#[test]
fn test_get_game_by_publi_publi_does_not_exist() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::CaseSensitive;
    let game_query = qr.get_game_by_publi("I do not exist", &st);
    assert_eq!(game_query.items.len(), 0);
}

// Test get_item_by_name
#[test]
fn test_get_item_by_name_name_exists() {
    let db = get_db_strict();
    let tag_query = db.get_all_devs();
    let item = tag_query.get_item_by_name("AX-GAME").unwrap();
    assert!(item.eq("AX-GAME"));
}
#[test]
fn test_get_item_by_name_name_does_not_exist() {
    let db = get_db_strict();
    let tag_query = db.get_all_devs();
    let item = tag_query.get_item_by_name("Does not exist");
    assert_eq!(item, None);
}

// Test get_game_by_runtime
#[test]
fn test_get_item_exists() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::CaseSensitive;
    let game_query = qr.clone().get_game_by_runtime("lwjgl", &st);
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.get(0).unwrap();
    assert_eq!(game.name, "Airships: Conquer the Skies".to_string());
    let st = SearchType::NotCaseSensitive;
    let game_query = qr.get_game_by_runtime("lwjgl", &st);
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.get(0).unwrap();
    assert_eq!(game.name, "Airships: Conquer the Skies".to_string());
}
#[test]
fn test_get_item_does_not_exist() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let st = SearchType::CaseSensitive;
    let game_query = qr.clone().get_game_by_runtime("I do not exist", &st);
    assert_eq!(game_query.get(0), None);
    let st = SearchType::NotCaseSensitive;
    let game_query = qr.get_game_by_runtime("I do not exist", &st);
    assert_eq!(game_query.get(0), None);
}

//-------------------
// SEARCH
//-------------------

// Test search_game_by_name
#[test]
fn test_search_game_by_name_game_exists() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let games = qr.search_game_by_name("Airships: Conquer the Skies");
    let games = games.into_inner();
    assert_eq!(games.len(), 1);
    assert_eq!(games[0].uid, 1595434339);
}
#[test]
fn test_search_game_by_name_game_exists_case_insensitive() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let games = qr.search_game_by_name("airships: conquer the skies");
    let games = games.into_inner();
    assert_eq!(games.len(), 1);
    assert_eq!(games[0].uid, 1595434339);
}
#[test]
fn test_search_game_by_name_name_does_not_exist() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let games = qr.search_game_by_name("I do not exist");
    let games = games.into_inner();
    assert!(games.is_empty());
}

// Test search_item_by_name
#[test]
fn test_search_item_by_name_name_exists() {
    let db = get_db_strict();
    let tag_query = db.get_all_engines();
    let item = tag_query.search_item_by_name("FNA").items[0];
    assert!(item.eq("FNA"));
}
#[test]
fn test_search_item_by_name_name_exists_case_sensitive() {
    let db = get_db_strict();
    let tag_query = db.get_all_engines();
    let item = tag_query.search_item_by_name("fna").items[0];
    assert!(item.eq("FNA"));
}
#[test]
fn test_search_item_by_name_name_does_not_exist() {
    let db = get_db_strict();
    let tag_query = db.get_all_engines();
    let items = tag_query.search_item_by_name("Does not exist").items;
    assert!(items.is_empty());
}
