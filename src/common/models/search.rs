use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SearchParameters {
    pub q: String,
    pub limit: Option<u16>,
    pub offset:Option<u16>,
    pub params: Option<HashMap<String, String>>
}

impl SearchParameters {
    pub fn with_offset(&mut self, value: u16) -> &Self {
        self.offset = Some(value);
        self
    }
    pub fn add_offset(&mut self, value: u16) -> &Self {
        self.offset = if let Some(cur) = self.offset {Some(cur + value)} else {Some(value)};
        self
    }
}

#[derive(Debug, Clone)]
pub struct SearchOffset {
    pub offset: u64,
    pub total: u64,
}