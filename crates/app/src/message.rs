use serenity::builder::CreateInteractionResponseData;

pub trait StructuredMessage {
    fn write_into(self, ctx: &mut CreateInteractionResponseData);
}
