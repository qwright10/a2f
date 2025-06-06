use std::borrow::Cow;
use crate::request::notification::{NotificationBuilder, NotificationOptions};
use crate::request::payload::{APSAlert, APSSound, Payload, APS};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct WebPushAlert<'a> {
    pub title: Cow<'a, str>,
    pub body: Cow<'a, str>,
    pub action: Cow<'a, str>,
}

/// A builder to create a simple APNs notification payload.
///
/// # Example
///
/// ```rust
/// # use a2::request::notification::{NotificationBuilder, WebNotificationBuilder, WebPushAlert};
/// # use a2::request::payload::PayloadLike;
/// # fn main() {
/// let mut builder = WebNotificationBuilder::new(WebPushAlert {title: "Hello".into(), body: "World".into(), action: "View".into()}, &["arg1"]);
/// builder.set_sound("prööt");
/// let payload = builder.build("device_id", Default::default())
///    .to_json_string().unwrap();
/// # }
/// ```
pub struct WebNotificationBuilder<'a> {
    alert: WebPushAlert<'a>,
    sound: Option<Cow<'a, str>>,
    url_args: Cow<'a, [Cow<'a, str>]>,
}

impl<'a> WebNotificationBuilder<'a> {
    /// Creates a new builder with the minimum amount of content.
    ///
    /// ```rust
    /// # use a2::request::notification::{WebNotificationBuilder, NotificationBuilder, WebPushAlert};
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = WebNotificationBuilder::new(WebPushAlert {title: "Hello".into(), body: "World".into(), action: "View".into()}, &["arg1"]);
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"Hello\",\"body\":\"World\",\"action\":\"View\"},\"url-args\":[\"arg1\"]}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn new<A, E>(alert: WebPushAlert<'a>, url_args: A) -> WebNotificationBuilder<'a>
    where
        A: AsRef<[E]>,
        E: Into<Cow<'a, str>> + Clone,
    {
        WebNotificationBuilder {
            alert,
            sound: None,
            url_args: url_args
                .as_ref()
                .into_iter()
                .cloned()
                .map(|arg| arg.into())
                .collect::<Vec<_>>()
                .into(),
        }
    }

    /// File name of the custom sound to play when receiving the notification.
    ///
    /// ```rust
    /// # use a2::request::notification::{WebNotificationBuilder, NotificationBuilder, WebPushAlert};
    /// # use a2::request::payload::PayloadLike;
    /// # fn main() {
    /// let mut builder = WebNotificationBuilder::new(WebPushAlert {title: "Hello".into(), body: "World".into(), action: "View".into()}, &["arg1"]);
    /// builder.set_sound("meow");
    /// let payload = builder.build("token", Default::default());
    ///
    /// assert_eq!(
    ///     "{\"aps\":{\"alert\":{\"title\":\"Hello\",\"body\":\"World\",\"action\":\"View\"},\"sound\":\"meow\",\"url-args\":[\"arg1\"]}}",
    ///     &payload.to_json_string().unwrap()
    /// );
    /// # }
    /// ```
    pub fn set_sound(&mut self, sound: impl Into<Cow<'a, str>>) -> &mut Self {
        self.sound = Some(sound.into());
        self
    }
}

impl<'a> NotificationBuilder<'a> for WebNotificationBuilder<'a> {
    fn build(self, device_token: impl Into<Cow<'a, str>>, options: NotificationOptions<'a>) -> Payload<'a> {
        Payload {
            aps: APS {
                alert: Some(APSAlert::WebPush(self.alert)),
                badge: None,
                sound: self.sound.map(APSSound::Sound),
                content_available: None,
                category: None,
                mutable_content: None,
                url_args: Some(self.url_args),
            },
            device_token: device_token.into(),
            options,
            data: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::payload::PayloadLike;
    use serde_json::Value;

    #[test]
    fn test_webpush_notification() {
        let payload = WebNotificationBuilder::new(
            WebPushAlert {
                action: "View".into(),
                title: "Hello".into(),
                body: "world".into(),
            },
            &["arg1"],
        )
        .build("device-token", Default::default())
        .to_json_string()
        .unwrap();

        let expected_payload = json!({
            "aps": {
                "alert": {
                    "title": "Hello",
                    "body": "world",
                    "action": "View",
                },
                "url-args": ["arg1"]
            }
        });

        assert_eq!(expected_payload, serde_json::from_str::<Value>(&payload).unwrap());
    }
}
