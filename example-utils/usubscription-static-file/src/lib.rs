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

use std::collections::{HashMap, HashSet};
use std::fs::{self, canonicalize};
use std::path::PathBuf;
use std::str::FromStr;
use up_rust::UUri;

pub struct USubscriptionStaticFile {}

impl Default for USubscriptionStaticFile {
    fn default() -> Self {
        Self::new()
    }
}

impl USubscriptionStaticFile {
    pub fn new() -> Self {
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
        let data =
            fs::read_to_string(subscription_json_file.unwrap()).expect("Unable to read file");
        let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");

        let mut subscribers_map = HashMap::new();
        for (key, value) in res.as_object().unwrap() {
            println!("key: {}, value: {}", key, value);
            let mut subscriber_set: HashSet<UUri> = HashSet::new();
            for subscriber in value.as_array().unwrap() {
                println!(
                    "subscriber: {}",
                    subscriber.as_str().expect("Unable to parse")
                );
                match UUri::from_str(subscriber.as_str().expect("Unable to parse")) {
                    Ok(uri) => {
                        println!("All good for subscriber");
                        subscriber_set.insert(uri);
                    }
                    Err(error) => {
                        println!("Error with Deserializing Subscriber: {error}");
                    }
                };
            }
            println!("key: {key}");
            match UUri::from_str(&key.to_string()) {
                Ok(mut uri) => {
                    println!("All good for key");
                    uri.resource_id = 0x8001;
                    subscribers_map.insert(uri, subscriber_set);
                }
                Err(error) => {
                    println!("Error with Deserializing Key: {error}");
                }
            };
        }
        println!("{}", res);
        dbg!(&subscribers_map);
        subscribers_map
    }
}
