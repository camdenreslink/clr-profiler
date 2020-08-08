#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    InvalidMethodHeaderFlags,
    InvalidCilOpcode,
}
