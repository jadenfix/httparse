use httparse::{Error, Request, EMPTY_HEADER};

#[test]
fn error_descriptions_are_static_and_match_display() {
    for error in [
        Error::HeaderName,
        Error::HeaderValue,
        Error::NewLine,
        Error::Status,
        Error::Token,
        Error::TooManyHeaders,
        Error::Version,
    ] {
        let description: &'static str = error.description_str();
        assert_eq!(description, error.to_string());
    }
}

#[test]
fn parsed_error_description_outlives_the_error() {
    let description: &'static str = {
        let mut headers = [EMPTY_HEADER; 1];
        let error = Request::new(&mut headers)
            .parse(b"GET / HTTP/1.1\r\ninvalid header: value\r\n\r\n")
            .unwrap_err();
        error.description_str()
    };
    assert_eq!(description, "invalid header name");
}
