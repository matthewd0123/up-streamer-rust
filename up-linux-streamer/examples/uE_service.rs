use async_trait::async_trait;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use up_rust::{UListener, UMessage, UMessageBuilder, UPayloadFormat, UStatus, UTransport, UUri};
use up_transport_zenoh::UPClientZenoh;
use zenoh::config::Config;

const SERVICE_AUTHORITY: &str = "linux";
const SERVICE_UE_ID: u16 = 0x1236;
const SERVICE_UE_VERSION_MAJOR: u8 = 1;
const SERVICE_RESOURCE_ID: u16 = 0x0896;

const CLIENT_AUTHORITY: &str = "me_authority";
const CLIENT_UE_ID: u16 = 0x5678;
const CLIENT_UE_VERSION_MAJOR: u8 = 1;
const CLIENT_RESOURCE_ID: u16 = 0;

const REQUEST_TTL: u32 = 1000;

struct ServiceRequestResponder {
    client: Arc<dyn UTransport>,
}
impl ServiceRequestResponder {
    pub fn new(client: Arc<dyn UTransport>) -> Self {
        Self { client }
    }
}
#[async_trait]
impl UListener for ServiceRequestResponder {
    async fn on_receive(&self, msg: UMessage) {
        println!("ServiceRequestResponder: Received a message: {msg:?}");

        let response_msg = UMessageBuilder::response_for_request(msg.attributes.as_ref().unwrap())
            .build()
            .unwrap();
        self.client.send(response_msg).await.unwrap();
    }

    async fn on_error(&self, err: UStatus) {
        println!("ServiceRequestResponder: Encountered an error: {err:?}");
    }
}

#[tokio::main]
async fn main() -> Result<(), UStatus> {
    env_logger::init();

    println!("uE_client");

    // TODO: Probably make somewhat configurable?
    let zenoh_config = Config::default();
    // TODO: Add error handling if we fail to create a UPClientZenoh
    let service: Arc<dyn UTransport> = Arc::new(
        UPClientZenoh::new(zenoh_config, "linux".to_string())
            .await
            .unwrap(),
    );

    let source_filter = UUri {
        authority_name: "*".to_string(),
        ue_id: 0x0000_FFFF,
        ue_version_major: 0xFF,
        resource_id: 0xFFFF,
        ..Default::default()
    };
    let sink_filter = UUri {
        authority_name: SERVICE_AUTHORITY.to_string(),
        ue_id: SERVICE_UE_ID as u32,
        ue_version_major: SERVICE_UE_VERSION_MAJOR as u32,
        resource_id: SERVICE_RESOURCE_ID as u32,
        ..Default::default()
    };

    let service_request_responder: Arc<dyn UListener> =
        Arc::new(ServiceRequestResponder::new(service.clone()));
    service
        .register_listener(
            &source_filter,
            Some(&sink_filter),
            service_request_responder.clone(),
        )
        .await?;

    thread::park();
    Ok(())
}
