#![no_main]

use dewi_core::*;
use risc0_zkvm::guest::env;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

risc0_zkvm::guest::entry!(main);

fn main() {
    let task = get_task();

    // Init the public output: a hashmap where we store rewards due to each device
    let mut rewards_map = HashMap::new();

    for message in task.iter() {
        // Calculate the rewards
        let reward = rewards_map.entry(message.device_id).or_insert(0 as u64);
        *reward = *reward + BASE_REWARD + message.connections * REWARD_PER_CONNECTION;
    }
    
    // Serialize the result array and send it back to the host
    let stringyfied_res = serde_json::to_string(&rewards_map).unwrap();

    env::commit(&stringyfied_res);
    println!("Guest execution completed on {} messages. Starting prooving...", TASK_SIZE);
}

fn get_task() -> Vec<DeviceMessage> {
    // The array of messages is a stringyfied json array of strings
    let task_str: String = env::read();
    let task_array: JsonValue = serde_json::from_str(&task_str)
        .expect("Error while deserializing the task array");
    
    // Deserialize each string of the array in a Message struct to finally obtain a Vec<Message>
    let task: Vec<DeviceMessage> = task_array.as_array().unwrap().iter()
        .map(|s| serde_json::from_str(s.as_str().unwrap()).unwrap())
        .collect();
    task
}
