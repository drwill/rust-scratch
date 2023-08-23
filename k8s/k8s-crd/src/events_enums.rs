
use strum::Display;

#[derive(Debug, PartialEq, Display)]
pub enum EventType {
    Normal,
    Warning,
}

#[derive(Debug, PartialEq, Display)]
pub enum EventReason {
    // Starting,
    // Ready,
    // Recovering,
    // Healthy,
    Unhealthy,
}
