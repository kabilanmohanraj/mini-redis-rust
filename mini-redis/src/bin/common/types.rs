use bytes::Bytes;
use tokio::sync::oneshot;

// represents operations such as SET, GET, PUBLISH etc.
// to enable communication between the client and the clerk
type OpResponse<T> = oneshot::Sender<mini_redis::Result<T>>;

pub enum Op {
    Set {
        key: String,
        value: Bytes,
        reply_channel: OpResponse<()>
    },
    Get {
        key: String,
        reply_channel: OpResponse<Option<Bytes>> // Option<Bytes> - to handle empty GET responses
    },
}