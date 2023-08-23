#![deny(rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::let_and_return,
    clippy::let_unit_value,
    clippy::missing_errors_doc,
    clippy::too_many_arguments
)]

pub mod events_enums;
pub mod widget_spec;

use events_enums::{EventType,EventReason};
use widget_spec::{Widget, WidgetSpec};

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
    
    let resource;

    match widgets_client.get_opt(resource_name).await? {
        Some(widget) => {
            debug!("Widget found {widget:?}");
            resource = widget;
        },
        None => {
            debug!("Widget not found; creating one.");
            match widgets_client.create(
                &PostParams::default(),
                &Widget {
                    metadata: ObjectMeta {
                        name: Some(String::from(resource_name)),
                        namespace: Some(String::from(namespace)),
                        ..Default::default()
                    },
                    spec: WidgetSpec {
                        age: Some(0),
                    }
                }
            ).await {
                Ok(widget) => {
                    debug!("Created widget {widget:?}");
                    resource = widget;
                },
                Err(err) => {
                    panic!("Error creating widget due to {err}");
                }
            };
        }
    }

    let events_client: kube::Api<Event> = kube::Api::default_namespaced(client);

    let utc_now = Utc::now();
    let my_event: Event = Event {
        type_: Some(String::from(EventType::Normal.to_string())),
        action: Some(String::from("Manipulated widget")),
        event_time: Some(MicroTime(utc_now)),
        reason: Some(String::from(EventReason::Unhealthy.to_string())),
        message: Some(String::from("Widget crumbled.")),
        reporting_component: Some(String::from("drw-component")),
        reporting_instance: Some(String::from("k8s-crd")),
        count: Some(1),
        first_timestamp: None,
        last_timestamp: None,
        series: None,
        source: None,
        related: None,
        // This is metadata about the object for which the event is raised.
        involved_object: ObjectReference {
            api_version: Some(String::from("stable.example.com/v1")),
            kind: Some(String::from("Widget")),
            namespace: Some(String::from(namespace)),
            name: resource.metadata.name.clone(),
            resource_version: resource.metadata.resource_version.clone(),
            uid: resource.metadata.uid.clone(),
            ..Default::default()
        },
        // This is metadata about this event object.
        metadata: ObjectMeta {
            // This needs to be a unique name for the event, otherwise you'll get a 409 Conflict,
            // so we'll use the resource name and the number of nanoseconds since unix epoch time.
            name: Some(format!("{:?}-{}", resource.metadata.name.clone().unwrap(), utc_now.timestamp_nanos())),
            namespace: Some(String::from(namespace)),
            ..Default::default()
        },
    };

    match events_client.create(&PostParams::default(), &my_event).await {
        Ok(event) => {
            println!("event logged\n{event:#?}");
        },
        Err(err) =>{
            panic!("error logging drw event: {err:?}");
        },
    };

    Ok(())
}
