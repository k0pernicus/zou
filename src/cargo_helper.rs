use authorization::{AuthorizationHeaderFactory, AuthorizationType, GetAuthorizationType};
use Bytes;
use bench::bench_mirrors;
use client::{Config, GetResponse};
use contentlength::GetContentLength;
use http_version::ValidateHttpVersion;
use hyper::header::{ByteRangeSpec, Headers, Range};
use MirrorsList;
use response::CheckResponseStatus;
use std::result::Result;
use std::path::Path;
use rayon::prelude::*;
use util::prompt_user;

pub struct CargoInfo<'a> {
    pub accept_partialcontent: bool,
    pub auth_header: Option<AuthorizationHeaderFactory>,
    pub content_length: Bytes,
    pub url: &'a str,
}

/// Get Rust structure that contains network benchmarks
pub fn get_cargo_info<'a>(
    url: &'a str,
    ssl_support: bool,
) -> Result<CargoInfo<'a>, String> {

    let current_config = Config { enable_ssl: ssl_support };

    let hyper_client = current_config.get_hyper_client();
    let client_response = hyper_client.get_head_response(url).unwrap();

    let auth_type = client_response.headers.get_authorization_type();
    let auth_header_factory = match auth_type {
        Some(a_type) => {
            match a_type {
                AuthorizationType::Basic => {
                    warning!("The remote content is protected by Basic Auth.");
                    warning!("Please to enter below your credential informations.");
                    let username = prompt_user("Username:");
                    let password = prompt_user("Password:");
                    Some(AuthorizationHeaderFactory::new(
                        AuthorizationType::Basic,
                        username,
                        Some(password),
                    ))
                }
                _ => {
                    panic!(
                        "The remote content is protected by {} \
                                                 Authorization, which is not supported!\nYou \
                                                 can create a new issue to report this problem \
                                                 in https://github.\
                                                 com/k0pernicus/zou/issues/new",
                        a_type
                    );
                }
            }
        }
        None => None,
    };

    let client_response = match auth_header_factory.clone() {
        Some(header_factory) => {
            let mut headers = Headers::new();
            headers.set(header_factory.build_header());
            hyper_client
                .get_head_response_using_headers(url, headers)
                .unwrap()
        }
        None => client_response,
    };

    let remote_content_length = match client_response.headers.get_content_length() {
        Some(remote_content_length) => remote_content_length,
        None => {
            warning!(
                "Cannot get the remote content length, using an \
                                 HEADER request."
            );
            warning!(
                "Trying to send an HTTP request, to get the remote \
                                 content length..."
            );

            // Trying to force the server to send to us the remote content length
            let mut custom_http_header = Headers::new();
            // HTTP header to get all the remote content - if the response is OK, get the
            // ContentLength information sent back from the server
            custom_http_header.set(Range::Bytes(vec![ByteRangeSpec::AllFrom(0)]));
            // Get a response from the server, using the custom HTTP request
            let client_response = hyper_client
                .get_http_response_using_headers(url, custom_http_header)
                .unwrap();
            // Try again to get the content length - if this one is unknown again, stop the program
            match client_response.headers.get_content_length() {
                Some(remote_content_length) => remote_content_length,
                None => {
                    error!("Second attempt has failed.");
                    0
                }
            }
        }
    };

    Ok(CargoInfo {
        // TODO: TO REFACTOR --- Be careful if there is only one server...
        accept_partialcontent: true,
        auth_header: auth_header_factory,
        content_length: remote_content_length,
        url: url,
    })
}
