use serde::{Deserialize, Serialize};

use crate::common::config::filter_action::FilterAction;

/// 单个模型的图片处理规则。
///
/// `model` 使用精确匹配，避免通配规则误伤相近模型名。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageFilterModel {
    /// 模型 ID（精确匹配）。
    pub model: String,
    /// 对该模型的图片处理方式。
    #[serde(flatten)]
    pub action: FilterAction,
}

#[cfg(test)]
mod tests {
    use crate::common::config::{
        filter_action::FilterAction, image_filter_model::ImageFilterModel,
    };

    #[test]
    fn replace_action_carries_replacement_text() -> Result<(), Box<dyn std::error::Error>> {
        let yaml = r#"
model: claude-3-haiku
action: replace
replacement: "[Image not supported]"
"#;

        let rule: ImageFilterModel = serde_yaml::from_str(yaml)?;

        assert_eq!(rule.model, "claude-3-haiku");
        assert_eq!(
            rule.action,
            FilterAction::Replace {
                replacement: "[Image not supported]".to_string()
            }
        );
        Ok(())
    }

    #[test]
    fn remove_action_has_no_variant_payload() -> Result<(), Box<dyn std::error::Error>> {
        let rule = ImageFilterModel {
            model: "deepseek-v4-pro".to_string(),
            action: FilterAction::Remove,
        };

        let yaml = serde_yaml::to_string(&rule)?;

        assert!(yaml.contains("action: remove"));
        assert!(!yaml.contains("replacement"));
        Ok(())
    }
}
