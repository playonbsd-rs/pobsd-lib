//! Provides a set of structs representing the PlayOnBSD database items.
//!
//! In particular, this module provides:
//! * A [`Game`] struct representing a game of the PlayOnBSD database
//! * A [`StoreLink`] struct representing each store link (store field in the
//! PlayOnBSD database). Multiple [`StoreLink`]s can be stored in a [`StoreLinks`]
//! collection.
//! * A [`GameStatus`] struct representing the status of a Gamei. The status
//! itself as a [`Status`] enum and the optional associated comment.
//!
pub(crate) mod field;
pub mod game;
pub mod game_status;
pub(crate) mod split_line;
pub mod store_links;

pub use self::game::Game;
pub use self::game_status::GameStatus;
pub use self::game_status::Status;
pub use self::store_links::Store;
pub use self::store_links::StoreLink;
pub use self::store_links::StoreLinks;
