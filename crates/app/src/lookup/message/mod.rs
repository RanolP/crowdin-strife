use engine::{db::TmEntryPair, language::Language};
use serenity::{builder::CreateInteractionResponseData, model::prelude::component::ButtonStyle};

use crate::message::StructuredMessage;

pub struct LookupResult {
    pub query: String,

    pub source_language: Language,
    pub target_language: Language,

    pub game_version: String,

    pub entries: Vec<TmEntryPair>,

    pub page: usize,
    pub total_pages: usize,
}

impl StructuredMessage for LookupResult {
    fn write_boxed_into<'a, 'b>(
        self: Box<Self>,
        mut ctx: &'b mut CreateInteractionResponseData<'a>,
    ) -> &'b mut CreateInteractionResponseData<'a> {
        let is_paged = self.total_pages > 1;
        let page = self.page;
        let total_pages = self.total_pages;

        ctx = ctx.embed(|embed| {
            let mut embed = embed.title(format!("\"{}\" at {}", self.query, self.game_version,));
            for entry in self.entries {
                embed = embed.fields(vec![
                    ("key", format!("[i](?test=true \"{}\")", entry.key), true),
                    (
                        self.source_language.name(),
                        format!("{}", entry.source.content),
                        true,
                    ),
                    (
                        self.target_language.name(),
                        format!(
                            "{}",
                            entry
                                .targets
                                .get(0)
                                .map(|entry| &*entry.content)
                                .unwrap_or("*번역 없음*")
                        ),
                        true,
                    ),
                ]);
            }
            embed.footer(|footer| {
                footer.text(format!("페이지 {} / {}", self.page, self.total_pages))
            })
        });

        if is_paged {
            ctx = ctx.components(|components| {
                components.create_action_row(|mut action_row| {
                    if page > 1 {
                        action_row = action_row.create_button(|button| {
                            button
                                .label("이전 페이지")
                                .style(ButtonStyle::Secondary)
                                .custom_id("prev")
                        })
                    }
                    if page < total_pages {
                        action_row = action_row.create_button(|button| {
                            button
                                .label("다음 페이지")
                                .style(ButtonStyle::Secondary)
                                .custom_id("next")
                        })
                    }
                    action_row
                })
            });
            ctx
        } else {
            ctx
        }
    }
}
