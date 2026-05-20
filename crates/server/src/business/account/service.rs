use crate::{
    business::account::dto::list::AccountListEntryDto, common::config::account::AccountConfig,
};

pub fn mask_auth(auth: &str) -> String {
    if auth.len() <= 12 {
        return "***".to_string();
    }
    format!("{}...{}", &auth[..6], &auth[auth.len() - 4..])
}

pub fn account_entry(account: &AccountConfig) -> AccountListEntryDto {
    AccountListEntryDto {
        name: account.name.clone(),
        label: account.label.clone(),
        auth_masked: mask_auth(&account.auth),
    }
}
