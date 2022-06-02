use serde::{
    ser,
    Serialize,
    Serializer,
};

use crate::{
    http::StatusCode,
    types::{
        Headers,
        MultiValueHeaders,
    },
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T: Serialize> {
    #[serde(serialize_with = "serialize_status_code")]
    pub status_code: StatusCode,

    #[serde(serialize_with = "serialize_body")]
    pub body: T,

    #[serde(skip_serializing_if = "Headers::is_empty")]
    pub headers: Headers,

    #[serde(skip_serializing_if = "MultiValueHeaders::is_empty")]
    pub multi_value_headers: MultiValueHeaders,

    pub is_base64_encoded: bool,
}

fn serialize_status_code<S: Serializer>(
    status_code: &StatusCode,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_u16(status_code.as_u16())
}

fn serialize_body<B, S>(
    body: &B,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    B: Serialize,
    S: Serializer,
{
    match serde_json::to_string(body) {
         Ok(s) => serializer.serialize_str(&s),
        Err(e) => Err(<S::Error as ser::Error>::custom(e)),
    }
}
