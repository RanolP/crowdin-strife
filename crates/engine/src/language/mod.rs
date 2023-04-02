mod infer;

#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    English,
    Korean,
}

impl Language {
    pub fn is_source(&self) -> bool {
        self == &Language::English
    }

    pub fn id(&self) -> &str {
        match self {
            Language::English => "en-US",
            Language::Korean => "ko-KR",
        }
    }

    pub fn from_id(id: &str) -> Option<Language> {
        match id {
            "en-US" => Some(Language::English),
            "ko-KR" => Some(Language::Korean),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Language::English => "English (US)",
            Language::Korean => "Korean",
        }
    }
}
