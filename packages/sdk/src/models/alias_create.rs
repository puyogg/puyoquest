/*
 * PPQ API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AliasCreate : Request body type for alias creation. The alias should be given as a path param.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AliasCreate {
    #[serde(rename = "char_id")]
    pub char_id: String,
    #[serde(rename = "internal")]
    pub internal: bool,
    #[serde(rename = "card_type")]
    pub card_type: models::CardType,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl AliasCreate {
    /// Request body type for alias creation. The alias should be given as a path param.
    pub fn new(char_id: String, internal: bool, card_type: models::CardType) -> AliasCreate {
        AliasCreate {
            char_id,
            internal,
            card_type,
            updated_at: None,
        }
    }
}

