extern crate libpobsd;
use libpobsd::db::GameDataBase;
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

// Test get_by_id method
#[test]
fn test_get_game_by_id_game_exists() {
    let db = get_db_strict();
    let game = db
        .get_game_by_id(1595434339)
        .expect("Game with id 1595434339 exists");
    assert_eq!(&game.name, "Airships: Conquer the Skies");
}
#[test]
fn test_get_game_by_id_game_does_not_exist() {
    let db = get_db_strict();
    let game = db.get_game_by_id(1);
    assert_eq!(game, None);
}

// Test get_by_name method
#[test]
fn test_get_game_by_name_game_exists() {
    let db = get_db_strict();
    let game = db
        .get_game_by_name("Airships: Conquer the Skies")
        .expect("Game with id 1595434339 exists");
    assert_eq!(game.uid, 1595434339);
}
#[test]
fn test_get_game_by_name_game_does_not_exist() {
    let db = get_db_strict();
    let game = db.get_game_by_name("I do not exist");
    assert_eq!(game, None);
}

// Test get_game_by_ids
#[test]
fn test_get_game_by_ids_with_existing_games() {
    let db = get_db_strict();
    let games = db.get_game_by_ids(vec![1595434339, 2316180984]);
    let games = games.into_inner();
    assert_eq!(games.len(), 2);
    assert_eq!(&games[0].name, "Airships: Conquer the Skies");
    assert_eq!(&games[1].name, "Alien Shepherd");
}
#[test]
fn test_get_game_by_ids_with_some_existing_games() {
    let db = get_db_strict();
    let games = db.get_game_by_ids(vec![1595434339, 1]);
    let games = games.into_inner();
    assert_eq!(games.len(), 1);
    assert_eq!(&games[0].name, "Airships: Conquer the Skies");
}
#[test]
fn test_get_game_by_ids_empty_vec() {
    let db = get_db_strict();
    let games = db.get_game_by_ids(vec![]);
    let games = games.into_inner();
    assert_eq!(games.len(), 0);
}

// Test get_game_by_steam_id
#[test]
fn test_get_by_steam_id_game_exists() {
    let db = get_db_strict();
    let game = db.get_game_by_steam_id(1869200);
    match game {
        Some(game) => assert_eq!(game.name, "The Adventures of Mr. Hat".to_string()),
        None => panic!(),
    }
}
#[test]
fn test_get_by_steam_id_game_does_not_exist() {
    let db = get_db_strict();
    let game = db.get_game_by_steam_id(0);
    match game {
        Some(_) => panic!(),
        None => assert_eq!(game, None),
    }
}

// Test get_game_by_tag
#[test]
fn test_get_game_by_tag_tag_exists() {
    let db = get_db_strict();
    let game_query = db.get_game_by_tag("indie");
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
    let game_query = db.get_game_by_tag("I do not exist");
    assert_eq!(game_query.items.len(), 0);
}

// Test get_game_by_year
#[test]
fn test_get_by_year_year_exists() {
    let db = get_db_strict();
    let game_query = db.get_game_by_year("2011");
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
    let game_query = db.get_game_by_year("2811");
    assert_eq!(game_query.items.len(), 0);
}

// Test get_game_by_engine
#[test]
fn test_get_game_by_engine_engine_exists() {
    let db = get_db_strict();
    let game_query = db.get_game_by_engine("godot");
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "The Adventures of Mr. Hat".to_string());
}
#[test]
fn test_get_game_by_engine_engine_does_not_exist() {
    let db = get_db_strict();
    let game_query = db.get_game_by_engine("I do not exist");
    assert_eq!(game_query.items.len(), 0);
}

// Test get_game_by_runtime
#[test]
fn test_get_game_by_runtime_runtime_exists() {
    let db = get_db_strict();
    let game_query = db.get_game_by_runtime("lwjgl");
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Airships: Conquer the Skies".to_string());
}
#[test]
fn test_get_game_by_runtime_runtime_does_not_exist() {
    let db = get_db_strict();
    let game_query = db.get_game_by_runtime("I do not exist");
    assert_eq!(game_query.items.len(), 0);
}

// Test get_game_by_genre
#[test]
fn test_get_game_by_genre_genre_exists() {
    let db = get_db_strict();
    let game_query = db.get_game_by_genre("shmup");
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Aeternum".to_string());
}
#[test]
fn test_get_game_by_genre_genre_does_not_exist() {
    let db = get_db_strict();
    let game_query = db.get_game_by_genre("I do not exist");
    assert_eq!(game_query.items.len(), 0);
}

// Test get_game_by_dev
#[test]
fn test_get_game_by_dev_dev_exists() {
    let db = get_db_strict();
    let game_query = db.get_game_by_dev("Creaky Lantern Games");
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Aeternum".to_string());
}
#[test]
fn test_get_game_by_dev_dev_does_not_exist() {
    let db = get_db_strict();
    let game_query = db.get_game_by_dev("I do not exist");
    assert_eq!(game_query.items.len(), 0);
}

// Test get_game_by_publi
#[test]
fn test_get_game_by_publi_publi_exists() {
    let db = get_db_strict();
    let game_query = db.get_game_by_publi("Florent Espanet");
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Alien Shepherd".to_string());
}
#[test]
fn test_get_game_by_publi_publi_does_not_exist() {
    let db = get_db_strict();
    let game_query = db.get_game_by_publi("I do not exist");
    assert_eq!(game_query.items.len(), 0);
}

// Test get_all_ methods
#[test]
fn test_get_all_games() {
    let db = get_db_strict();
    let games = db.get_all_games();
    assert_eq!(games.items.len(), 9);
}
#[test]
fn test_get_all_devs() {
    let db = get_db_strict();
    let tag_query = db.get_all_devs();
    assert_eq!(tag_query.items.len(), 7);
    for tag in vec![
        "AX-GAME",
        "Smudged Cat Games",
        "Creaky Lantern Games",
        "David Stark",
        "Haruneko Entertainment",
        "Florent Espanet",
        "Lea Espanet",
    ] {
        assert!(tag_query.items.contains(&&tag.to_string()));
    }
}
#[test]
fn test_get_all_engines() {
    let db = get_db_strict();
    let tag_query = db.get_all_engines();
    assert_eq!(tag_query.items.len(), 7);
    for tag in vec![
        "godot",
        "FNA",
        "RPG Maker",
        "lwjgl",
        "XNA",
        "HashLink",
        "NW.js",
    ] {
        assert!(tag_query.items.contains(&&tag.to_string()));
    }
}
#[test]
fn test_get_all_tags() {
    let db = get_db_strict();
    let tag_query = db.get_all_tags();
    assert_eq!(tag_query.items.len(), 5);
    for tag in vec!["indie", "free", "manga", "bullethell", "anime"] {
        assert!(tag_query.items.contains(&&tag.to_string()));
    }
}
#[test]
fn test_get_all_years() {
    let db = get_db_strict();
    let tag_query = db.get_all_years();
    assert_eq!(tag_query.items.len(), 6);
    for tag in vec!["2011", "2012", "2017", "2018", "2014", "2022"] {
        assert!(tag_query.items.contains(&&tag.to_string()));
    }
}
#[test]
fn test_get_all_genres() {
    let db = get_db_strict();
    let tag_query = db.get_all_genres();
    assert_eq!(tag_query.items.len(), 6);
    for genre in vec![
        "Puzzle Platformer",
        "RPG",
        "shmup",
        "RTS",
        "Platformer",
        "platformer",
    ] {
        assert!(tag_query.items.contains(&&genre.to_string()));
    }
}
#[test]
fn test_get_all_publis() {
    let db = get_db_strict();
    let tag_query = db.get_all_publis();
    assert_eq!(tag_query.items.len(), 3);
    for genre in vec!["Fun Quarter", "Creaky Lantern Games", "Florent Espanet"] {
        assert!(tag_query.items.contains(&&genre.to_string()));
    }
}
#[test]
fn test_get_all_runtimes() {
    let db = get_db_strict();
    let tag_query = db.get_all_runtimes();
    assert_eq!(tag_query.items.len(), 7);
    for runtime in vec![
        "godot",
        "easyrpg",
        "fnaify",
        "lwjgl",
        "HashLink",
        "HTML5",
        "HumblePlay",
    ] {
        assert!(tag_query.items.contains(&&runtime.to_string()));
    }
}

//-------------------
// SEARCH
//-------------------

// Test search_game_by_name
#[test]
fn test_search_game_by_name_game_exists() {
    let db = get_db_strict();
    let games = db.search_game_by_name("Airships: Conquer the Skies");
    let games = games.into_inner();
    assert_eq!(games.len(), 1);
    assert_eq!(games[0].uid, 1595434339);
}
#[test]
fn test_search_game_by_name_game_exists_case_insensitive() {
    let db = get_db_strict();
    let games = db.search_game_by_name("airships: conquer the skies");
    let games = games.into_inner();
    assert_eq!(games.len(), 1);
    assert_eq!(games[0].uid, 1595434339);
}
#[test]
fn test_search_game_by_name_name_does_not_exist() {
    let db = get_db_strict();
    let games = db.search_game_by_name("I do not exist");
    let games = games.into_inner();
    assert!(games.is_empty());
}

// Test search_game_by_dev
#[test]
fn test_search_game_by_dev_dev_exists() {
    let db = get_db_strict();
    let games = db.search_game_by_devs("David Stark");
    let games = games.into_inner();
    assert_eq!(games.len(), 1);
    assert_eq!(games[0].uid, 1595434339);
}
#[test]
fn test_search_game_by_dev_dev_exists_case_insensitive() {
    let db = get_db_strict();
    let games = db.search_game_by_devs("david stark");
    let games = games.into_inner();
    assert_eq!(games.len(), 1);
    assert_eq!(games[0].uid, 1595434339);
}
#[test]
fn test_search_game_by_dev_dev_does_not_exist() {
    let db = get_db_strict();
    let games = db.search_game_by_devs("I do not exist");
    let games = games.into_inner();
    assert!(games.is_empty());
}

// Test search_game_by_tags
#[test]
fn test_search_game_by_tags_tag_exists() {
    let db = get_db_strict();
    let games = db.search_game_by_tags("Indie");
    let games = games.into_inner();
    assert_eq!(games.len(), 3);
    assert_eq!(games[0].name, "The Adventures of Mr. Hat");
    assert_eq!(games[1].name, "The Adventures of Shuggy");
    assert_eq!(games[2].name, "Aeternum");
}
#[test]
fn test_search_game_by_tags_tag_exists_case_insensitive() {
    let db = get_db_strict();
    let games = db.search_game_by_tags("indie");
    let games = games.into_inner();
    assert_eq!(games.len(), 3);
    assert_eq!(games[0].name, "The Adventures of Mr. Hat");
    assert_eq!(games[1].name, "The Adventures of Shuggy");
    assert_eq!(games[2].name, "Aeternum");
}
#[test]
fn test_search_game_by_tags_tag_does_not_exist() {
    let db = get_db_strict();
    let games = db.search_game_by_tags("I do not exist");
    let games = games.into_inner();
    assert!(games.is_empty());
}

// Test search_game_by_genres
#[test]
fn test_search_game_by_genres_genre_exists() {
    let db = get_db_strict();
    let games = db.search_game_by_genres("RTS");
    let games = games.into_inner();
    assert_eq!(games.len(), 1);
    assert_eq!(games[0].name, "Airships: Conquer the Skies");
}
#[test]
fn test_search_game_by_genres_genre_exists_case_insensitive() {
    let db = get_db_strict();
    let games = db.search_game_by_genres("rts");
    let games = games.into_inner();
    assert_eq!(games.len(), 1);
    assert_eq!(games[0].name, "Airships: Conquer the Skies");
}
#[test]
fn test_search_game_by_genres_genre_does_not_exist() {
    let db = get_db_strict();
    let games = db.search_game_by_genres("I do not exist");
    let games = games.into_inner();
    assert!(games.is_empty());
}

// Test search_game_by_engine
#[test]
fn test_search_game_by_engine_engine_exists() {
    let db = get_db_strict();
    let games = db.search_game_by_engine("FNA");
    let games = games.into_inner();
    assert_eq!(games.len(), 2);
    assert_eq!(games[0].name, "The Adventures of Shuggy");
    assert_eq!(games[1].name, "Aeternum");
}
#[test]
fn test_search_game_by_engine_engine_exists_case_insenstive() {
    let db = get_db_strict();
    let games = db.search_game_by_engine("fna");
    let games = games.into_inner();
    assert_eq!(games.len(), 2);
    assert_eq!(games[0].name, "The Adventures of Shuggy");
    assert_eq!(games[1].name, "Aeternum");
}
#[test]
fn test_search_game_by_engine_engine_does_not_exist() {
    let db = get_db_strict();
    let games = db.search_game_by_engine("I do not exist");
    let games = games.into_inner();
    assert!(games.is_empty());
}
