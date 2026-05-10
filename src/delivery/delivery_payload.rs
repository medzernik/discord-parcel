use serde::Deserialize;

use crate::delivery::delivery_codes::DeliveryStatus;

#[derive(Debug, Deserialize)]
pub struct DeliveryEvent {
    date: String,
    event: String,
}

#[derive(Debug, Deserialize)]
pub struct Delivery {
    carrier_code: String,
    description: String,
    events: Vec<DeliveryEvent>,
    status_code: DeliveryStatus,
    tracking_number: String,
}

#[derive(Debug, Deserialize)]
pub struct Deliveries {
    deliveries: Vec<Delivery>,
    success: bool,
}
