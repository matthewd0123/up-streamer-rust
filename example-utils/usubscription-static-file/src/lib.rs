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
 
use std::collections::{HashSet, HashMap};
use up_rust::{UUri, UStatus};
use async_std::sync::Mutex;
use subscription_cache::{SubscribersMap, SubscriptionCache};
use serde_json;
use std::fs;
use std::str::FromStr;


// pub trait USubscription {
//     async fn fetch_all_subscribers(&self) -> SubscriptionCache;
// }


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

    pub fn fetch_subscribers(&self, topic: UUri) -> HashMap<UUri, HashSet<UUri>>{
        // Reads in a file and builds it into a subscription_cache data type
        // This is a static file, so we will just return the same set of subscribers
        // for all URIs
        println!("fetch_subscribers for topic: {}", topic);
        let subscription_json_file = "./testdata.json";
        let data = fs::read_to_string(subscription_json_file).expect("Unable to read file");
        let res: serde_json::Value = serde_json::from_str(&data).expect("Unable to parse");

        let mut subscribers_map = HashMap::new();
        for (key, value) in res.as_object().unwrap() {
            println!("key: {}, value: {}", key, value);
            let mut subscriber_set: HashSet<UUri> = HashSet::new();
            for subscriber in value.as_array().unwrap() {
                println!("subscriber: {}", subscriber);
                subscriber_set.insert(UUri::from_str(&subscriber.to_string()).unwrap());
            }
            subscribers_map.insert(UUri::from_str(&key.to_string()).unwrap(), subscriber_set);
        }
        println!("{}", res);
        subscribers_map
    }
} 