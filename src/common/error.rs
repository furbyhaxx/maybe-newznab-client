use thiserror::Error;

pub type XmlError = serde_xml_rs::Error;
pub type JsonError = serde_json::Error;
pub type RssError = rss::Error;

#[derive(Debug, Error)]
#[error("ModelError: {0}")]
pub enum ModelError {
    #[error("XmlParseError: {0}")]
    XmlParseError(#[from] XmlError),

    #[error("XmlParseError: {0}")]
    JsonParseError(#[from] JsonError),

    #[error("XmlParseError: {0}")]
    RssParseError(#[from] RssError),

    #[error("XmlParseError: {0}")]
    RssExtensionError(String),
}