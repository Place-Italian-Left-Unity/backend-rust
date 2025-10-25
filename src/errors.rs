pub enum Error {
    IoError(std::io::Error),
}

impl Error {
    pub fn to_response(&self) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
        let err_str = match self {
            Self::IoError(e) => format!("IO Error: {e}"),
        };
        let err_str = err_str.into_bytes();
        let data_len = err_str.len();
        tiny_http::Response::new(
            tiny_http::StatusCode(404),
            vec![tiny_http::Header::from_bytes(b"Content-Type", b"text/plain").unwrap()],
            std::io::Cursor::new(err_str),
            Some(data_len),
            None,
        )
    }
}
