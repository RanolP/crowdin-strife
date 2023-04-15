use serenity::{
    builder::{CreateInteractionResponseData, EditInteractionResponse},
    model::prelude::component::ButtonStyle,
};

pub type BoxedStructuredMessage = Box<dyn StructuredMessage + Sync + Send>;

pub trait StructuredMessage {
    fn content(&self) -> Option<String> {
        None
    }

    fn components(&self) -> Vec<ActionRow> {
        vec![]
    }

    fn embed(&self) -> Option<Embed> {
        None
    }
}

impl StructuredMessage for String {
    fn content(&self) -> Option<String> {
        Some(self.clone())
    }
}

pub struct ActionRow {
    pub items: Vec<Component>,
}

pub enum Component {
    Button(ComponentButton),
}

pub struct ComponentButton {
    pub label: String,
    pub style: Option<ButtonStyle>,
    pub action: ButtonAction,
    pub disabled: Option<bool>,
}
pub enum ButtonAction {
    Id(String),
    Url(String),
}

pub struct Embed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub fields: Vec<EmbedField>,
    pub footer: Option<EmbedFooter>,
}

pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

pub struct EmbedFooter {
    pub text: Option<String>,
}
pub trait Render {
    fn render(&mut self, message: &dyn StructuredMessage) -> &mut Self;
}

impl Render for CreateInteractionResponseData<'_> {
    fn render(&mut self, message: &dyn StructuredMessage) -> &mut Self {
        if let Some(content) = message.content() {
            self.content(content);
        }
        let action_rows = message.components();
        self.components(move |components| {
            for ActionRow { items } in action_rows {
                components.create_action_row(|action_row| {
                    for component in items {
                        match component {
                            Component::Button(ComponentButton {
                                label,
                                style,
                                action,
                                disabled,
                            }) => action_row.create_button(|button| {
                                button.label(label);
                                if let Some(style) = style {
                                    button.style(style);
                                }
                                match action {
                                    ButtonAction::Id(id) => button.custom_id(id),
                                    ButtonAction::Url(url) => button.url(url),
                                };
                                if let Some(disabled) = disabled {
                                    button.disabled(disabled);
                                }
                                button
                            }),
                        };
                    }
                    action_row
                });
            }
            components
        });
        if let Some(Embed {
            title,
            description,
            fields,
            footer,
        }) = message.embed()
        {
            self.embed(|embed| {
                if let Some(title) = title {
                    embed.title(title);
                }
                if let Some(description) = description {
                    embed.description(description);
                }
                if !fields.is_empty() {
                    embed.fields(
                        fields
                            .into_iter()
                            .map(|field| (field.name, field.value, field.inline)),
                    );
                }
                if let Some(EmbedFooter { text }) = footer {
                    embed.footer(|footer| {
                        if let Some(text) = text {
                            footer.text(text);
                        }
                        footer
                    });
                }
                embed
            });
        }
        self
    }
}

impl Render for EditInteractionResponse {
    fn render(&mut self, message: &dyn StructuredMessage) -> &mut Self {
        if let Some(content) = message.content() {
            self.content(content);
        }
        let action_rows = message.components();
        self.components(move |components| {
            for ActionRow { items } in action_rows {
                components.create_action_row(|action_row| {
                    for component in items {
                        match component {
                            Component::Button(ComponentButton {
                                label,
                                style,
                                action,
                                disabled,
                            }) => action_row.create_button(|button| {
                                button.label(label);
                                if let Some(style) = style {
                                    button.style(style);
                                }
                                match action {
                                    ButtonAction::Id(id) => button.custom_id(id),
                                    ButtonAction::Url(url) => button.url(url),
                                };
                                if let Some(disabled) = disabled {
                                    button.disabled(disabled);
                                }
                                button
                            }),
                        };
                    }
                    action_row
                });
            }
            components
        });
        if let Some(Embed {
            title,
            description,
            fields,
            footer,
        }) = message.embed()
        {
            self.embed(|embed| {
                if let Some(title) = title {
                    embed.title(title);
                }
                if let Some(description) = description {
                    embed.description(description);
                }
                if !fields.is_empty() {
                    embed.fields(
                        fields
                            .into_iter()
                            .map(|field| (field.name, field.value, field.inline)),
                    );
                }
                if let Some(EmbedFooter { text }) = footer {
                    embed.footer(|footer| {
                        if let Some(text) = text {
                            footer.text(text);
                        }
                        footer
                    });
                }
                embed
            });
        }
        self
    }
}
