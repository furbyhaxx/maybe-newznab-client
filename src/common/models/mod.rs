mod error;
mod caps;
mod search_result;
mod search;

use std::collections::HashMap;
use std::fmt::Display;
use serde::Deserialize;

pub use error::{
    NewznabError,
};
pub(crate) use error::NewznabRawError;

pub use self::{
    caps::*,
    search::*,
    search_result::*,
};

pub type RssItem = rss::Item;

#[derive(Debug, Default, Deserialize)]
pub enum Format {
    #[default]
    Xml,
    Json,
}


#[derive(Debug, Clone)]
pub enum Function {
    Caps,
    Register{ email: String },
    Search (SearchParameters)
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = {
            match self {
                Function::Caps => {"caps"}
                Function::Register{..} => {"register"}
                Function::Search { .. } => { "search" }
            }
        };
        write!(f, "{}", repr)
    }
}

impl Function {
    pub fn unwrap_search(&self) -> SearchParameters {
        match self {
            Self::Search(params) => params.clone(),
            _ => {panic!("Called unwrap_search on a non Search Function")}
        }
    }
}
