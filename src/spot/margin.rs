use crate::client::Client;

pub struct Margin {
    pub client: Client,
    pub recv_window: u64,
}