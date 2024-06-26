//! Provides a representation of the query result returned when
//! interogating the [`crate::GameDataBase`]. [`QueryResult`] is itself queryable
//! and return another [`QueryResult`].
use crate::db::Item;
use crate::{Game, GameFilter, SearchType};

use paste::paste;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

macro_rules! filter_games_by {
    ($field:ident) => {
        paste! {
            /// Returns the games which field contains the given value.
            /// It can be case sensitive or insensitive depending on the
            /// [`SearchType`] variant.
            pub fn [<filter_games_by_ $field>](self, field: &str, search_type: &SearchType) -> QueryResult<&'a Game> {
                let items = GameFilter::default().[<set_ $field>](field).filter_games(self.items, search_type);
                QueryResult::new(items)
            }
        }
    };
}

/// Queryable representation of the result of a query of the [`crate::GameDataBase`].
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryResult<T> {
    /// Number of items in the query result.
    pub count: usize,
    /// Items returned by the query.
    pub items: Vec<T>,
}

impl<T: Ord> QueryResult<T> {
    /// Creates a new [`QueryResult`] from a vector of items.
    pub fn new(mut items: Vec<T>) -> Self {
        items.sort();
        Self {
            count: items.len(),
            items,
        }
    }
    /// Returns a reference to an element corresponding to the given index.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
    /// Returns the items of the [`QueryResult`] as a vector.
    pub fn into_inner(self) -> Vec<T> {
        self.items
    }
}

impl<'a> QueryResult<&'a Item> {
    /// Returns a reference to an item corresponding to the given name (case sensitive)
    pub fn get_item_by_name<'b>(self, name: &'b str) -> Option<&'a Item> {
        let mut items: Vec<&Item> = self.items.into_iter().filter(|a| a.eq(&name)).collect();
        items.pop()
    }
    /// Returns a [`QueryResult`] from items that matches the given name (case insensitive)
    pub fn filter_items_by_name<'b>(self, name: &'b str) -> QueryResult<&'a Item> {
        let items: Vec<&Item> = self
            .items
            .into_iter()
            .filter(|a| a.to_lowercase().contains(&name.to_lowercase()))
            .collect();
        QueryResult::new(items)
    }
}

impl<'a> QueryResult<&'a Game> {
    /// Returns the game associated with the given name.
    /// It can be case sensitive or insensitive depending on the
    /// [`SearchType`] variant.
    pub fn get_game_by_name(self, name: &str, search_type: &SearchType) -> Option<&'a Game> {
        let mut items = GameFilter::default()
            .set_name(name)
            .filter_games(self.items, search_type);
        items.sort();
        items.pop()
    }

    filter_games_by!(name);
    filter_games_by!(runtime);
    filter_games_by!(year);
    filter_games_by!(engine);
    filter_games_by!(dev);
    filter_games_by!(publi);
    filter_games_by!(genre);
    filter_games_by!(tag);
}

impl<T> IntoIterator for QueryResult<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

#[cfg(test)]
mod query_results_tests {
    use crate::QueryResult;
    #[test]
    fn test_new() {
        let v = vec!["item1".to_string(), "item2".to_string()];
        let v2 = v.clone();
        let qr = QueryResult::new(v);
        assert_eq!(qr.items, v2);
        assert_eq!(qr.count, 2);
    }
}
