// error.rs
//  Server Error handling (union of errors used by server)
//

use std::fmt;
use std::io::Error as IoError;
use futures::channel::mpsc::SendError;

use fluvio_types::PartitionError;
use k8_client::ClientError;
use kf_socket::KfSocketError;

#[derive(Debug)]
pub enum ScError {
    IoError(IoError),
    SendError(SendError),
    ClientError(ClientError),
    SocketError(KfSocketError),
    PartitionError(PartitionError),
}

impl fmt::Display for ScError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IoError(err) => write!(f, "{}", err),
            Self::SendError(err) => write!(f, "{}", err),
            Self::ClientError(err) => write!(f, "{}", err),
            Self::SocketError(err) => write!(f, "{}", err),
            Self::PartitionError(err) => write!(f, "{}", err),
        }
    }
}

impl From<IoError> for ScError {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}

impl From<SendError> for ScError {
    fn from(error: SendError) -> Self {
        Self::SendError(error)
    }
}

impl From<ClientError> for ScError {
    fn from(error: ClientError) -> Self {
        Self::ClientError(error)
    }
}

impl From<KfSocketError> for ScError {
    fn from(error: KfSocketError) -> Self {
        Self::SocketError(error)
    }
}

impl From<PartitionError> for ScError {
    fn from(error: PartitionError) -> Self {
        Self::PartitionError(error)
    }
}
