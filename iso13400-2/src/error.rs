
#[derive(Debug, thiserror::Error)]
pub enum Iso13400Error {
    #[error("ISO13400-2 - input error: {0}")]
    InputError(String),

    #[error("ISO 13400-2 - invalid payload length: {actual} expect at least or equal {expected}")]
    InvalidPayloadLength { actual: usize, expected: usize },
    #[error("ISO 13400-2 - invalid length: {actual} expect at least or equal {expected}")]
    InvalidLength { actual: usize, expected: usize },
    #[error("ISO 13400-2 - invalid version: {version}, reverse: {reverse}")]
    InvalidVersion { version: u8, reverse: u8 },
    #[error("Iso 13400-2 - invalid payload type: {0}")]
    InvalidPayloadType(u16),
}
