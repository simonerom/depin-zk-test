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

#![no_main]
//#![no_std]


use provehash_core::*;

use risc0_zkvm::{
    guest::env
};

use sha2::{Digest as _, Sha256};
risc0_zkvm::guest::entry!(main);

fn main() {
    // Load the slice of messages from the host
    let mut batch: [Message; BATCH_SIZE] = [Default::default(); BATCH_SIZE];
    env::read_slice(&mut batch);

    // Iterate over the slice messages and compute the hash of the data filed
    // Build an array that maps the device id to the first 2 bytes of the hash

    // init the array
    let mut res: [Owned; BATCH_SIZE] = [Default::default(); BATCH_SIZE];
    let mut i = 0;
    for message in batch.iter() {
        // Calculate the location distance from the Coloseum in Rome
        let distance = (message.latitude as f64 - 41.890251).powi(2) + (message.longitude as f64 - 12.492373).powi(2);
        //let distance = distance.sqrt() as u64;

        if (distance as  u64) < 5 {
            println!("Device ID: {} is very close: {}", message.device_id, distance);
            res[i] = Owned {device_id: message.device_id, distance: distance as u64};
            i+=1;
        }
        //let data_str = format!("{:?}{:?}",message.latitude, message.longitude);
        //let _digest = Sha256::digest(&data_str.as_bytes());
        // extract the first 2 bytes of the hash
        /*let hash_str = format!("{:?}", digest);
        let code16 = &hash_str[2..4];
        let code64 = u16::from_str_radix(code, 64).unwrap();
        res[i] = Owned {device_id, code};
        i+=1*/
    }
    env::commit(&BATCH_SIZE);
    println!("Guest execution completed on {} messages. Starting prooving...", BATCH_SIZE);
}
