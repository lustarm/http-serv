use std::collections::HashMap;

pub enum HttpType {
    // None for when we don't
    // know http type...
    // if http_type == HttpType::NONE
    NONE,
    GET,
    POST,
}

// Just easier to read later on
type HttpHeader = HashMap<String, String>;

pub struct HttpPayload {
    headers: HttpHeader,
    http_type: HttpType,
}

impl HttpPayload {
    pub fn new() -> Self {
        HttpPayload {
            // Create new hashmap
            headers: HashMap::new(),

            // Default to none
            http_type: HttpType::NONE,
        }
    }

    pub fn set_type(&mut self, http_type: HttpType) {
        self.http_type = http_type;
    }

    // Self explainitory
    pub fn add_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn get_header(&self, key: String) -> Option<&String> {
        self.headers.get(key.as_str())
    }
}
