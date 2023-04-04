//! Provides a representation of the query result returned when
//! interogating the database. Query results are themselves queriable
//! and return another query result.
use super::Item;
use crate::parser::Game;
use paste::paste;

macro_rules! get_game_by {
    ($field:ident) => {
        paste! {
            /// Get game by field name (case sensitive)
            pub fn [<get_game_by_ $field>](self, field: &str) -> QueryResult<&'a Game> {
                let mut items: Vec<&Game> = self
                    .items
                    .clone()
                    .into_iter()
                    .filter(|a| a.$field.eq(&Some(field.to_string())))
                    .collect();
                items.sort();
                QueryResult{
                    count: items.len(),
                    items
                }
            }
        }
    };
    (array $field:ident) => {
        paste! {
            /// Get game by field name (case sensitive)
            pub fn [<get_game_by_ $field>](self, field: &str) -> QueryResult<&'a Game> {
                let mut items: Vec<&Game> = self
                    .items
                    .clone()
                    .into_iter()
                    .filter(|a| match &a.$field {
                        Some(items) => items.contains(&field.to_string()),
                        None => false,
                    })
                    .collect();
                items.sort();
                QueryResult{
                    count: items.len(),
                    items
                }
            }
        }
    };
}

/// Representation of the result of a query
pub struct QueryResult<T> {
    /// Number of items in the query result
    pub count: usize,
    /// Vector of items
    pub items: Vec<T>,
}

impl<T> QueryResult<T> {
    // Returns a reference to an element or subslice depending on the type of index
    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
    // Returns a mutable reference to an element or subslice depending on the type of index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.items.get_mut(index)
    }
    // Return the vector of items stored in the query result
    pub fn into_inner(self) -> Vec<T> {
        self.items
    }
}

impl QueryResult<Item> {
    /// Get item by name (case sensitive)
    pub fn get_item_by_name(&self, name: &str) -> Option<Item> {
        let mut items: Vec<&Item> = self.items.iter().filter(|a| a.eq(&name)).collect();
        items.pop().cloned()
    }
    /// Search items by name (case insensitive)
    pub fn search_item_by_name(self, name: &str) -> QueryResult<Item> {
        let items: Vec<Item> = self
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
    pub fn search_game_by_name(self, name: &str) -> QueryResult<&'a Game> {
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
    get_game_by!(dev);
    get_game_by!(publi);
    get_game_by!(engine);
    get_game_by!(array genres);
    get_game_by!(array tags);
}
