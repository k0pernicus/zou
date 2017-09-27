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

/// Get useful informations about the mirror (or server) state
///
/// # Arguments
/// * `accept_partialcontent`: is the server accepts the PartialContent status ?
/// * `is_accessible`: is the server is up, and is the response status is ok ?
/// * `url`: the URL of the server
struct MirrorInfo<'a> {
    accept_partialcontent: bool,
    is_accessible: bool,
    url: &'a str,
}

impl<'a> Default for MirrorInfo<'a> {
    /// Default data structure
    fn default() -> MirrorInfo<'a> {
        MirrorInfo {
            accept_partialcontent: false,
            is_accessible: false,
            url: "",
        }
    }
}

pub struct CargoInfo<'a> {
    pub accept_partialcontent: bool,
    pub auth_header: Option<AuthorizationHeaderFactory>,
    pub best_mirrors: MirrorsList<'a>,
    pub content_length: Bytes,
}

fn get_mirror_info<'a>(filename: &str, server_url: &'a str, ssl_support: bool) -> MirrorInfo<'a> {

    let mut mirror_info = MirrorInfo::default();
    mirror_info.url = server_url;

    let file_url = Path::new(server_url).join(filename);
    info!(&format!("Checking server {}", server_url));
    // Check any Path error
    if file_url.to_str().is_none() {
        return mirror_info;
    }
    let file_url = file_url.to_str().unwrap();
    let current_config = Config { enable_ssl: ssl_support };
    let hyper_client = current_config.get_hyper_client();
    // hyper_client.set_read_timeout(Some(Duration::from_secs(3)));
    let client_response = hyper_client.get_head_response(file_url).unwrap();

    info!(&format!(
        "[{}] Waiting a response from the remote server... ",
        server_url
    ));

    if !client_response.version.greater_than_http_11() {
        warning!(&format!("[{}] HTTP version <= 1.0 detected", server_url));
    } else {
        ok!("");
    }

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
                    error!(&format!(
                        "The remote content is protected by {} \
                                                 Authorization, which is not supported!\nYou \
                                                 can create a new issue to report this problem \
                                                 in https://github.\
                                                 com/k0pernicus/zou/issues/new",
                        a_type
                    ));
                    // Stop the function - the credentials are not OK
                    return mirror_info;
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
                .get_head_response_using_headers(file_url, headers)
                .unwrap()
        }
        None => client_response,
    };

    // If there is some trouble with the response, return back the information about the mirror
    if !client_response.is_ok() {
        return mirror_info;
    }

    mirror_info.is_accessible = true;

    info!(&format!(
        "[{}] Checking the server's support for PartialContent headers...",
        server_url
    ));

    // Ask the first byte, just to know if the server accept PartialContent status
    let mut header = Headers::new();
    header.set(Range::Bytes(vec![ByteRangeSpec::FromTo(0, 1)]));

    let client_response = hyper_client
        .get_head_response_using_headers(file_url, header)
        .unwrap();

    // Is the mirror supports PartialContent status ?
    mirror_info.accept_partialcontent = client_response.check_partialcontent_status();
    mirror_info
}

/// Get Rust structure that contains network benchmarks
pub fn get_cargo_info<'a>(
    filename: &str,
    server_urls: MirrorsList<'a>,
    ssl_support: bool,
) -> Result<CargoInfo<'a>, String> {

    let current_config = Config { enable_ssl: ssl_support };

    let best_mirrors = match server_urls.len() {
        0 | 1 => server_urls,
        _ => {
            info!("Getting informations about possible mirrors...");

            let mut available_mirrors: MirrorsList<'a> = Vec::with_capacity(server_urls.len());
            let candidates: Vec<MirrorInfo<'a>> = server_urls
                .par_iter()
                .map(|server_url| {
                    get_mirror_info(filename, server_url, ssl_support)
                })
                .collect();

            // If any 'true' mirrors...
            // TODO: For multi-threaded code
            for candidate in candidates {
                if candidate.is_accessible && candidate.accept_partialcontent {
                    info!(&format!("Mirror {} is ok!", candidate.url));
                    available_mirrors.push(candidate.url);
                } else {
                    warning!(&format!("Mirror {} is not ok!", candidate.url));
                }
            }

            if available_mirrors.is_empty() {
                panic!("No mirrors available!");
            } else {
                info!(&format!("{} mirror(s) available!", available_mirrors.len()));
            }

            info!("Ranking mirrors...");

            // Get best mirrors from the list of available mirrors
            bench_mirrors(available_mirrors, filename, ssl_support)
        }
    };

    let path_fst_mirror = Path::new(best_mirrors[0]).join(filename);
    // Get the first mirror to get global informations
    let fst_mirror = path_fst_mirror.to_str().unwrap();

    let hyper_client = current_config.get_hyper_client();
    let client_response = hyper_client.get_head_response(fst_mirror).unwrap();

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
                .get_head_response_using_headers(fst_mirror, headers)
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
                .get_http_response_using_headers(filename, custom_http_header)
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
        best_mirrors: best_mirrors,
        content_length: remote_content_length,
    })
}
