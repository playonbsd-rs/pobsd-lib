//! Provides a representations for the store links associated to each game.
use std::fmt::Display;

use regex::Regex;

/// Represents the store in which the game is available
#[derive(Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub enum Store {
    Steam,
    Gog,
    HumbleBundle,
    ItchIo,
    Epic,
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
    /// Id of the game for the store
    pub id: Option<usize>,
}

impl StoreLink {
    /// Create a StoreLink given an url.
    /// At the moment, it only handles Gog and Steam games.
    pub fn from(url: &str) -> Self {
        if url.contains("steampowered") {
            Self {
                store: Store::Steam,
                url: url.to_string(),
                id: get_steam_id(url),
            }
        } else if url.contains("gog.com") {
            Self {
                store: Store::Gog,
                url: url.to_string(),
                id: None,
            }
        } else if url.contains("humblebundle.com") {
            Self {
                store: Store::HumbleBundle,
                url: url.to_string(),
                id: None,
            }
        } else if url.contains("itch.io") {
            Self {
                store: Store::ItchIo,
                url: url.to_string(),
                id: None,
            }
        } else if url.contains("epicgames.com") {
            Self {
                store: Store::Epic,
                url: url.to_string(),
                id: None,
            }
        } else {
            Self {
                store: Store::Unknown,
                url: url.to_string(),
                id: None,
            }
        }
    }
}

impl Display for StoreLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}

// Return the steam id from a store url
fn get_steam_id(url: &str) -> Option<usize> {
    let re = Regex::new(r"https://store.steampowered.com/app/(\d+)(/?.+)?").unwrap();
    if let Some(cap) = re.captures(url) {
        if let Some(cap) = cap.get(1) {
            return cap.as_str().parse::<usize>().ok();
        };
    };
    None
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
    /// Return true if a Steam game is present, false otherwise
    pub fn has_steam(&self) -> bool {
        let links = self.inner_ref();
        let res: Vec<&StoreLink> = links.iter().filter(|a| a.store.eq(&Store::Steam)).collect();
        !res.is_empty()
    }
    /// Return true if a Steam game is present, false otherwise
    pub fn has_gog(&self) -> bool {
        let links = self.inner_ref();
        let res: Vec<&StoreLink> = links.iter().filter(|a| a.store.eq(&Store::Gog)).collect();
        !res.is_empty()
    }
}

impl Display for StoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

#[cfg(test)]
mod store_link_tests {
    use super::*;
    // get_steam_id
    #[test]
    fn test_get_steam_id_with_steam_url() {
        let id = get_steam_id("https://store.steampowered.com/app/1878910/LoupLaine/");
        assert_eq!(id, Some(1878910));
    }
    #[test]
    fn test_get_steam_id_with_non_steam_url() {
        let id = get_steam_id("https://humblebundle.com/app/1878910/LoupLaine/");
        assert_eq!(id, None);
    }
    // StoreLink
    #[test]
    fn test_store_link_from_steam_url() {
        let store = StoreLink::from("https://store.steampowered.com/app/1878910/LoupLaine/");
        assert_eq!(store.id, Some(1878910));
        assert_eq!(store.store, Store::Steam);

        let store = StoreLink::from("https://store.steampowered.com/app/1878910");
        assert_eq!(store.id, Some(1878910));
        assert_eq!(store.store, Store::Steam);

        let store = StoreLink::from("https://store.steampowered.com/app/1878910/");
        assert_eq!(store.id, Some(1878910));
        assert_eq!(store.store, Store::Steam);

        let store = StoreLink::from("https://store.steampowered.com/app/1878910/LoupLaine");
        assert_eq!(store.id, Some(1878910));
        assert_eq!(store.store, Store::Steam);
    }
    #[test]
    fn test_store_link_from_gog_url() {
        let store = StoreLink::from("https://gog.com/app/1878910/LoupLaine/");
        assert_eq!(store.id, None);
        assert_eq!(store.store, Store::Gog);
    }
    #[test]
    fn test_store_link_from_humblebundle_url() {
        let store = StoreLink::from("https://humblebundle.com/app/1878910/LoupLaine/");
        assert_eq!(store.id, None);
        assert_eq!(store.store, Store::HumbleBundle);
    }
    #[test]
    fn test_store_link_from_itchio_url() {
        let store = StoreLink::from("https://plug-in-digital.itch.io/dead-cells");
        assert_eq!(store.id, None);
        assert_eq!(store.store, Store::ItchIo);
    }
    #[test]
    fn test_store_link_from_epic_url() {
        let store =
            StoreLink::from("https://www.epicgames.com/store/en-US/product/axiom-verge/home");
        assert_eq!(store.id, None);
        assert_eq!(store.store, Store::Epic);
    }
    #[test]
    fn test_store_link_from_unknown_url() {
        let store = StoreLink::from("https://unknown.com/app/1878910/LoupLaine/");
        assert_eq!(store.id, None);
        assert_eq!(store.store, Store::Unknown);
    }
    #[test]
    fn test_store_link_display() {
        let store = StoreLink::from("https://unknown.com/app/1878910/LoupLaine/");
        assert_eq!(
            format!("{}", store),
            String::from("https://unknown.com/app/1878910/LoupLaine/")
        );
    }
    // StoreLinks
    #[test]
    fn test_store_links_new_method() {
        let v: Vec<StoreLink> = vec![];
        let v2 = v.clone();
        let st = StoreLinks::new(v);
        assert_eq!(st, StoreLinks(v2));
    }
    #[test]
    fn test_store_links_push_method() {
        let v: Vec<StoreLink> = vec![];
        let store = StoreLink::from("https://humblebundle.com/app/1878910/LoupLaine/");
        let mut st = StoreLinks::new(v);
        st.push(store);
        assert_eq!(
            st.inner_ref()[0].url,
            "https://humblebundle.com/app/1878910/LoupLaine/"
        );
    }
    #[test]
    fn test_store_links_into_inner_method() {
        let v: Vec<StoreLink> = vec![];
        let v2 = v.clone();
        let st = StoreLinks::new(v);
        assert_eq!(st.into_inner(), v2);
    }
    #[test]
    fn test_store_links_inner_ref_method() {
        let store = StoreLink::from("https://humblebundle.com/app/1878910/LoupLaine/");
        let v: Vec<StoreLink> = vec![store];
        let st = StoreLinks::new(v);
        let v: &Vec<StoreLink> = st.inner_ref();
        assert_eq!(v[0].url, "https://humblebundle.com/app/1878910/LoupLaine/");
    }
    #[test]
    fn test_store_links_inner_mut_ref_method() {
        let store = StoreLink::from("https://humblebundle.com/app/1878910/LoupLaine/");
        let v: Vec<StoreLink> = vec![store];
        let mut st = StoreLinks::new(v);
        let v: &mut Vec<StoreLink> = st.inner_mut_ref();
        assert_eq!(v[0].url, "https://humblebundle.com/app/1878910/LoupLaine/");
    }
    #[test]
    fn test_store_links_has_steam_method() {
        let v: Vec<StoreLink> = vec![];
        let store = StoreLink::from("https://humblebundle.com/app/1878910/LoupLaine/");
        let mut st = StoreLinks::new(v);
        st.push(store);
        assert!(!st.has_steam());
        let store = StoreLink::from("https://store.steampowered.com/app/1878910/LoupLaine");
        st.push(store);
        assert!(st.has_steam());
    }
    #[test]
    fn test_store_links_has_gog_method() {
        let v: Vec<StoreLink> = vec![];
        let store = StoreLink::from("https://humblebundle.com/app/1878910/LoupLaine/");
        let mut st = StoreLinks::new(v);
        st.push(store);
        assert!(!st.has_gog());
        let store = StoreLink::from("https://gog.com/app/1878910/LoupLaine/");
        st.push(store);
        assert!(st.has_gog());
    }
    #[test]
    fn test_store_links_display() {
        let v: Vec<StoreLink> = vec![];
        let store = StoreLink::from("https://humblebundle.com/app/1878910/LoupLaine/");
        let mut st = StoreLinks::new(v);
        st.push(store);
        let store = StoreLink::from("https://gog.com/app/1878910/LoupLaine/");
        st.push(store);
        assert_eq!(format!("{}", st), String::from("https://humblebundle.com/app/1878910/LoupLaine/ https://gog.com/app/1878910/LoupLaine/"));
    }
}
