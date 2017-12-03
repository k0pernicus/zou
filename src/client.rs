use hyper::client::Client;
use hyper::client::response::Response;
use hyper::error::Error;
use hyper::header::Headers;
use hyper::method::Method;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

/// Structure to store if SSL is required, and to
/// implement a default HTTP/HTTPS client
pub struct Config {
    pub enable_ssl: bool,
}

impl Config {
    /// Get the HTTP/HTTPS Hyper client
    pub fn get_hyper_client(&self) -> Client {
        if !self.enable_ssl {
            return Client::default();
        }
        Client::default_ssl()
    }
}

/// Trait to instantiate an Hyper client, with SSL support
trait SSLSupport {
    /// Function to return a Client, with SSL support
    fn default_ssl() -> Client;
}

impl SSLSupport for Client {
    fn default_ssl() -> Client {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        Client::with_connector(connector)
    }
}

/// Trait that represents some methods to send a specific request
pub trait GetResponse {
    /// Given a specific URL, get the header without the content body (useful to not waste time,
    /// ressources and informations)
    fn get_head_response(&self, url: &str) -> Result<Response, Error>;

    /// Given a specific URL and an header, get the header without the content body (useful to not
    /// waste time, ressources and informations)
    fn get_head_response_using_headers(
        &self,
        url: &str,
        header: Headers,
    ) -> Result<Response, Error>;

    /// Given a specific URL, get the response from the target server
    fn get_http_response(&self, url: &str) -> Result<Response, Error>;

    /// Given a specific URL and an header, get the response from the target server
    fn get_http_response_using_headers(
        &self,
        url: &str,
        header: Headers,
    ) -> Result<Response, Error>;
}

impl GetResponse for Client {
    fn get_head_response(&self, url: &str) -> Result<Response, Error> {
        self.request(Method::Head, url).send()
    }

    fn get_head_response_using_headers(
        &self,
        url: &str,
        custom_header: Headers,
    ) -> Result<Response, Error> {
        self.request(Method::Head, url)
            .headers(custom_header)
            .send()
    }

    fn get_http_response(&self, url: &str) -> Result<Response, Error> {
        self.get_http_response_using_headers(url, Headers::new())
    }

    fn get_http_response_using_headers(
        &self,
        url: &str,
        custom_header: Headers,
    ) -> Result<Response, Error> {
        self.request(Method::Get, url).headers(custom_header).send()
    }
}
