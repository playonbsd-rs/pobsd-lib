//! Provides a representations for the store links associated to each game.
use regex::Regex;

/// Represents the store in which the game is available
#[derive(Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub enum Store {
    /// For steam games
    Steam,
    /// For Gog games
    Gog,
    /// For games on other stores
    #[default]
    Unknown,
}

/// Represent a store link
#[derive(Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct StoreLink {
    /// Store related to the link
    pub store: Store,
    /// Link where the game can be found
    pub url: String,
}

impl StoreLink {
    /// Create a StoreLink given an url.
    /// At the moment, it only handles Gog and Steam games.
    pub fn from(url: &str) -> Self {
        if url.contains("steampowered") {
            Self {
                store: Store::Steam,
                url: url.to_string(),
            }
        } else if url.contains("gog.com") {
            Self {
                store: Store::Gog,
                url: url.to_string(),
            }
        } else {
            Self {
                store: Store::Unknown,
                url: url.to_string(),
            }
        }
    }
    /// Return the id of the game for a given store.
    /// Only works for Steam at the moment and return
    /// None for other stores.
    pub fn get_id(&self) -> Option<usize> {
        let re = Regex::new(r"https://store.steampowered.com/app/(\d+)(/?.+)?").unwrap();
        match &self.store {
            Store::Steam => {
                let cap = re.captures(&self.url).unwrap();
                if let Some(cap) = cap.get(1) {
                    return cap.as_str().parse::<usize>().ok();
                };
                None
            }
            _ => None,
        }
    }
}

/// Represent a collection of store links
#[derive(Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct StoreLinks(pub Vec<StoreLink>);

impl StoreLinks {
    /// Create a StoreLinks with the given items
    pub fn new(items: Vec<StoreLink>) -> Self {
        Self(items)
    }
    /// Add a StoreLink to the collection
    pub fn push(&mut self, store: StoreLink) {
        self.0.push(store)
    }
    /// Return a imutable reference to the vector of StoreLink.
    pub fn inner_ref(&self) -> &Vec<StoreLink> {
        &self.0
    }
    /// Return a mutable reference to the vector of StoreLink.
    pub fn inner_mut_ref(&mut self) -> &mut Vec<StoreLink> {
        &mut self.0
    }
    /// Return the vector of StoreLink.
    pub fn into_inner(self) -> Vec<StoreLink> {
        self.0
    }
}

#[cfg(test)]
mod store_link_tests {
    use super::*;
    #[test]
    fn test_get_id_steam() {
        let store = StoreLink {
            store: Store::Steam,
            url: "https://store.steampowered.com/app/1878910/LoupLaine/".to_string(),
        };
        assert_eq!(store.get_id(), Some(1878910));

        let store = StoreLink {
            store: Store::Steam,
            url: "https://store.steampowered.com/app/1878910".to_string(),
        };
        assert_eq!(store.get_id(), Some(1878910));

        let store = StoreLink {
            store: Store::Steam,
            url: "https://store.steampowered.com/app/1878910/".to_string(),
        };
        assert_eq!(store.get_id(), Some(1878910));

        let store = StoreLink {
            store: Store::Steam,
            url: "https://store.steampowered.com/app/1878910/LoupLaine".to_string(),
        };
        assert_eq!(store.get_id(), Some(1878910));
    }
    #[test]
    fn test_get_id_gog() {
        let store = StoreLink {
            store: Store::Gog,
            url: "https://store.steampowered.com/app/1878910/LoupLaine/".to_string(),
        };
        assert_eq!(store.get_id(), None);
    }
}
