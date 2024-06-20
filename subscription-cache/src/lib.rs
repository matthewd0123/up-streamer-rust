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
use up_rust::{UStatus, UUri};

pub type SubscribersMap = Mutex<HashMap<UUri, HashSet<UUri>>>;

pub struct SubscriberInfoFoo {
    pub uri: UUri,
}

pub struct SubscriptionRequestFoo {
    pub topic: UUri,
    pub subscriber: SubscriberInfoFoo,
}

pub enum State {
    Unsubscribed = 0,
    SubscribePending = 1,
    Subscribed = 2,
    UnsubscribePending = 3,
}

pub struct SubscriptionStatusFoo {
    pub state: State,
    pub message: String,
}

pub struct SubscriptionResponseFoo {
    pub status: SubscriptionStatusFoo,
    pub topic: UUri,
}

pub struct UnsubscribeRequestFoo {
    pub topic: UUri,
    pub subscriber: SubscriberInfoFoo,
}

pub struct FetchSubscriptionsRequestFoo {
    pub topic: UUri,
}

pub struct SubscriptionFoo {
    pub topic: UUri,
    pub subscriber: SubscriberInfoFoo,
    pub status: SubscriptionStatusFoo,
}

pub struct FetchSubscriptionsResponseFoo {
    pub subscriptions: HashSet<SubscriptionFoo>,
}

pub struct FetchSubscribersRequestFoo {
    pub topic: UUri,
}

pub struct FetchSubscribersResponseFoo {
    pub subscribers: HashSet<UUri>,
}

pub struct SubscriptionCache {
    subscription_cache_map: SubscribersMap,
}

impl SubscriptionCache {
    pub fn new(subscription_cache_map: SubscribersMap) -> Self {
        Self {
            subscription_cache_map,
        }
    }

    // pub async fn subscribe(&self, subscription_request: SubscriptionRequestFoo) -> Result<SubscriptionResponseFoo, UStatus> {

    //     let mut subscribers_map = self.subscribers_map.lock().await;

    //     // Check if the topic exists in the map
    //     if !subscribers_map.contains_key(&subscription_request.topic) {
    //         // If the topic does not exist, create a new HashSet for it
    //         subscribers_map.insert(subscription_request.topic.clone(), HashSet::new());
    //     }

    //     // Add the subscriber to the HashSet associated with the topic
    //     let was_inserted = if let Some(subscribers) = subscribers_map.get_mut(&subscription_request.topic) {
    //         subscribers.insert(subscription_request.subscriber.uri.clone())
    //     } else {
    //         false
    //     };

    //     let subscription_status = if was_inserted {
    //         SubscriptionStatusFoo {
    //             state: State::Subscribed,
    //             message: "Subscription success".to_string(),
    //         }
    //     } else {
    //         SubscriptionStatusFoo {
    //             state: State::Subscribed,
    //             message: "Already subscribed".to_string(),
    //         }
    //     };

    //     let subscription_response = SubscriptionResponseFoo {
    //         status: subscription_status,
    //         topic: subscription_request.topic,
    //     };

    //     Ok(subscription_response)
    // }

    // pub async fn unsubscribe(&self, unsubscribe_request: UnsubscribeRequestFoo) -> Result<(), UStatus> {
    //     Ok(())
    // }

    // pub async fn fetch_subscriptions(&self, fetch_subscriptions_request: FetchSubscriptionsRequestFoo) -> Result<FetchSubscriptionsResponseFoo, UStatus> {
    //     todo!()
    // }

    // async fn register_for_notifications(&self, notifications_request: NotificationsRequest) -> Result<(), UStatus> {
    //     todo!()
    // }

    // async fn unregister_for_notifications(&self, notifications_request: NotificationsRequest) -> Result<(), UStatus> {
    //     todo!()
    // }

    pub async fn fetch_subscribers_internal(
        &self,
        fetch_subscribers_request: FetchSubscribersRequestFoo,
    ) -> Result<FetchSubscribersResponseFoo, UStatus> {
        Ok(FetchSubscribersResponseFoo {
            subscribers: self
                .subscription_cache_map
                .lock()
                .await
                .get(&fetch_subscribers_request.topic)
                .cloned()
                .unwrap_or_default(),
        })
    }
}
