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
    pub application_id: Snowflake,
    pub guild_id: Option<Snowflake>,
    pub name: String,
    pub options: Option<Vec<ApplicationCommandOption>>,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApplicationCommandOption {
    /// Type of option
    // type: one of application command option type

    /// 1-32 character name
    name: String,
    /// 1-100 character description
    description: String,
    /// If the parameter is required or optional--default false
    required: Option<bool>,
    // choices?	array of application command option choice	Choices for STRING, INTEGER, and NUMBER types for the user to pick from, max 25
    // options?	array of application command option	If the option is a subcommand or subcommand group type, these nested options will be the parameters
    // channel_types?	array of channel types	If the option is a channel type, the channels shown will be restricted to these types
    // min_value?	integer for INTEGER options, double for NUMBER options	If the option is an INTEGER or NUMBER type, the minimum value permitted
    // max_value?	integer for INTEGER options, double for NUMBER options	If the option is an INTEGER or NUMBER type, the maximum value permitted
    // min_length?	integer	For option type STRING, the minimum allowed length (minimum of 0, maximum of 6000)
    // max_length?	integer	For option type STRING, the maximum allowed length (minimum of 1, maximum of 6000)
    // autocomplete? *	boolean	If autocomplete interactions are enabled for this STRING, INTEGER, or NUMBER type option
}
