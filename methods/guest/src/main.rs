#![no_main]

use provehash_core::*;
use risc0_zkvm::guest::env;
use serde_json::Value as JsonValue;
risc0_zkvm::guest::entry!(main);

fn main() {
    // The string is a stringyfied json array of strings
    let task_str: String = env::read();
    let batch_array: JsonValue = serde_json::from_str(&task_str)
        .expect("Error while deserializing the task array");
    
    // Deserialize each string of the array in a Message struct to finally obtain a Vec<Message>
    let batch: Vec<Message> = batch_array.as_array().unwrap().iter()
        .map(|s| serde_json::from_str(s.as_str().unwrap()).unwrap())
        .collect();

    // Init the output array
    let mut res: Vec<Result> = Vec::new();
    for message in batch.iter() {
        // Calculate the distance
        let distance = (message.latitude - 41).pow(2) + (message.longitude - 12).pow(2);

        if ((distance) as u64) < 10 {
            println!("Device ID: {} is very close: ({},{}){}", message.device_id, 
                 message.latitude, message.longitude, distance);
            res.push(Result {
                device_id: message.device_id,
                distance: distance as u64,
            });
        }
    }

    env::commit(&res);
    println!("Guest execution completed on {} messages. Starting prooving...", BATCH_SIZE);
}
