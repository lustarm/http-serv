use std::collections::HashMap;

#[derive(PartialEq)]
pub enum HttpType {
    // None for when we don't
    // know http type...
    // if http_type == HttpType::NONE
    NONE,
    GET,
    _POST,
}

pub enum HttpStatus {
    Ok,
    NotFound,
    InternalServerError,
    // Add more status codes as needed
}

impl HttpStatus {
    fn code(&self) -> u16 {
        match self {
            HttpStatus::Ok => 200,
            HttpStatus::NotFound => 404,
            HttpStatus::InternalServerError => 500,
            // Handle other status codes
        }
    }

    fn message(&self) -> &'static str {
        match self {
            HttpStatus::Ok => "OK",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::InternalServerError => "Internal Server Error",
            // Handle other status messages
        }
    }
}

// Just easier to read later on
type HttpHeaders = HashMap<String, String>;

pub struct HttpPayload {
    headers: HttpHeaders,
    http_type: HttpType,
    path: String,
}

impl HttpPayload {
    pub fn new() -> Self {
        HttpPayload {
            // Create new hashmap
            headers: HashMap::new(),

            // Default to none
            http_type: HttpType::NONE,
            path: String::new(),
        }
    }

    pub fn set_type(&mut self, http_type: HttpType) {
        self.http_type = http_type;
    }

    pub fn get_type(self) -> HttpType {
        self.http_type
    }

    // Self explanatory
    pub fn add_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn _get_header(&self, key: String) -> Option<&String> {
        self.headers.get(key.as_str())
    }
}

pub struct HttpResponse {
    version: &'static str,
    status: HttpStatus,
    headers: Vec<(String, String)>,
    body: String,
}

impl HttpResponse {
    pub fn new(version: &'static str, status: HttpStatus, body: &str) -> Self {
        HttpResponse {
            version,
            status,
            headers: Vec::new(),
            body: body.to_string(),
        }
    }

    pub fn _add_header(&mut self, name: &str, value: &str) {
        self.headers.push((name.to_string(), value.to_string()));
    }

    pub fn to_string(&self) -> String {
        let mut response = format!(
            "{} {} {}\r\n",
            self.version,
            self.status.code(),
            self.status.message()
        );
        for (name, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", name, value));
        }
        response.push_str(&format!("Content-Length: {}\r\n", self.body.len()));
        response.push_str("\r\n");
        response.push_str(&self.body);
        response
    }
}
