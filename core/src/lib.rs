// Define the message type

use serde::{Serialize, Deserialize};
pub const TASK_SIZE: usize = 10;
pub const BASE_REWARD: u64 = 1;
pub const REWARD_PER_CONNECTION: u64 = 1;

#[derive(Serialize, Deserialize)]
pub struct DeviceMessage {
    pub device_id: u64,
    pub connections: u64
}


