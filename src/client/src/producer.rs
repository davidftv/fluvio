use std::io::Error as IoError;
use std::io::ErrorKind;

use tracing::debug;
use tracing::trace;

use dataplane::ReplicaKey;

use crate::ClientError;
use crate::spu::SpuPool;
use crate::client::SerialFrame;

/// produce message to replica leader
pub struct Producer {
    replica: ReplicaKey,
    pool: SpuPool,
}

impl Producer {
    pub fn new(replica: ReplicaKey, pool: SpuPool) -> Self {
        Self { replica, pool }
    }

    pub fn replica(&self) -> &ReplicaKey {
        &self.replica
    }

    /// send records to spu leader for replica
    pub async fn send_record(&mut self, record: Vec<u8>) -> Result<(), ClientError> {
        debug!(
            "sending records: {} bytes to: {}",
            record.len(),
            self.replica
        );

        let spu_client = self.pool.create_serial_socket(&self.replica).await?;

        debug!("connect to replica leader at: {}", spu_client);

        send_record_raw(spu_client, &self.replica, record).await
    }
}

/// Sends record to a target server (Kf, SPU, or SC)
async fn send_record_raw<F: SerialFrame>(
    mut leader: F,
    replica: &ReplicaKey,
    record: Vec<u8>,
) -> Result<(), ClientError> {
    use dataplane::produce::DefaultProduceRequest;
    use dataplane::produce::DefaultPartitionRequest;
    use dataplane::produce::DefaultTopicRequest;
    use dataplane::batch::DefaultBatch;
    use dataplane::record::DefaultRecord;

    // build produce log request message
    let mut request = DefaultProduceRequest::default();
    let mut topic_request = DefaultTopicRequest::default();
    let mut partition_request = DefaultPartitionRequest::default();

    debug!(
        "send record {} bytes to: replica: {}, {}",
        record.len(),
        replica,
        leader
    );

    let record_msg: DefaultRecord = record.into();
    let mut batch = DefaultBatch::default();
    batch.records.push(record_msg);

    partition_request.partition_index = replica.partition;
    partition_request.records.batches.push(batch);
    topic_request.name = replica.topic.to_owned();
    topic_request.partitions.push(partition_request);

    request.acks = 1;
    request.timeout_ms = 1500;
    request.topics.push(topic_request);

    trace!("produce request: {:#?}", request);

    let response = leader.send_receive(request).await?;

    trace!("received response: {:?}", response);

    // process response
    match response.find_partition_response(&replica.topic, replica.partition) {
        Some(partition_response) => {
            if partition_response.error_code.is_error() {
                return Err(ClientError::IoError(IoError::new(
                    ErrorKind::Other,
                    partition_response.error_code.to_sentence(),
                )));
            }
            Ok(())
        }
        None => Err(ClientError::IoError(IoError::new(
            ErrorKind::Other,
            "unknown error",
        ))),
    }
}
