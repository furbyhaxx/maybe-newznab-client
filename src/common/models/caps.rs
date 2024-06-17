use serde::{Deserialize, Deserializer, Serialize};



#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
pub struct Caps {
    pub server: Server,
    pub limits: Limits,
    pub retention: Retention,
    pub searching: Searching,

    // #[serde(rename = "category")]
    // #[serde(rename = "$value")]
    // categories: Vec<Category>,
    pub categories: Categories,
    // categories: Categories,
}

#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
pub struct Server {
    title: String,
    email: String,
    url: String,
    image: String,
}

#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
pub struct Limits {
    pub(crate) max: i32,
    pub(crate) default: i32,
}

impl Limits {
    pub fn max(&self) -> i32 {self.max}
    pub fn default(&self) -> i32 {self.default}
}

#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
pub struct Retention {
    days: u32,
}
// error: unknown rename rule `rename_all = "$value"`, expected one of "lowercase", "UPPERCASE", "PascalCase", "camelCase", "snake_case", "SCREAMING_SNAKE_CASE", "kebab-case", "SCREAMING-KEBAB-CASE"

#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Searching {
    search: Option<Search>,
    tv_search: Option<Search>,
    movie_search: Option<Search>,
    audio_search: Option<Search>,
    book_search: Option<Search>,
}

#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
pub struct Search {
    #[serde(deserialize_with = "bool_from_yes_no")]
    available: bool,
    #[serde(rename = "supportedParams")]
    supported_params: Option<String>,
}

#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
pub struct Categories {
    #[serde(rename = "category")]
    categories: Vec<Category>,
}

#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
pub struct Category {
    id: String,
    name: String,
    #[serde(rename = "subcat")]
    sub_categories: Option<Vec<SubCategory>>,
}

#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
pub struct SubCategory {
    id: String,
    name: String,
}

impl TryFrom<String> for Caps {
    type Error = serde_xml_rs::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let x = serde_xml_rs::from_str::<Self>(&value);
        x
    }
}

/// Helper function for serde
/// Deserializes a yes/no string field into a bool
fn bool_from_yes_no<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
{
    use serde::de::Error;
    use serde::Deserialize;

    String::deserialize(deserializer)
        .and_then(|string| if string == "yes" {Ok(true)} else {Ok(false)} )
}

#[cfg(test)]
mod tests {
    use crate::common::models::caps::{Caps, Categories, Category, Limits, Retention, Search, Searching, Server, SubCategory};

    #[test]
    fn deserialize_caps() {
        let input = r#"
        <caps>
            <server title="NZBHydra 2" email="theotherp@posteo.net" url="https://github.com/theotherp/nzbhydra2" image="https://raw.githubusercontent.com/theotherp/nzbhydra2/master/core/ui-src/img/banner-bright.png"/>
            <limits max="100" default="100"/>
            <retention days="3000"/>
            <searching>
                <search available="yes" supportedParams="q,cat,limit,offset,minage,maxage,minsize,maxsize"/>
                <tv-search available="yes" supportedParams="q,season,ep,cat,limit,offset,minage,maxage,minsize,maxsize,rid,tvdbid,tvmazeid,imdbid,traktid"/>
                <movie-search available="yes" supportedParams="q,cat,limit,offset,minage,maxage,minsize,maxsize,imdbid,tmdbid"/>
                <audio-search available="no" supportedParams=""/>
                <book-search available="yes" supportedParams="q,author,title,cat,limit,offset,minage,maxage,minsize,maxsize"/>
            </searching>
            <categories>
                <category id="1000" name="Console"/>
                <category id="2000" name="Movies">
                    <subcat id="2040" name="Movies HD"/>
                    <subcat id="2030" name="Movies SD"/>
                </category>
            </categories>
        </caps>
        "#;

        let expected = Caps {
            server: Server {
                title: "NZBHydra 2".to_string(),
                email: "theotherp@posteo.net".to_string(),
                url: "https://github.com/theotherp/nzbhydra2".to_string(),
                image: "https://raw.githubusercontent.com/theotherp/nzbhydra2/master/core/ui-src/img/banner-bright.png".to_string(),
            },
            limits: Limits {
                max: 100,
                default: 100,
            },
            retention: Retention {
                days: 3000,
            },
            searching: Searching {
                search: Search { available: true, supported_params: Some("q,cat,limit,offset,minage,maxage,minsize,maxsize".to_string()) }.into(),
                tv_search: Search { available: true, supported_params: Some("q,season,ep,cat,limit,offset,minage,maxage,minsize,maxsize,rid,tvdbid,tvmazeid,imdbid,traktid".to_string()) }.into(),
                movie_search: Search { available: true, supported_params: Some("q,cat,limit,offset,minage,maxage,minsize,maxsize,imdbid,tmdbid".to_string()) }.into(),
                audio_search: Search { available: false, supported_params: Some("".to_string()) }.into(),
                book_search: Search { available: true, supported_params: Some("q,author,title,cat,limit,offset,minage,maxage,minsize,maxsize".to_string()) }.into(),
            },
            categories: Categories {
                categories: vec![
                    Category {
                        id: "1000".to_string(),
                        name: "Console".to_string(),
                        sub_categories: None,
                    },
                    Category {
                        id: "2000".to_string(),
                        name: "Movies".to_string(),
                        sub_categories: Some(vec![
                            SubCategory { id: "2040".to_string(), name: "Movies HD".to_string() },
                            SubCategory { id: "2030".to_string(), name: "Movies SD".to_string() },
                        ]),
                    }
                ],
            },
        };

        let deserialized = serde_xml_rs::from_str::<Caps>(&input).unwrap();

        assert_eq!(deserialized, expected);
    }
}