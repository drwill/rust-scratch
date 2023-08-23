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
use k8s_openapi::chrono::Utc;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::MicroTime;
use kube::api::PostParams;
use kube::core::ObjectMeta;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let namespace = "default";
    let resource_name = "drw-widget1";
    
    let client = kube::Client::try_default().await?;
    let widgets_client: kube::Api<Widget> = kube::Api::namespaced(client.clone(), namespace);
    
    let resource_version;
    let uid;

    match widgets_client.get_opt(resource_name).await? {
        Some(widget) => {
            println!("Widget found {widget:?}");
            uid = widget.metadata.uid.unwrap_or_default();
            resource_version = widget.metadata.resource_version.unwrap_or_default();
        },
        None => {
            println!("Widget not found; creating one.");
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
                    println!("Created widget {widget:?}");
                    uid = widget.metadata.uid.unwrap_or_default();
                    resource_version = widget.metadata.resource_version.unwrap_or_default();
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
            field_path: None,
            kind: Some(String::from("Widget")),
            namespace: Some(String::from(namespace)),
            name: Some(String::from(resource_name)),
            resource_version: Some(resource_version.clone()),
            uid: Some(uid.clone()),
            ..Default::default()
        },
        // This is metadata about this event object.
        metadata: ObjectMeta {
            // This needs to be a unique name for the event, otherwise you'll get a 409 Conflict,
            // so we'll use the resource name and the number of nanoseconds since unix epoch time.
            name: Some(format!("{}-{}", resource_name, utc_now.timestamp_nanos())),
            namespace: Some(String::from(namespace)),
            ..Default::default()
        },
    };

    match events_client.create(
        &PostParams {
            dry_run: false,
            ..Default::default()
        },
        &my_event
    ).await {
        Ok(event) => {
            println!("drw event logged {event:?}");
        },
        Err(err) =>{
            eprintln!("error logging drw event: {err:?}");
        },
    };

    Ok(())
}
