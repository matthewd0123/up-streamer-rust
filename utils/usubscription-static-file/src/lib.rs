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

#![allow(clippy::mutable_key_type)]

use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashSet;
use std::fs::{self, canonicalize};
use std::path::PathBuf;
use std::str::FromStr;
use up_rust::core::usubscription::{
    FetchSubscribersRequest, FetchSubscribersResponse, FetchSubscriptionsRequest,
    FetchSubscriptionsResponse, NotificationsRequest, SubscriberInfo, Subscription,
    SubscriptionRequest, SubscriptionResponse, USubscription, UnsubscribeRequest,
};
use up_rust::{UCode, UStatus, UUri};

pub struct USubscriptionStaticFile {
    static_file: String,
}

impl USubscriptionStaticFile {
    pub fn new(static_file: String) -> Self {
        USubscriptionStaticFile { static_file }
    }
}

#[async_trait]
impl USubscription for USubscriptionStaticFile {
    async fn subscribe(
        &self,
        _subscription_request: SubscriptionRequest,
    ) -> Result<SubscriptionResponse, UStatus> {
        todo!()
    }

    async fn fetch_subscriptions(
        &self,
        fetch_subscriptions_request: FetchSubscriptionsRequest,
    ) -> Result<FetchSubscriptionsResponse, UStatus> {
        // Reads in a file and builds it into a subscription_cache data type
        // This is a static file, so we will just return the same set of subscribers
        // for all URIs
        println!(
            "fetch_subscriptions for topic: {}",
            fetch_subscriptions_request.subscriber()
        );

        let subscription_json_file = PathBuf::from(self.static_file.clone());

        let mut subscriptions_vec = Vec::new();

        println!("subscription_json_file: {:?}", subscription_json_file);
        println!(
            "canonicalize: {:?}",
            canonicalize("./utils/usubscription-static-file/static-configs/testdata.json")
        );

        let subscription_json_file = match canonicalize(subscription_json_file) {
            Ok(path) => path,
            Err(e) => {
                return Err(UStatus::fail_with_code(
                    UCode::INVALID_ARGUMENT,
                    format!("Static subscription file not found: {e:?}"),
                ))
            }
        };

        let data = fs::read_to_string(subscription_json_file).map_err(|e| {
            UStatus::fail_with_code(
                UCode::INVALID_ARGUMENT,
                format!("Unable to read file: {e:?}"),
            )
        })?;

        let res: Value = serde_json::from_str(&data).map_err(|e| {
            UStatus::fail_with_code(
                UCode::INVALID_ARGUMENT,
                format!("Unable to parse JSON: {e:?}"),
            )
        })?;

        if let Some(obj) = res.as_object() {
            for (key, value) in obj {
                println!("key: {}, value: {}", key, value);
                let mut subscriber_set: HashSet<UUri> = HashSet::new();

                if let Some(array) = value.as_array() {
                    for subscriber in array {
                        println!("subscriber: {}", subscriber);

                        if let Some(subscriber_str) = subscriber.as_str() {
                            match UUri::from_str(subscriber_str) {
                                Ok(uri) => {
                                    println!("All good for subscriber");
                                    subscriber_set.insert(uri);
                                }
                                Err(error) => {
                                    println!("Error with Deserializing Subscriber: {}", error);
                                }
                            }
                        } else {
                            println!("Unable to parse subscriber");
                        }
                    }
                }

                println!("key: {}", key);
                let topic = match UUri::from_str(&key.to_string()) {
                    Ok(mut uri) => {
                        println!("All good for key");
                        uri.resource_id = 0x8001;
                        uri
                    }
                    Err(error) => {
                        println!("Error with Deserializing Key: {}", error);
                        continue;
                    }
                };

                let details_vec = Vec::new();
                for subscriber in subscriber_set {
                    let subscriber_info_tmp = SubscriberInfo {
                        uri: Some(subscriber).into(),
                        details: details_vec.clone(),
                        ..Default::default()
                    };
                    let subscription_tmp = Subscription {
                        topic: Some(topic.clone()).into(),
                        subscriber: Some(subscriber_info_tmp).into(),
                        ..Default::default()
                    };
                    subscriptions_vec.push(subscription_tmp);
                }
            }
        }

        println!("{}", res);
        dbg!(&subscriptions_vec);
        let fetch_response = FetchSubscriptionsResponse {
            subscriptions: subscriptions_vec,
            ..Default::default()
        };

        Ok(fetch_response)
    }

    async fn unsubscribe(&self, _unsubscribe_request: UnsubscribeRequest) -> Result<(), UStatus> {
        todo!()
    }

    async fn register_for_notifications(
        &self,
        _notifications_register_request: NotificationsRequest,
    ) -> Result<(), UStatus> {
        Ok(())
    }

    async fn unregister_for_notifications(
        &self,
        _notifications_unregister_request: NotificationsRequest,
    ) -> Result<(), UStatus> {
        todo!()
    }

    async fn fetch_subscribers(
        &self,
        _fetch_subscribers_request: FetchSubscribersRequest,
    ) -> Result<FetchSubscribersResponse, UStatus> {
        todo!();
    }
}
