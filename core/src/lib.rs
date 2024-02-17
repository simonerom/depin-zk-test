// Define the message type

use bytemuck::{Pod, Zeroable};
use serde::{Serialize, Deserialize};
pub const BATCH_SIZE: usize = 10;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Message {
    pub device_id: u64,
    pub signature: u64,
    pub latitude: u32,
    pub longitude: u32,
}
#[derive(Serialize, Deserialize)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Result {
    pub device_id: u64,
    pub distance: u64
}

// Implement Default for the Message type
impl Default for Message {
    fn default() -> Self {
        Message {
            device_id: 0,
            signature: 0,
            latitude: 0,
            longitude: 0,
        }
    }
}

// Implement Default for the Result type
impl Default for Result {
    fn default() -> Self {
        Result {
            device_id: 0,
            distance: 0
        }
    }
}

