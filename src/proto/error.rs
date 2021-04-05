// use std::error;
// use std::fmt;
use thiserror::Error;

pub type ProtoResult<T> = std::result::Result<T, ProtoError>;

#[derive(Error, Debug)]
/// ProtoError enumerates all the possible errors returned during protobuf message processing
pub enum ProtoError {
    /// A error trying to send a UDP message
    #[error("Send error")]
    UDPSendError { source: std::io::Error },
    /// A error trying to deserialize a protobuf message into an object
    #[error("Failed to deserialize protobuf message")]
    DeserializeError { source: quick_protobuf::Error },
    /// A error trying to serialize an object into a protobuf message
    #[error("Failed to serialize protobuf message")]
    SerializeError { source: quick_protobuf::Error },
    /// A error trying to bind to a UDP port
    #[error("Bind error")]
    UDPBindError { source: std::io::Error },
    /// All other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
/*

impl error::Error for ProtoError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            //ProtoError::EmptyVec => None,

            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            ProtoError::UDPSendError(ref e) => Some(e),
            ProtoError::UDPBindError(ref e) => Some(e),
        }
    }
}

// Implement the conversion from `ParseIntError` to `DoubleError`.
// This will be automatically called by `?` if a `ParseIntError`
// needs to be converted into a `DoubleError`.
impl From<ParseIntError> for DoubleError {
    fn from(err: ParseIntError) -> DoubleError {
        DoubleError::Parse(err)
    }
}

impl fmt::Display for ProtoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProtoError::UDPSendError(e) => write!(f, "UDP Error during send: {}", e),
            ProtoError::UDPBindError(e) => write!(f, "UDP Error during bind: {}", e),
        }
    }
}

*/
