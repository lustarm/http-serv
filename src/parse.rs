use log::info;

use crate::http::*;

pub fn parse_req(input: &[u8]) -> HttpPayload {
    let mut lines = input.split(|&byte| byte == b'\r'); // Split on `\r`

    // Create http payload
    let mut http_payload = HttpPayload::new();

    // Process the first line
    if let Some(line) = lines.next() {
        let line = line.trim_ascii_end(); // Trim trailing `\n` and any spaces

        // Check if it's a valid GET request line
        if line.starts_with(b"GET ") {
            // Extract the path and HTTP version
            let parts: Vec<&[u8]> = line.split(|&byte| byte == b' ').collect();
            if parts.len() >= 3 {
                let path = String::from_utf8_lossy(parts[1]);
                let version = String::from_utf8_lossy(parts[2]);

                // Set payload to GET
                info!("Method: GET");
                http_payload.set_type(HttpType::GET);

                // We don't care abt this rn
                info!("Path: {}", path);
                info!("Version: {}", version);
            }
        } else {
            info!("Invalid request");
        }
    }

    // Process the remaining lines (headers)
    for line in lines {
        let line = line.trim_ascii_start(); // Trim leading `\n` and any spaces

        if !line.is_empty() {
            let header: Vec<&[u8]> = line.split(|&byte| byte == b':').collect();
            if header.len() == 2 {
                let name = String::from_utf8_lossy(header[0]).trim().to_string();
                let value = String::from_utf8_lossy(header[1]).trim().to_string();

                // Add header to payload
                info!("Header: {}: {}", name, value);
                http_payload.add_header(name, value);
            }
        }
    }

    // Return payload
    http_payload
}
