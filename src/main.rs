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

use dewi_core::*;
use rand::Rng;
use serde_json;
use dewi_demo::process_task;
use dewi_demo_methods::DEWI_ID;

fn main() {
    let mut task_array: Vec<String> = Default::default();
    // Fill in the task with some data
    for i in 0..TASK_SIZE {
        // Create a new message struct
        let mut message= DeviceMessage { device_id:0, connections:0};
        message.device_id = rand::thread_rng().gen_range(0..3) as u64;
        // Let's generate a valid random number of connected clients
        message.connections = rand::thread_rng().gen_range(0..4) as u64;
        // Serialize the message to a JSON string
        let serstr: String = serde_json::to_string(&message).unwrap();
        println!("Message: {}", serstr);
        task_array.push(serstr);
    }

    let task_str = serde_json::to_string(&task_array).unwrap();
    println!("Processing task: {}", task_str);
    let (receipt, _) = process_task(task_str);

    // Here is where one would send 'receipt' over the network...

    // Verify receipt, panic if it's wrong
    receipt.verify(DEWI_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
}
