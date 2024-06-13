use async_trait::async_trait;
use log::trace;
use std::fs::canonicalize;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use up_rust::{UListener, UMessage, UMessageBuilder, UStatus, UTransport, UUri};
use up_transport_vsomeip::UPTransportVsomeip;

const ME_AUTHORITY: &str = "me_authority";
const SERVICE_UE_ID: u16 = 0x4321;
const SERVICE_UE_VERSION_MAJOR: u8 = 1;
const SERVICE_RESOURCE_ID: u16 = 0x0421;

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

    println!("mE_service");

    let crate_dir = env!("CARGO_MANIFEST_DIR");
    // TODO: Make configurable to pass the path to the vsomeip config as a command line argument
    let vsomeip_config = PathBuf::from(crate_dir).join("vsomeip-configs/mE_service.json");
    let vsomeip_config = canonicalize(vsomeip_config).ok();
    trace!("vsomeip_config: {vsomeip_config:?}");

    // There will be a single vsomeip_transport, as there is a connection into device and a streamer
    // TODO: Add error handling if we fail to create a UPTransportVsomeip
    let vsomeip_transport: Arc<dyn UTransport> = Arc::new(
        UPTransportVsomeip::new_with_config(
            &"me_authority".to_string(),
            SERVICE_UE_ID,
            &vsomeip_config.unwrap(),
        )
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
        authority_name: ME_AUTHORITY.to_string(),
        ue_id: SERVICE_UE_ID as u32,
        ue_version_major: SERVICE_UE_VERSION_MAJOR as u32,
        resource_id: SERVICE_RESOURCE_ID as u32,
        ..Default::default()
    };
    let service_request_responder: Arc<dyn UListener> =
        Arc::new(ServiceRequestResponder::new(vsomeip_transport.clone()));
    // TODO: Need to revisit how the vsomeip config file is used in non point-to-point cases
    vsomeip_transport
        .register_listener(
            &source_filter,
            Some(&sink_filter),
            service_request_responder.clone(),
        )
        .await?;

    thread::park();
    Ok(())
}
