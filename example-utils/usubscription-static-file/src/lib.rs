/********************************************************************************
 * Copyright (c) 2024 Contributors to the Eclipse Foundation
 *
 * See the NOTICE file(s) distributed with this work for additional
 * information regarding copyright ownership.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Apache License Version 2.0 which is available at
 * https://www.apache.org/licenses/LICENSE-2.0
 *
 * SPDX-License-Identifier: Apache-2.0
 ********************************************************************************/

use serde_json;
use std::collections::{HashMap, HashSet};
use std::fs::{self, canonicalize};
use std::path::PathBuf;
use std::str::FromStr;
use up_rust::UUri;

pub struct USubscriptionStaticFile {
    // subscribers_map: Mutex<HashMap<UUri, HashSet<UUri>>>,
}

impl USubscriptionStaticFile {
    pub fn new() -> Self {
        // USubscriptionStaticFile {
        //     subscribers_map: Mutex::new(HashMap::new())
        // }
        Self {}
    }

    pub fn fetch_subscribers(&self, topic: UUri) -> HashMap<UUri, HashSet<UUri>> {
        // Reads in a file and builds it into a subscription_cache data type
        // This is a static file, so we will just return the same set of subscribers
        // for all URIs
        println!("fetch_subscribers for topic: {}", topic);
        let crate_dir = env!("CARGO_MANIFEST_DIR");
        let subscription_json_file = PathBuf::from(crate_dir).join("static-configs/testdata.json");
        let subscription_json_file = canonicalize(subscription_json_file).ok();
        println!("subscription_json_file: {subscription_json_file:?}");
        // let subscription_json_file = "./testdata.json";
        let data = fs::read_to_string(subscription_json_file.unwrap()).expect("Unable to read file");
        let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");

        let mut subscribers_map = HashMap::new();
        for (key, value) in res.as_object().unwrap() {
            println!("key: {}, value: {}", key, value);
            let mut subscriber_set: HashSet<UUri> = HashSet::new();
            for subscriber in value.as_array().unwrap() {
                println!("subscriber: {}", subscriber.as_str().expect("Unable to parse"));
                match UUri::from_str(&subscriber.as_str().expect("Unable to parse")) {
                    Ok(uri) => {
                        println!("All good for subscriber");
                        subscriber_set.insert(uri);
                    }
                    Err(error) => {
                        println!("Error with Deserializing Subscriber: {error}");
                    }
                };
            }
            println!("key: {}", key.to_string());
            match UUri::from_str(&key.to_string()) {
                Ok(uri) => {
                    println!("All good for key");
                    subscribers_map.insert(uri, subscriber_set);
                }
                Err(error) => {
                    println!("Error with Deserializing Key: {error}");
                }
            };
        }
        println!("{}", res);
        subscribers_map
    }
}
