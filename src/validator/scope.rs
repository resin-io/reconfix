use crate::schema::Schema;

use super::error::ValidationError;
use super::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ScopedSchema<'a> {
    schema: &'a Schema,
    schema_path: PathBuf,
    data_path: PathBuf,
}

impl<'a> ScopedSchema<'a> {
    pub fn new(schema: &Schema) -> ScopedSchema {
        ScopedSchema {
            schema,
            schema_path: PathBuf::new(),
            data_path: PathBuf::new(),
        }
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn schema_path(&self) -> &PathBuf {
        &self.schema_path
    }

    pub fn data_path(&self) -> &PathBuf {
        &self.data_path
    }
}

impl<'a> ScopedSchema<'a> {
    pub fn scope_with_schema<'b>(&self, schema: &'b Schema) -> ScopedSchema<'b> {
        ScopedSchema {
            schema,
            schema_path: self.schema_path.clone(),
            data_path: self.data_path.clone(),
        }
    }

    pub fn scope_with_data_index(&self, index: usize) -> ScopedSchema {
        let mut data_path = self.data_path.clone();
        data_path.push_index(index);

        ScopedSchema {
            schema: self.schema,
            schema_path: self.schema_path.clone(),
            data_path,
        }
    }

    pub fn scope_with_data_property<S: Into<String>>(&self, property: S) -> ScopedSchema {
        let mut data_path = self.data_path.clone();
        data_path.push_property(property);

        ScopedSchema {
            schema: self.schema,
            schema_path: self.schema_path.clone(),
            data_path,
        }
    }

    pub fn scope_with_schema_index(&self, index: usize) -> ScopedSchema {
        let mut schema_path = self.schema_path.clone();
        schema_path.push_index(index);

        ScopedSchema {
            schema: self.schema,
            schema_path,
            data_path: self.data_path.clone(),
        }
    }

    pub fn scope_with_schema_keyword<S: Into<String>>(&self, keyword: S) -> ScopedSchema {
        let mut schema_path = self.schema_path.clone();
        schema_path.push_property(keyword);

        ScopedSchema {
            schema: self.schema,
            schema_path,
            data_path: self.data_path.clone(),
        }
    }
}

impl<'a> ScopedSchema<'a> {
    pub fn error<S1, S2>(&self, keyword: S1, message: S2) -> ValidationError
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        ValidationError::new(
            keyword,
            self.schema_path().to_string(),
            self.data_path().to_string(),
            message,
        )
    }
}
