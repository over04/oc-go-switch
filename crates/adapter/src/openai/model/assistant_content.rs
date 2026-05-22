use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OpenAiAssistantContent {
    #[default]
    Missing,
    Null,
    Text(String),
}

impl OpenAiAssistantContent {
    pub fn is_missing(&self) -> bool {
        matches!(self, Self::Missing)
    }
}

impl<'de> Deserialize<'de> for OpenAiAssistantContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Option::<String>::deserialize(deserializer)?;
        Ok(match value {
            Some(text) => Self::Text(text),
            None => Self::Null,
        })
    }
}

impl Serialize for OpenAiAssistantContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Missing => serializer.serialize_unit(),
            Self::Null => serializer.serialize_none(),
            Self::Text(text) => serializer.serialize_str(text),
        }
    }
}
