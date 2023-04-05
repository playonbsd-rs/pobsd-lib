extern crate libpobsd;
use libpobsd::db::GameDataBase;
use libpobsd::parser::{Game, Parser, ParserResult, ParsingMode};

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

#[test]
fn test_get_game_by_id() {
    let db = get_db_strict();
    let game = db
        .get_game_by_id(1595434339)
        .expect("Game with id 1595434339 exists");
    assert_eq!(&game.name, "Airships: Conquer the Skies");
}

#[test]
fn test_get_game_by_name() {
    let db = get_db_strict();
    let game = db
        .get_game_by_name("Airships: Conquer the Skies")
        .expect("Game with id 1595434339 exists");
    assert_eq!(game.uid, 1595434339);
    let game = db.get_game_by_name("I do not exist");
    assert_eq!(game, None);
}

#[test]
fn test_query_get_game_by_name() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let game = qr
        .get_game_by_name("Airships: Conquer the Skies")
        .expect("Game with id 1595434339 exists");
    assert_eq!(game.uid, 1595434339);
    let game = db.get_game_by_name("I do not exist");
    assert_eq!(game, None);
}

#[test]
fn test_get_game_by_ids() {
    let db = get_db_strict();
    let games = db.get_game_by_ids(vec![1595434339, 2316180984]);
    let games = games.into_inner();
    assert_eq!(&games[0].name, "Airships: Conquer the Skies");
    assert_eq!(&games[1].name, "Alien Shepherd");
}
#[test]
fn test_get_by_steam_id() {
    let db = get_db_strict();
    let game = db.get_game_by_steam_id(1869200);
    match game {
        Some(game) => assert_eq!(game.name, "The Adventures of Mr. Hat".to_string()),
        None => panic!(),
    }
    let game = db.get_game_by_steam_id(0);
    match game {
        Some(_) => panic!(),
        None => assert_eq!(game, None),
    }
}

#[test]
fn test_get_by_tag() {
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
fn test_query_result_get_by_tag() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let game_query = qr.get_game_by_tags("indie");
    assert_eq!(game_query.items.len(), 3);
    let game: &Game = game_query.get(0).unwrap();
    assert_eq!(game.name, "The Adventures of Mr. Hat".to_string());
}

#[test]
fn test_query_result_get() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let game_query = qr.get_game_by_tags("indie");
    let game = game_query.get(1).unwrap();
    assert_eq!(game.name, "The Adventures of Shuggy".to_string());
}

#[test]
fn test_query_result_into_inner() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let game_query = qr.get_game_by_tags("indie");
    let games = game_query.into_inner();
    let game = games.get(2).unwrap();
    assert_eq!(game.name, "Aeternum".to_string());
}

#[test]
fn test_query_result_get_by_tag_empty() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let game_query = qr.get_game_by_tags("notatag");
    assert_eq!(game_query.items.len(), 0);
}

#[test]
fn test_get_by_year() {
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
fn test_query_result_get_by_year() {
    let db = get_db_strict();
    let qr = db.get_all_games();
    let game_query = qr.get_game_by_year("2011");
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(
        game.name,
        "AaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome".to_string()
    );
}

#[test]
fn test_get_by_engine() {
    let db = get_db_strict();
    let game_query = db.get_game_by_engine("godot");
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "The Adventures of Mr. Hat".to_string());
}

#[test]
fn test_get_by_runtime() {
    let db = get_db_strict();
    let game_query = db.get_game_by_runtime("lwjgl");
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Airships: Conquer the Skies".to_string());
}

#[test]
fn test_get_by_genre() {
    let db = get_db_strict();
    let game_query = db.get_game_by_genre("shmup");
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Aeternum".to_string());
}

#[test]
fn test_get_by_dev() {
    let db = get_db_strict();
    let game_query = db.get_game_by_dev("Creaky Lantern Games");
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Aeternum".to_string());
}

#[test]
fn test_get_by_publi() {
    let db = get_db_strict();
    let game_query = db.get_game_by_publi("Florent Espanet");
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Alien Shepherd".to_string());
}

#[test]
fn test_get_all_games() {
    let db = get_db_strict();
    let games = db.get_all_games();
    assert_eq!(games.items.len(), 9);
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

#[test]
fn test_search_game_by_name() {
    let db = get_db_strict();
    let games = db.search_game_by_name("Airships: Conquer the Skies");
    let games = games.into_inner();
    assert_eq!(games[0].uid, 1595434339);
    let db = get_db_strict();
    let games = db.search_game_by_name("airships: conquer the skies");
    let games = games.into_inner();
    assert_eq!(games[0].uid, 1595434339);
    let games = db.search_game_by_name("I do not exist");
    let games = games.into_inner();
    assert!(games.is_empty());
}
#[test]
fn test_search_game_by_tags() {
    let db = get_db_strict();
    let games = db.search_game_by_tags("Indie");
    let games = games.into_inner();
    assert_eq!(games[0].name, "The Adventures of Mr. Hat");
    assert_eq!(games[1].name, "The Adventures of Shuggy");
    let db = get_db_strict();
    let games = db.search_game_by_tags("indie");
    let games = games.into_inner();
    assert_eq!(games[0].name, "The Adventures of Mr. Hat");
    assert_eq!(games[1].name, "The Adventures of Shuggy");
    let games = db.search_game_by_name("I do not exist");
    let games = games.into_inner();
    assert!(games.is_empty());
}
#[test]
fn test_search_game_by_engine() {
    let db = get_db_strict();
    let games = db.search_game_by_engine("FNA");
    let games = games.into_inner();
    assert_eq!(games[0].name, "The Adventures of Shuggy");
    assert_eq!(games[1].name, "Aeternum");
    let db = get_db_strict();
    let games = db.search_game_by_engine("fna");
    let games = games.into_inner();
    assert_eq!(games[0].name, "The Adventures of Shuggy");
    assert_eq!(games[1].name, "Aeternum");
    let games = db.search_game_by_engine("I do not exist");
    let games = games.into_inner();
    assert!(games.is_empty());
}
