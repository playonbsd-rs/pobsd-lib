//! Provides a representation of the query result returned when
//! interogating the database. Query results are themselves queryable
//! and return another query result.
use super::Item;
use crate::db::{game_filer::GameFilter, SearchType};
use crate::parser::Game;
use paste::paste;

macro_rules! get_game_by {
    ($field:ident) => {
        paste! {
            /// Get game by field name
            pub fn [<filter_games_by_ $field>](self, field: &str, search_type: &SearchType) -> QueryResult<&'a Game> {

                let mut items = GameFilter::default().[<set_ $field>](field).filter_games(self.items, search_type);
                items.sort();
                QueryResult{
                    count: items.len(),
                    items
                }
            }
        }
    };
}

/// Queryable representation of the result of a query
#[derive(Default, Debug, Clone)]
pub struct QueryResult<T> {
    /// Number of items in the query result
    pub count: usize,
    /// Vector of items
    pub items: Vec<T>,
}

impl<T> QueryResult<T> {
    // Create a new QueryResult from a vector
    pub fn new(items: Vec<T>) -> Self {
        Self {
            count: items.len(),
            items,
        }
    }
    // Returns a reference to an element or subslice depending on the type of index
    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
    // Return the vector of items stored in the query result
    pub fn into_inner(self) -> Vec<T> {
        self.items
    }
}

impl<'a> QueryResult<&'a Item> {
    /// Get item by name (case sensitive)
    pub fn get_item_by_name<'b>(self, name: &'b str) -> Option<&'a Item> {
        let mut items: Vec<&Item> = self.items.into_iter().filter(|a| a.eq(&name)).collect();
        items.pop()
    }
    /// Search items by name (case insensitive)
    pub fn filter_items_by_name<'b>(self, name: &'b str) -> QueryResult<&'a Item> {
        let items: Vec<&Item> = self
            .items
            .into_iter()
            .filter(|a| a.to_lowercase().contains(&name.to_lowercase()))
            .collect();
        QueryResult {
            count: items.len(),
            items,
        }
    }
}

impl<'a> QueryResult<&'a Game> {
    /// Get game by name (case sensitive)
    pub fn get_game_by_name(self, name: &str) -> Option<&'a Game> {
        let mut items: Vec<&Game> = self.items.into_iter().filter(|a| a.name.eq(name)).collect();
        items.pop()
    }
    /// Search games by name (case insensitive)
    pub fn filter_games_by_name(self, name: &str) -> QueryResult<&'a Game> {
        let items: Vec<&Game> = self
            .items
            .into_iter()
            .filter(|a| a.name.to_lowercase().contains(&name.to_lowercase()))
            .collect();
        QueryResult {
            count: items.len(),
            items,
        }
    }
    get_game_by!(runtime);
    get_game_by!(year);
    get_game_by!(engine);
    get_game_by!(dev);
    get_game_by!(publi);
    get_game_by!(genre);
    get_game_by!(tag);
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
