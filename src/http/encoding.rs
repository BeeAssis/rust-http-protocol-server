
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;

use crate::http::request::Request;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Gzip,
    Brotli,
    Deflate,
    Zstd,
}

pub fn parse_encoding(token: &str) -> Option<Encoding> {
    let token = token.trim();

    if token.eq_ignore_ascii_case("gzip") {
        Some(Encoding::Gzip)
    } else if token.eq_ignore_ascii_case("br") {
        Some(Encoding::Brotli)
    } else if token.eq_ignore_ascii_case("deflate") {
        Some(Encoding::Deflate)
    } else if token.eq_ignore_ascii_case("zstd") {
        Some(Encoding::Zstd)
    } else {
        None
    }

}

pub fn select_encoding(request: &Request) -> Option<Encoding> {
    let header = request.headers.get("accept-encoding")?;

    for token in header.split(',') {
        if let Some(compression_scheme) = parse_encoding(token) {
            match compression_scheme {
                Encoding::Gzip => return Some(Encoding::Gzip),
                Encoding::Brotli => {

                },
                Encoding::Deflate =>{

                },
                Encoding::Zstd => {

                },
            }
        }
    }

    None
}

pub fn gzip_compress(data: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

    encoder.write_all(data).unwrap();

    encoder.finish().unwrap()

}