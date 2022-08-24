use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::Snowflake;

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u32)]
pub enum ApplicationCommandKind {
    ChatInput = 1,
    User = 2,
    Message = 3,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationCommand {
    pub id: Option<Snowflake>,
    #[serde(rename = "type")]
    pub kind: Option<ApplicationCommandKind>,
    pub application_id: Option<Snowflake>,
    pub guild_id: Option<Snowflake>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<ApplicationCommandOption>,
    pub description: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u32)]
pub enum ApplicationCommandOptionKind {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    /// Any integer between -2^53 and 2^53
    Integer = 4,
    Boolean = 5,
    User = 6,
    /// Includes all channel types + categories
    Channel = 7,
    Role = 8,
    /// Includes users and roles
    Mentionable = 9,
    /// Any double between -2^53 and 2^53
    Number = 10,
    /// attachment object
    Attachment = 11,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationCommandOption {
    /// Type of option
    #[serde(rename = "type")]
    pub kind: ApplicationCommandOptionKind,

    /// 1-32 character name
    pub name: String,

    /// 1-100 character description
    pub description: Option<String>,

    /// If the parameter is required or optional--default false
    pub required: Option<bool>,

    // Choices for STRING, INTEGER, and NUMBER types for the user to pick from, max 25
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub choices: Vec<ApplicationCommandOptionChoice>,

    /// If the option is a subcommand or subcommand group type, these nested options will be the parameters
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<ApplicationCommandOption>,
    // channel_types?	array of channel types	If the option is a channel type, the channels shown will be restricted to these types

    // min_value?	integer for INTEGER options, double for NUMBER options	If the option is an INTEGER or NUMBER type, the minimum value permitted

    // max_value?	integer for INTEGER options, double for NUMBER options	If the option is an INTEGER or NUMBER type, the maximum value permitted

    // min_length?	integer	For option type STRING, the minimum allowed length (minimum of 0, maximum of 6000)

    // max_length?	integer	For option type STRING, the maximum allowed length (minimum of 1, maximum of 6000)

    // autocomplete? *	boolean	If autocomplete interactions are enabled for this STRING, INTEGER, or NUMBER type option
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationCommandOptionChoice {
    /// 1-100 character choice name
    pub name: String,

    /// Value for the choice, up to 100 characters if string
    pub value: ApplicationCommandOptionChoiceValue,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApplicationCommandOptionChoiceValue {
    String(String),
    Int(i64),
    Double(f64),
}
