use serenity::builder::CreateInteractionResponseData;

pub type StructuredMessageBox = Box<dyn StructuredMessage + Sync + Send>;

pub trait StructuredMessage {
    fn write_boxed_into<'a, 'b>(
        self: Box<Self>,
        ctx: &'b mut CreateInteractionResponseData<'a>,
    ) -> &'b mut CreateInteractionResponseData<'a>;

    fn write_into<'a, 'b>(
        self,
        ctx: &'b mut CreateInteractionResponseData<'a>,
    ) -> &'b mut CreateInteractionResponseData<'a>
    where
        Self: Sized,
    {
        Box::new(self).write_boxed_into(ctx)
    }
}

impl StructuredMessage for String {
    fn write_boxed_into<'a, 'b>(
        self: Box<Self>,
        ctx: &'b mut CreateInteractionResponseData<'a>,
    ) -> &'b mut CreateInteractionResponseData<'a> {
        ctx.content(self)
    }
}
