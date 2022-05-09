pub mod get;
pub mod set;

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::Get;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailSubmission<State = Get> {
    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(rename = "identityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    identity_id: Option<String>,

    #[serde(rename = "emailId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    email_id: Option<String>,

    #[serde(rename = "threadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    thread_id: Option<String>,

    #[serde(rename = "envelope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    envelope: Option<Envelope>,

    #[serde(rename = "sendAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    send_at: Option<DateTime<Utc>>,

    #[serde(rename = "undoStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    undo_status: Option<UndoStatus>,

    #[serde(rename = "deliveryStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_status: Option<HashMap<String, DeliveryStatus>>,

    #[serde(rename = "dsnBlobIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    dsn_blob_ids: Option<Vec<String>>,

    #[serde(rename = "mdnBlobIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    mdn_blob_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    #[serde(rename = "mailFrom")]
    mail_from: Address,

    #[serde(rename = "rcptTo")]
    rcpt_to: Vec<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address<State = Get> {
    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    email: String,
    parameters: Option<HashMap<String, Option<String>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UndoStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "final")]
    Final,
    #[serde(rename = "canceled")]
    Canceled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryStatus {
    #[serde(rename = "smtpReply")]
    smtp_reply: String,

    #[serde(rename = "delivered")]
    delivered: Delivered,

    #[serde(rename = "displayed")]
    displayed: Displayed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Delivered {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Displayed {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "yes")]
    Yes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmailSubmissionProperty {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "identityId")]
    IdentityId,
    #[serde(rename = "emailId")]
    EmailId,
    #[serde(rename = "threadId")]
    ThreadId,
    #[serde(rename = "envelope")]
    Envelope,
    #[serde(rename = "sendAt")]
    SendAt,
    #[serde(rename = "undoStatus")]
    UndoStatus,
    #[serde(rename = "deliveryStatus")]
    DeliveryStatus,
    #[serde(rename = "dsnBlobIds")]
    DsnBlobIds,
    #[serde(rename = "mdnBlobIds")]
    MdnBlobIds,
}