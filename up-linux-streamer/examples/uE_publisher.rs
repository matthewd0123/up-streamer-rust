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

use async_trait::async_trait;
use hello_world_protos::hello_world_service::{HelloRequest, HelloResponse};
use protobuf::Message;
use std::sync::Arc;
use std::time::Duration;
use up_rust::{UListener, UMessage, UMessageBuilder, UPayloadFormat, UStatus, UTransport, UUri};
use up_transport_zenoh::UPClientZenoh;
use zenoh::config::{EndPoint, Config};
use std::str::FromStr;

const SERVICE_AUTHORITY: &str = "me_authority";
const SERVICE_UE_ID: u16 = 0x4321;
const SERVICE_UE_VERSION_MAJOR: u8 = 1;
const SERVICE_RESOURCE_ID: u16 = 0x0421;

const PUB_TOPIC_AUTHORITY: &str = "pub_topic";
const PUB_TOPIC_UE_ID: u16 = 0x1236;
const PUB_TOPIC_UE_VERSION_MAJOR: u8 = 1;
const PUB_TOPIC_RESOURCE_ID: u16 = 0x8001;

const REQUEST_TTL: u32 = 1000;

struct ServiceResponseListener;

#[async_trait]
impl UListener for ServiceResponseListener {
    async fn on_receive(&self, msg: UMessage) {
        println!("ServiceResponseListener: Received a message: {msg:?}");

        let Some(payload_bytes) = msg.payload else {
            panic!("No payload bytes");
        };

        let Ok(hello_response) = HelloResponse::parse_from_bytes(&payload_bytes) else {
            panic!("Unable to parse into HelloResponse");
        };

        println!("Here we received response: {hello_response:?}");
    }

    async fn on_error(&self, err: UStatus) {
        println!("ServiceResponseListener: Encountered an error: {err:?}");
    }
}

#[tokio::main]
async fn main() -> Result<(), UStatus> {
    //std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    println!("uE_client");

    // TODO: Probably make somewhat configurable?
    // Create a configuration object
    let mut zenoh_config = Config::default();

    // Specify the address to listen on using IPv4
    let ipv4_endpoint = EndPoint::from_str("tcp/0.0.0.0:7445");

    // Add the IPv4 endpoint to the Zenoh configuration
    zenoh_config.listen.endpoints.push(ipv4_endpoint.expect("FAIL"));
    // TODO: Add error handling if we fail to create a UPClientZenoh
    let client: Arc<dyn UTransport> = Arc::new(
        UPClientZenoh::new(zenoh_config, "linux".to_string())
            .await
            .unwrap(),
    );

    let source = UUri {
        authority_name: PUB_TOPIC_AUTHORITY.to_string(),
        ue_id: PUB_TOPIC_UE_ID as u32,
        ue_version_major: PUB_TOPIC_UE_VERSION_MAJOR as u32,
        resource_id: PUB_TOPIC_RESOURCE_ID as u32,
        ..Default::default()
    };
    let sink = UUri {
        authority_name: SERVICE_AUTHORITY.to_string(),
        ue_id: SERVICE_UE_ID as u32,
        ue_version_major: SERVICE_UE_VERSION_MAJOR as u32,
        resource_id: SERVICE_RESOURCE_ID as u32,
        ..Default::default()
    };

    //let service_response_listener: Arc<dyn UListener> = Arc::new(ServiceResponseListener);
    //client
    //    .register_listener(&sink, Some(&source), service_response_listener)
    //    .await?;

    let mut i = 0;
    loop {
        tokio::time::sleep(Duration::from_millis(1000)).await;

        let hello_request = HelloRequest {
            name: format!("ue_client@i={}", i).to_string(),
            ..Default::default()
        };
        i += 1;

        let request_msg = UMessageBuilder::publish(source.clone())
            .build_with_protobuf_payload(&hello_request)
            .unwrap();
        println!("Sending Publish message:\n{request_msg:?}");

        client.send(request_msg).await?;
    }
}
