use hyper::Url;

#[derive(Debug)]
pub enum Protocol {
    HTTP,
    HTTPS,
}

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