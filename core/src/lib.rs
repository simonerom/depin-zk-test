// Define the message type

use bytemuck::{Pod, Zeroable};
pub const BATCH_SIZE: usize = 100;

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Message {
    pub device_id: u64,
    pub signature: u64,
    pub latitude: u32,
    pub longitude: u32,
}
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Owned {
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

// Implement Default for the Owned type
impl Default for Owned {
    fn default() -> Self {
        Owned {
            device_id: 0,
            distance: 0
        }
    }
}