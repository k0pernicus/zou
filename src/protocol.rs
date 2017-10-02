use hyper::Url;

/// Supported protocols
#[derive(Debug)]
pub enum Protocol {
    HTTP,
    HTTPS,
}

/// Returns an Option type, that contains a Protocol enum
pub fn get_protocol(url: &str) -> Option<Protocol> {
    match Url::parse(url) {
        Ok(url) => match url.scheme() {
            "http" => Some(Protocol::HTTP),
            "https" => Some(Protocol::HTTPS),
            _ => None,
        },
        Err(error) => {
            warning!(&format!("Canno't extract the protocol from the URL: {}", error));
            None
        }
    }
}