// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use provehash_core::*;
use rand::Rng;
use serde_json;
use hello_world::process_batch;
use hello_world_methods::MULTIPLY_ID;

fn main() {
    let mut batch: Vec<String> = Default::default();
    // Fill in the batch with some data
    for i in 0..BATCH_SIZE {
        // Create a new message struct
        let mut message = Message::default();
        message.device_id = i as u64;
        // Let's generate a valid random latitude, as an integer in the range [-11, 14]
        message.latitude = rand::thread_rng().gen_range(40..45) as u32;
        // Let's generate a valid random longitude, as an integer in the range [-41, 42]
        message.longitude = rand::thread_rng().gen_range(10..18) as u32;
        // Serialize the message to a JSON string
        let serstr: String = serde_json::to_string(&message).unwrap();
        println!("Message: {}", serstr);
        batch.push(serstr);
    }

    let (receipt, _) = process_batch(batch);

    // Here is where one would send 'receipt' over the network...

    // Verify receipt, panic if it's wrong
    receipt.verify(MULTIPLY_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
}
