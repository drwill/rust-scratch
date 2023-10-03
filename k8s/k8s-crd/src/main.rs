#![deny(rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::default_trait_access,
    clippy::let_unit_value,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
    clippy::let_underscore_untyped,
    let_underscore_drop
)]

pub mod events_enums;
pub mod widget_spec;

use events_enums::{EventType,EventReason};
use widget_spec::Widget;

use k8s_openapi::api::core::v1::{Event, ObjectReference};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime;
use k8s_openapi::chrono::Utc;
use kube::api::PostParams;
use kube::core::ObjectMeta;
use tracing::debug;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let namespace = "default";
    let resource_name = "drw-widget1";
    
    let client = kube::Client::try_default().await?;
    let widgets_client: kube::Api<Widget> = kube::Api::namespaced(client.clone(), namespace);

    let resource = match widgets_client.get_opt(resource_name).await {
        Ok(Some(widget)) => {
            debug!("Widget found {widget:?}");
            widget
        },
        Ok(None) => {
            panic!("Widget {resource_name} not found.");
        },
        Err(err) => {
            panic!("Error getting widget {resource_name} due to {err:?}");
        }
    };

    let events_client: kube::Api<Event> = kube::Api::namespaced(client, namespace);

    let utc_now = Utc::now();
    let my_event: Event = Event {
        type_: Some(EventType::Normal.to_string()),
        action: Some("Manipulated widget".to_owned()),
        event_time: Some(MicroTime(utc_now)),
        reason: Some(EventReason::Unhealthy.to_string()),
        message: Some("Widget crumbled.".to_owned()),
        reporting_component: Some("drw-component".to_owned()),
        reporting_instance: Some("k8s-crd".to_owned()),
        count: Some(1),
        first_timestamp: None,
        last_timestamp: None,
        series: None,
        source: None,
        related: None,
        // This is metadata about the object for which the event is raised.
        involved_object: ObjectReference {
            api_version: Some("stable.example.com/v1".to_owned()),
            kind: Some("Widget".to_owned()),
            namespace: Some(namespace.to_owned()),
            name: resource.metadata.name.clone(),
            resource_version: resource.metadata.resource_version.clone(),
            uid: resource.metadata.uid.clone(),
            ..Default::default()
        },
        // This is metadata about this event object.
        metadata: ObjectMeta {
            // This needs to be a unique name for the event, otherwise you'll get a 409 Conflict,
            // so we'll use the resource name and the number of nanoseconds since unix epoch time.
            name: Some(format!(
                "{:?}-{}",
                resource.metadata.name.unwrap_or(String::new()),
                utc_now.timestamp_nanos())),
            namespace: Some(namespace.to_owned()),
            ..Default::default()
        },
    };

    let event =  events_client.create(&PostParams::default(), &my_event).await?;
    println!("event logged\n{event:?}");

    Ok(())
}
