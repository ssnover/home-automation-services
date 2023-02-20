use roslibrust::RosLibRustError;

pub fn to_io_error(err: RosLibRustError) -> std::io::Error {
    match err {
        RosLibRustError::Disconnected => std::io::ErrorKind::ConnectionReset,
        RosLibRustError::CommFailure(_err) => std::io::ErrorKind::Other,
        RosLibRustError::InvalidMessage(_err) => std::io::ErrorKind::InvalidInput,
        RosLibRustError::Timeout(_err) => std::io::ErrorKind::TimedOut,
        RosLibRustError::Unexpected(_err) => std::io::ErrorKind::Other,
    }
    .into()
}
