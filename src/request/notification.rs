/// The `aps` notification content builders
mod default;
mod options;
mod web;

use std::borrow::Cow;
pub use self::default::{DefaultAlert, DefaultNotificationBuilder, DefaultSound};
pub use self::options::{CollapseId, NotificationOptions, Priority, PushType};
pub use self::web::{WebNotificationBuilder, WebPushAlert};

use crate::request::payload::Payload;

pub trait NotificationBuilder<'a> {
    /// Generates the request payload to be send with the `Client`.
    fn build(self, device_token: impl Into<Cow<'a, str>>, options: NotificationOptions<'a>) -> Payload<'a>;
}
