use authorization::{AuthorizationHeaderFactory, AuthorizationType, GetAuthorizationType};
use Bytes;
use client::{Config, GetResponse};
use contentlength::GetContentLength;
use hyper::header::{ByteRangeSpec, Headers, Range};
use std::error;
use std::fmt;
use std::result::Result;
use util::prompt_user;

/// Contains informations about the remote server
pub struct RemoteServerInformations<'a> {
    pub accept_partialcontent: bool,
    pub auth_header: Option<AuthorizationHeaderFactory>,
    pub file: RemoteFileInformations,
    pub url: &'a str,
}

/// Contains informations about the remote file
pub struct RemoteFileInformations {
    pub content_length: Bytes,
}

/// Some enumeration to display easily errors
#[derive(Debug)]
pub enum RemoteServerError {
    TooMuchAttempting(usize),
    UnknownAuthorizationType(AuthorizationType),
}

impl fmt::Display for RemoteServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoteServerError::TooMuchAttempting(ref attempts) => write!(f, "{} attempts failed", attempts),
            RemoteServerError::UnknownAuthorizationType(ref unknown_type) => write!(f, "{} is not supporting by Zou.\
                                                                                        You can create a new issue to report this problem \
                                                                                        at https://github.com/k0pernicus/zou/issues/new", unknown_type) 
        }
    }
}

impl error::Error for RemoteServerError {
    fn description(&self) -> &str {
        match *self {
            RemoteServerError::TooMuchAttempting(_) => "Many attempts failed",
            RemoteServerError::UnknownAuthorizationType(_) => "Authorization type not supported"
        }
    }
}

type RemoteServerInformationsResult<'a> = Result<RemoteServerInformations<'a>, RemoteServerError>;

/// Get Rust structure that contains network benchmarks
pub fn get_cargo_info<'a>(
    url: &'a str,
    ssl_support: bool,
) -> RemoteServerInformationsResult<'a> {

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
                    return Err(RemoteServerError::UnknownAuthorizationType(a_type));
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
                    return Err(RemoteServerError::TooMuchAttempting(2));
                }
            }
        }
    };

    Ok(RemoteServerInformations {
        // TODO: TO REFACTOR --- Be careful if there is only one server...
        accept_partialcontent: true,
        auth_header: auth_header_factory,
        file: RemoteFileInformations {
            content_length: remote_content_length,
        },    
        url: url,
    })
}
