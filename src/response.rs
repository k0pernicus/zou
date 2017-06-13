use hyper::client::response::Response;
use hyper::status::StatusCode;

pub trait CheckResponseStatus {
    /// Function to check if the `PartialContent` status is contained
    /// in the HTTP header response
    fn check_partialcontent_status(&self) -> bool;
    fn is_ok(&self) -> bool;
}

impl CheckResponseStatus for Response {
    fn check_partialcontent_status(&self) -> bool {
        self.status == StatusCode::PartialContent
    }
    fn is_ok(&self) -> bool {
        self.status.is_success()
    }
}
