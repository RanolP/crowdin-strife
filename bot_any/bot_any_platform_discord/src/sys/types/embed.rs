use serde::Serialize;

#[derive(Serialize)]
pub struct Embed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    // timestamp?	ISO8601 timestamp	timestamp of embed content
    pub color: u32,
    // footer?	embed footer object	footer information
    // image?	embed image object	image information
    // thumbnail?	embed thumbnail object	thumbnail information
    // video?	embed video object	video information
    // provider?	embed provider object	provider information
    // author?	embed author object	author information
    // fields?	array of embed field objects	fields information
}
