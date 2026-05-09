// State of parcels as referenced in: https://parcelapp.net/help/api-view-deliveries.html
//
// 0 - completed delivery.
// 1 - frozen delivery. There were no updates for a long time or something else makes the app believe that it will never be updated in the future.
// 2 - delivery in transit.
// 3 - delivery expecting a pickup by the recipient.
// 4 - out for delivery.
// 5 - delivery not found.
// 6 - failed delivery attempt.
// 7 - delivery exception, something is wrong and requires your attention.
// 8 - carrier has received information about a package, but has not physically received it yet.

pub enum DeliveryStatus {
    Completed,
    Frozen,
    Transit,
    RecepientToPickup,
    OutForDelivery,
    DeliveryNotFound,
    FailedDeliveryAttempt,
    ActionRequired,
    LabelCreated,
}
