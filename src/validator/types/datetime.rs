use chrono;
use serde_json::Value;

use super::{
    super::{scope::ScopedSchema, state::ValidationState},
    string::validate_as_string,
};

pub fn validate_as_datetime(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let mut state = validate_as_string(scope, data);

    if state.is_valid()
        && chrono::DateTime::parse_from_rfc3339(data.as_str().expect("invalid validate_as_string")).is_err()
    {
        state.push_error(scope.invalid_error("unable to parse"));
    }

    state
}

pub fn validate_as_date(scope: &ScopedSchema, _data: &Value) -> ValidationState {
    ValidationState::new_with_error(scope.invalid_error("TODO"))
}

pub fn validate_as_time(scope: &ScopedSchema, _data: &Value) -> ValidationState {
    ValidationState::new_with_error(scope.invalid_error("TODO"))
}