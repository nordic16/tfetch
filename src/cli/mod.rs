pub mod tfetch;

// will be needed in the future.
pub enum TMessage {
    DONE,
    BEGINREQ,
}

// generic struct that contains information for a given piece of media.
pub struct Media {
    name: String,
    url: String,
}