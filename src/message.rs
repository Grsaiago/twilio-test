use std::collections::HashMap;

use axum::{extract::Query, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use headers::ContentType;
use serde::{Deserialize, Serialize};
use tracing::info;
use twilio::twiml::{self, Twiml};

#[derive(Deserialize, Serialize, Debug)]
pub struct WhatsappMessage {
    #[serde(rename(deserialize = "MessageSid"))]
    pub message_sid: String, // MessageSid: Unique identifier for the message.

    //#[serde(rename(deserialize = "SmsSid"))]
    //pub sms_sid: String, // SmsSid: Same value as MessageSid (deprecated).
    //
    //#[serde(rename(deserialize = "SmsMessageSid"))]
    //pub sms_message_sid: String, // SmsMessageSid: Same value as MessageSid (deprecated).
    //
    #[serde(rename(deserialize = "AccountSid"))]
    pub account_sid: String, // AccountSid: ID of the associated account.

    #[serde(rename(deserialize = "MessagingServiceSid"))]
    pub messaging_service_sid: String, // MessagingServiceSid: ID of the messaging service.

    #[serde(rename(deserialize = "From"))]
    pub from: String, // From: Sender's phone number or channel address.

    #[serde(rename(deserialize = "To"))]
    pub to: String, // To: Recipient's phone number or channel address.

    #[serde(rename(deserialize = "Body"))]
    pub body: String, // Body: Text body of the message.

    #[serde(rename(deserialize = "NumMedia"))]
    pub num_media: u32, // NumMedia: Number of media items.

    #[serde(rename(deserialize = "NumSegments"))]
    pub num_segments: u32, // NumSegments: Number of message segments.

    // whatsApp specific fields start
    #[serde(rename(deserialize = "ProfileName"))]
    pub profile_name: String, // ProfileName: The sender's WhatsApp profile name.

    #[serde(rename(deserialize = "WaId"))]
    pub wa_id: String, // WaId: The sender's WhatsApp ID (typically a phone number).

    #[serde(rename(deserialize = "Forwarded"))]
    pub forwarded: bool, // Forwarded: True if the message has been forwarded once.

    #[serde(rename(deserialize = "FrequentlyForwarded"))]
    pub frequently_forwarded: bool, // FrequentlyForwarded: True if the message has been frequently forwarded.

    #[serde(rename(deserialize = "ButtonText"))]
    pub button_text: String, // ButtonText: The text of a Quick reply button.
}

pub async fn handle_message(Json(message): Json<HashMap<String, String>>) -> impl IntoResponse {
    let json_pretty = serde_json::to_string_pretty(&message).unwrap();
    info!("Pinged handle_message_post Twiml message: {}", json_pretty);
    let res = Twiml::new()
        .add(&twiml::Message {
            //txt: format!("Você apertou a opção: {}", message.button_text),
            txt: "Você apertou a alguma opção".to_string(),
        })
        .as_twiml();
    (TypedHeader(ContentType::xml()), res)
}

pub async fn handle_get_message(
    Query(message): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let json_pretty = serde_json::to_string_pretty(&message).unwrap();
    info!("Pinged handle_message_get Twiml message: {}", json_pretty);
    let res = Twiml::new()
        .add(&twiml::Message {
            //txt: format!("Você apertou a opção: {}", message.button_text),
            txt: "Você apertou a alguma opção".to_string(),
        })
        .as_twiml();
    (TypedHeader(ContentType::xml()), res)
}
