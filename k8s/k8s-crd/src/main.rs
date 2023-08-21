#![deny(rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::let_and_return,
    clippy::let_unit_value,
    clippy::missing_errors_doc,
    clippy::too_many_arguments
)]

use k8s_openapi::api::core::v1::{Event, EventSource, ObjectReference};
use k8s_openapi::chrono::Utc;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime;
use kube::api::PostParams;
use kube::core::ObjectMeta;
use uuid::Uuid;
use strum::Display;

#[derive(Debug, PartialEq, Display)]
enum EventType {
    Normal,
    //Warning,
}

#[derive(Debug, PartialEq, Display)]
enum EventReason {
    Starting,
    // Ready,
    // Recovering,
    // Healthy,
    // Unhealthy,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = kube::Client::try_default().await?;
    let events_client: kube::Api<Event> = kube::Api::default_namespaced(client);

    let uuid = Uuid::new_v4();
    let namespace = "default";
    let resource_version = "1.0.0";
    let pod_name = "drw-pod";

    let my_event = Event {
        type_: Some(String::from(EventType::Normal.to_string())),
        action: Some(String::from("Ran sample")),
        count: Some(1),
        event_time: Some(MicroTime(Utc::now())),
        first_timestamp: None,
        last_timestamp: None,
        reason: Some(String::from(EventReason::Starting.to_string())),
        message: Some(String::from("drw event sample container started")),
        related: None,
        reporting_component: Some(String::from("drw-component")),
        reporting_instance: Some(String::from("k8s-crd")),
        series: None,
        source: Some(EventSource {
            component: Some(String::from("drw-component")),
            host: Some(String::from("drw-host1")),
        }),
        involved_object: ObjectReference {
            api_version: Some(String::from("v1")),
            field_path: None,
            kind: Some(String::from("Pod")),
            name: Some(String::from(pod_name)),
            namespace: Some(String::from(namespace)),
            resource_version: Some(String::from(resource_version)),
            uid: Some(String::from(uuid.to_string())),
        },
        metadata: ObjectMeta {
            name: Some(String::from(pod_name)),
            creation_timestamp: None,
            annotations: None,
            deletion_grace_period_seconds: None,
            deletion_timestamp: None,
            finalizers: None,
            generate_name: None,
            labels: None,
            generation: None,
            managed_fields: None,
            namespace: Some(String::from(namespace)),
            owner_references: None,
            resource_version: Some(String::from(resource_version)),
            self_link: None,
            uid: Some(String::from(uuid.to_string())),
        },
    };

    match events_client.create(&PostParams::default(), &my_event).await {
        Ok(_) => {
            println!("drw event logged");
        },
        Err(err) =>{
            eprintln!("error logging drw event: {err:?}");
        },
    };

    Ok(())
}
