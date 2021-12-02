#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IOError,
    ParseError(&'static str)
}

pub fn to_module_error(_e: std::io::Error) -> Error {
    Error::IOError
}
