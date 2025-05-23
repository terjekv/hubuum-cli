use hubuum_client::Group;

use super::{append_key_value, OutputFormatterWithPadding};
use crate::errors::AppError;

impl OutputFormatterWithPadding for Group {
    fn format(&self, padding: usize) -> Result<(), AppError> {
        append_key_value("Name", &self.groupname, padding)?;
        append_key_value("Description", &self.description, padding)?;
        append_key_value("Created", self.created_at, padding)?;
        append_key_value("Updated", self.updated_at, padding)?;
        Ok(())
    }
}
