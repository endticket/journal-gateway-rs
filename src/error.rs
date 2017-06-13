use std::io;
use hyper;
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
    }
}
