#![no_main]

use provehash_core::*;
use risc0_zkvm::guest::env;
use serde_json;
risc0_zkvm::guest::entry!(main);

fn main() {
    // Load the slice of messages from the host
    // println!("Guest execution started... ");
    let batch: [String; BATCH_SIZE] = env::read();
    // println!("Received {} messages.", batch.len());
    // Parse the JSON messages into a slice of Message structs
    let batch: Vec<Message> = batch.iter().map(|s| serde_json::from_str(s).unwrap()).collect();

    // Init the output array
    let mut res: Vec<Result> = Vec::new();
    for message in batch.iter() {
        // Calculate the distance
        let distance = (message.latitude - 41).pow(2) + (message.longitude - 12).pow(2);

        if ((distance) as u64) < 10 {
            // println!("Device ID: {} is very close: ({},{}){}", message.device_id, 
            //     message.latitude, message.longitude, distance);
            res.push(Result {
                device_id: message.device_id,
                distance: distance as u64,
            });
        }
    }

    env::commit(&res);
    println!("Guest execution completed on {} messages. Starting prooving...", BATCH_SIZE);
}
