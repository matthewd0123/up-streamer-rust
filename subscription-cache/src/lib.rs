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

use async_std::sync::Mutex;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use up_rust::core::usubscription::{
    EventDeliveryConfig, FetchSubscriptionsResponse, SubscribeAttributes, SubscriberInfo,
    SubscriptionStatus,
};
use up_rust::UUri;

pub type SubscribersMap = Mutex<HashMap<String, HashSet<SubscriptionInformation>>>;

// Tracks subscription information inside the SubscriptionCache
pub struct SubscriptionInformation {
    pub topic: UUri,
    pub subscriber: SubscriberInfo,
    pub status: SubscriptionStatus,
    pub attributes: SubscribeAttributes,
    pub config: EventDeliveryConfig,
}

impl Eq for SubscriptionInformation {}

impl PartialEq for SubscriptionInformation {
    fn eq(&self, other: &Self) -> bool {
        self.subscriber == other.subscriber
    }
}

impl Hash for SubscriptionInformation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.subscriber.hash(state);
    }
}

impl Clone for SubscriptionInformation {
    fn clone(&self) -> Self {
        Self {
            topic: self.topic.clone(),
            subscriber: self.subscriber.clone(),
            status: self.status.clone(),
            attributes: self.attributes.clone(),
            config: self.config.clone(),
        }
    }
}

pub struct SubscriptionCache {
    subscription_cache_map: SubscribersMap,
}

/// A [`SubscriptionCache`] is used to store and manage subscriptions to
/// topics. It is kept local to the streamer. The streamer will receive updates
/// from the subscription service, and update the SubscriptionCache accordingly.
impl SubscriptionCache {
    pub fn new(subscription_cache_map: FetchSubscriptionsResponse) -> Self {
        let mut subscription_cache_hash_map = HashMap::new();
        for subscription in subscription_cache_map.subscriptions {
            let topic = if let Some(topic) = subscription.topic.into_option() {
                topic
            } else {
                println!("Unable to parse URI from subscription, skipping...");
                continue;
            };
            let subscriber = if let Some(subscriber) = subscription.subscriber.into_option() {
                subscriber
            } else {
                println!("Unable to parse subscriber from subscription, skipping...");
                continue;
            };
            let status = if let Some(status) = subscription.status.into_option() {
                status
            } else {
                println!("Unable to parse status from subscription, setting as default");
                SubscriptionStatus::default()
            };
            let attributes = if let Some(attributes) = subscription.attributes.into_option() {
                attributes
            } else {
                println!("Unable to parse attributes from subscription, setting as default");
                SubscribeAttributes::default()
            };
            let config = if let Some(config) = subscription.config.into_option() {
                config
            } else {
                println!("Unable to parse config from subscription, setting as default");
                EventDeliveryConfig::default()
            };
            // Create new hashset if the key does not exist and insert the subscription
            let subscription_information = SubscriptionInformation {
                topic: topic.clone(),
                subscriber,
                status,
                attributes,
                config,
            };
            let subscriber_authority_name = subscription_information
                .subscriber
                .uri
                .as_ref()
                .unwrap()
                .authority_name
                .clone();
            subscription_cache_hash_map
                .entry(subscriber_authority_name)
                .or_insert_with(HashSet::new)
                .insert(subscription_information);
        }
        Self {
            subscription_cache_map: Mutex::new(subscription_cache_hash_map),
        }
    }

    pub async fn fetch_cache_entry(
        &self,
        entry: String,
    ) -> Option<HashSet<SubscriptionInformation>> {
        let map = self.subscription_cache_map.lock().await;
        map.get(&entry).cloned()
    }

    pub async fn fetch_cache(&self) -> HashMap<String, HashSet<SubscriptionInformation>> {
        self.subscription_cache_map.lock().await.clone()
    }
}
