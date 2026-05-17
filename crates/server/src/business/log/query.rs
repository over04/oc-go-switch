use serde::Deserialize;

use crate::common::model::direction::Direction;

#[derive(Debug, Deserialize)]
pub struct LogListQuery {
    pub limit: Option<usize>,
    pub direction: Option<Direction>,
    pub success: Option<bool>,
}
