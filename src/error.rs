use std::io;
use hyper;
use hyper::status::StatusCode;
use serde_json;

error_chain! {
    foreign_links {
            HyperError(hyper::Error);
            UrlError(hyper::error::ParseError);
            IoError(io::Error);
            JsonError(serde_json::error::Error);
    }

    errors {
            NoEntries {
                description("No journal entries")
                display("No journal entries received")
            }
            RequestError(status_code: StatusCode, body: String) {
                description("Not OK result code")
                display("Not OK result code received: {} Result body: {}", status_code, body)
            }
    }
}
