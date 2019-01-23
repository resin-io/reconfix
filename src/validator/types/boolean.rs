use serde_json::Value;

use super::super::{scope::ScopedSchema, state::ValidationState};

pub fn validate_as_boolean(scope: &ScopedSchema, data: &Value) -> ValidationState {
    if !data.is_boolean() {
        ValidationState::new_with_error(scope.invalid_error("type"))
    } else {
        ValidationState::new()
    }
}