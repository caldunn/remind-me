pub struct NotificationRequest {}

trait SendNotification {
    fn send() -> bool;
}

pub struct Email {
    address: String,
}

impl SendNotification for Email {
    fn send() -> bool {
        true
    }
}

/*
pub enum ComType {
    Email(address: String),
    Webhook(url: String),
    SMS(number: String)
}
*/
