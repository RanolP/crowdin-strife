use engine::{
    db::{MinecraftPlatform, TmEntryPair},
    language::Language,
};
use serenity::model::prelude::component::ButtonStyle;

use crate::{
    e2k_base::serialize,
    message::{
        ActionRow, ButtonAction, Component, ComponentButton, Embed, EmbedField, EmbedFooter,
        StructuredMessage,
    },
    msgdata::encode_msgdata,
};

pub struct LookupResult {
    pub query: String,

    pub source_language: Language,
    pub target_language: Language,

    pub game_version: String,
    pub platform: MinecraftPlatform,

    pub entries: Vec<TmEntryPair>,

    pub page: usize,
    pub total_pages: usize,
}

impl StructuredMessage for LookupResult {
    fn embed(&self) -> Option<crate::message::Embed> {
        let mut source_entries = Vec::new();
        let mut target_entries = Vec::new();

        for entry in &self.entries {
            source_entries.push(format!(
                "{}{}",
                entry.source.content.clone(),
                encode_msgdata(entry.key.clone())
            ));
            target_entries.push(format!(
                "{}{}",
                entry
                    .targets
                    .get(0)
                    .map(|entry| &*entry.content)
                    .unwrap_or("*번역 없음*"),
                encode_msgdata(entry.key.clone())
            ));
        }

        Some(Embed {
            title: Some(format!("▷ {}", self.query)),
            description: Some(format!(
                "{}{}",
                self.game_version,
                encode_msgdata(serialize(
                    self.platform.clone(),
                    self.source_language.clone(),
                    self.target_language.clone(),
                    self.query.clone(),
                    (self.page - 1).try_into().unwrap(),
                    self.total_pages.try_into().unwrap(),
                ))
            )),
            fields: vec![
                EmbedField {
                    name: self.source_language.name().to_string(),
                    value: source_entries.join("\n"),
                    inline: true,
                },
                EmbedField {
                    name: self.target_language.name().to_string(),
                    value: target_entries.join("\n"),
                    inline: true,
                },
            ],
            footer: Some(EmbedFooter {
                text: Some(format!("페이지 {} / {}", self.page, self.total_pages,)),
            }),
        })
    }

    fn components(&self) -> Vec<ActionRow> {
        let is_paged = self.total_pages > 1;
        let page = self.page;
        let total_pages = self.total_pages;

        if !is_paged {
            return vec![];
        }

        vec![ActionRow {
            items: vec![
                Component::Button(ComponentButton {
                    label: "이전 페이지".to_string(),
                    style: Some(ButtonStyle::Secondary),
                    action: ButtonAction::Id("prev".to_string()),
                    disabled: Some(page == 1),
                }),
                Component::Button(ComponentButton {
                    label: "다음 페이지".to_string(),
                    style: Some(ButtonStyle::Secondary),
                    action: ButtonAction::Id("next".to_string()),
                    disabled: Some(page == total_pages),
                }),
            ],
        }]
    }
}
