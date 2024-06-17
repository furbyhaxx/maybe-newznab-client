use std::collections::HashMap;
use std::str::FromStr;
use std::vec::IntoIter;
use maybe_async::maybe_async;
use rss::Channel;
use crate::Client;
use crate::common::error::ModelError;
use crate::common::Function;
use crate::common::models::{RssItem, SearchOffset, SearchParameters};

#[derive(Debug)]
pub struct SearchResult {
    pub offset: SearchOffset,
    pub items: Vec<rss::Item>,
}

impl SearchResult {
    pub fn len(&self) -> usize {self.items.len()}

    // pub fn pages(&self) -> usize {
    //     self.offset.total / self.offset.
    // }
}

impl TryFrom<&str> for SearchResult {
    type Error = ModelError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let res = SearchResult::try_from(Channel::from_str(value)?);
        if let Ok(v) = res {
            Ok(v)
        } else {
            Err(res.unwrap_err().into())
        }
    }
}

impl IntoIterator for SearchResult {
    type Item = rss::Item;
    type IntoIter = IntoIter<rss::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub enum SearchResultItem {
    #[default]
    Generic,
    TvShow,
    Movie,
    Book,
    Audio,
    Audiobook,
    Software,
    Game,
}

impl SearchResultItem {
    pub fn is_generic(&self) -> bool {match self { SearchResultItem::Generic => {true}, _ => {false} }}
    pub fn is_tv_show(&self) -> bool {match self { SearchResultItem::TvShow => {true}, _ => {false} }}
    pub fn is_movie(&self) -> bool {match self { SearchResultItem::Movie => {true}, _ => {false} }}
    pub fn is_book(&self) -> bool {match self { SearchResultItem::Book => {true}, _ => {false} }}
    pub fn is_audio(&self) -> bool {match self { SearchResultItem::Audio => {true}, _ => {false} }}
    pub fn is_audiobook(&self) -> bool {match self { SearchResultItem::Audiobook => {true}, _ => {false} }}
    pub fn is_software(&self) -> bool {match self { SearchResultItem::Software => {true}, _ => {false} }}
    pub fn is_game(&self) -> bool {match self { SearchResultItem::Game => {true}, _ => {false} }}
}

impl TryFrom<rss::Channel> for SearchResult {
    type Error = ModelError;

    fn try_from(channel: Channel) -> Result<Self, Self::Error> {
        let res = channel.get_nn_ext();
        if let Some(mut offset) = channel.get_nn_ext() {
            Ok(
                Self {
                    offset,
                    items: channel.items,
                }
            )
        } else {
            Err(ModelError::RssExtensionError("Could not get Newznab Extension from RSS.".to_string()))
        }
    }
}

pub trait GetNewznabExtension<'a, T> {
    fn get_nn_ext(&'a self) -> Option<T>;
}

impl<'a> GetNewznabExtension<'a, SearchOffset> for rss::Channel {
    fn get_nn_ext(&'a self) -> Option<SearchOffset> {
        if let Some(namespace) = self.extensions.get("newznab") {
            if let Some(ext) = namespace.get("response") {
                if let Some(response) = ext.get(0) {
                    Some(
                        SearchOffset {
                            offset: response.attrs.get("offset").unwrap().parse().unwrap(),
                            total: response.attrs.get("total").unwrap().parse().unwrap(),
                        }
                    )
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<'a> GetNewznabExtension<'a, HashMap<&'a String, &'a String>> for rss::Item {
    fn get_nn_ext(&'a self) -> Option<HashMap<&'a String, &'a String>> {
        if let Some(namespace) = self.extensions.get("newznab") {
            if let Some(attrs) = namespace.get("attr") {
                Some(
                    attrs.iter().map(|x| {
                        (x.attrs.get("name").unwrap(), x.attrs.get("value").unwrap())
                    }).collect::<HashMap<_, _>>()
                )
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct ActiveSearchResult {
    pub search_parameters: SearchParameters,
    pub search_offset: SearchOffset,
    pub items: Vec<RssItem>,
}

#[cfg_attr(target_arch = "wasm32", maybe_async(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), maybe_async)]
impl ActiveSearchResult {
    #[maybe_async::maybe_async(AFIT)]
    pub async fn more(&mut self, client: &Client) {
        let amount = client.caps.limits.max();
        self.get_more(client, amount).await
    }

    #[maybe_async::maybe_async(AFIT)]
    pub async fn get_more(&mut self, client: &Client, offset: i32) {

        let total = (self.search_offset.total - self.search_offset.offset).min(offset as u64);
        let step_size = client.caps.limits.max().min(offset);

        for _ in (0..total).step_by(step_size as usize) {
            let left = (self.search_offset.total - self.search_offset.offset).min(step_size as u64);
            let step = step_size.min(left as i32) as u16;

            &self.search_parameters.add_offset(step);
            let next = client.search(Function::Search(self.search_parameters.clone())).await;
            if let Ok(new) = next {
                self.items.extend(new.items);
                self.search_offset = new.search_offset;
            }
        }
    }

    #[maybe_async::maybe_async(AFIT)]
    pub async fn all(&mut self, client: &Client) {
        let amount = client.caps.limits.max();
        for _ in (self.search_offset.offset..(self.search_offset.total-amount as u64)).step_by(amount as usize) {
            self.get_more(client, (self.search_offset.total - self.search_offset.offset).min(amount as u64) as i32).await;
        }
    }
}