#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IOError,
    ParseError
}

pub fn to_module_error(_e: std::io::Error) -> Error {
    Error::IOError
}
