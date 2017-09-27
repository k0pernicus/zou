use hyper::client::Client;
use hyper::client::response::Response;
use hyper::error::Error;
use hyper::header::Headers;
use hyper::method::Method;
use hyper::net::HttpsConnector;
use hyper_openssl::OpensslClient;

use SSL_SUPPORT;

/// Returns an Hyper client, which enables (or not) an HTTPS connector
pub fn get_hyper_client() -> Client {
    unsafe {
        if !SSL_SUPPORT {
            return Client::default();
        }
    }
    // Get client enabling SSL
    let ssl = OpensslClient::new().unwrap();
    // Connect the SSL client with HttpsConnector from Hyper
    let connector = HttpsConnector::new(ssl);
    // Return the client
    Client::with_connector(connector)
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
