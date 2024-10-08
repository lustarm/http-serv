use log::info;
use std::collections::HashMap;
use tokio::net::TcpListener;

use crate::handle;
use crate::router::Router;

#[derive(PartialEq, Clone)]
pub enum HttpType {
    // None for when we don't
    // know http type...
    // if http_type == HttpType::NONE
    GET,
    _NONE,
    _POST,
}

pub enum HttpStatus {
    Ok,
    NotFound,
    _InternalServerError,
    // Add more status codes as needed
}

impl HttpStatus {
    fn code(&self) -> u16 {
        match self {
            HttpStatus::Ok => 200,
            HttpStatus::NotFound => 404,
            HttpStatus::_InternalServerError => 500,
            // Handle other status codes
        }
    }

    fn message(&self) -> &'static str {
        match self {
            HttpStatus::Ok => "OK",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::_InternalServerError => "Internal Server Error",
            // Handle other status messages
        }
    }
}

// Just easier to read later on
type HttpHeaders = HashMap<String, String>;

#[derive(Clone)]
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
            http_type: HttpType::_NONE,
            path: String::new(),
        }
    }

    // Http type
    pub fn set_type(&mut self, http_type: HttpType) {
        self.http_type = http_type;
    }

    pub fn get_type(self) -> HttpType {
        self.http_type
    }

    // Path
    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    pub fn get_path(self) -> String {
        self.path
    }

    // Headers
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

pub struct HttpService {
    listener: TcpListener,
    router: Router,
}

impl HttpService {
    pub async fn new(router: Router) -> Self {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        HttpService { listener, router }
    }

    pub async fn listen_and_serve(self) -> std::io::Result<()> {
        loop {
            let (writer, _) = self.listener.accept().await?;
            let router = self.router.clone();

            tokio::spawn(async move {
                /*
                    GET / HTTP/1.1
                    Host: localhost:8080
                    User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:130.0) Gecko/20100101 Firefox/130.0
                    Accept
                */

                // Create handler

                let mut handler = handle::Handler::new(writer, router);
                handler
                    .handle_request()
                    .await
                    .unwrap_or_else(|e| info!("Error: {}", e));
            });
        }
    }
}
