use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum SerializedFormat {
    Bool,
    Number,
    String(StringFormat),
    List(Box<SerializedFormat>),
    Set(Box<SerializedFormat>),
    Map(StringFormat, Box<SerializedFormat>),
    Optional(Box<SerializedFormat>),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum StringFormat {
    Any,
    Pattern(&'static str),
    Choices(&'static [&'static str]),
}

#[derive(Debug, Clone, Serialize)]
pub struct FeatureDefinition {
    pub key: &'static str,
    pub description: String,
    pub format: SerializedFormat,
    pub value: Value,
    pub default: Value,
    pub modified_at: Option<DateTime<Utc>>,
    pub modified_by: Option<String>,
}
