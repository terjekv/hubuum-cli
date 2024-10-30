use cli_command_derive::CliCommand;
use hubuum_client::{Authenticated, Class, ClassPost, IntoResourceFilter, QueryFilter, SyncClient};
use serde::{Deserialize, Serialize};

use super::shared::find_class_by_name;
use super::CliCommand;
use super::{CliCommandInfo, CliOption};

use crate::autocomplete::{classes, namespaces};
use crate::commands::shared::find_namespace_by_name;
use crate::errors::AppError;
use crate::formatting::{OutputFormatter, OutputFormatterWithPadding};
use crate::output::append_key_value;
use crate::tokenizer::CommandTokenizer;

trait GetClassname {
    fn classname(&self) -> Option<String>;
}

#[derive(Debug, Serialize, Deserialize, Clone, CliCommand, Default)]
#[command_info(
    about = "Create a new class",
    long_about = "Create a new class with the specified properties.",
    examples = r#"-n MyClass -N namespace_1 -d "My class description"
--name MyClass --namespace namespace_1 --description 'My class' --schema '{\"type\": \"object\"}'"#
)]
pub struct ClassNew {
    #[option(short = "n", long = "name", help = "Name of the class")]
    pub name: String,
    #[option(
        short = "N",
        long = "namespace",
        help = "Namespace name",
        autocomplete = "namespaces"
    )]
    pub namespace: String,
    #[option(short = "d", long = "description", help = "Description of the class")]
    pub description: String,
    #[option(short = "s", long = "schema", help = "JSON schema for the class")]
    pub json_schema: Option<serde_json::Value>,
    #[option(
        short = "v",
        long = "validate",
        help = "Validate against schema, requires schema to be set"
    )]
    pub validate_schema: Option<bool>,
}

impl CliCommand for ClassNew {
    fn execute(
        &self,
        client: &SyncClient<Authenticated>,
        tokens: &CommandTokenizer,
    ) -> Result<(), AppError> {
        let new = &self.new_from_tokens(tokens)?;
        let namespace = find_namespace_by_name(client, &new.namespace)?;

        let result = client.classes().create(ClassPost {
            name: new.name.clone(),
            namespace_id: namespace.id,
            description: new.description.clone(),
            json_schema: new.json_schema.clone(),
            validate_schema: new.validate_schema,
        })?;

        result.format(15)?;

        Ok(())
    }
}

impl IntoResourceFilter<Class> for &ClassInfo {
    fn into_resource_filter(self) -> Vec<QueryFilter> {
        let mut filters = vec![];
        if let Some(name) = &self.name {
            filters.push(QueryFilter {
                key: "name".to_string(),
                value: name.clone(),
            });
        }

        filters
    }
}

impl GetClassname for &ClassInfo {
    fn classname(&self) -> Option<String> {
        self.name.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, CliCommand, Default)]
pub struct ClassInfo {
    #[option(
        short = "n",
        long = "name",
        help = "Name of the class",
        autocomplete = "classes"
    )]
    pub name: Option<String>,
}

impl CliCommand for ClassInfo {
    fn execute(
        &self,
        client: &SyncClient<Authenticated>,
        tokens: &CommandTokenizer,
    ) -> Result<(), AppError> {
        let mut query = self.new_from_tokens(tokens)?;
        query.name = classname_or_pos(&query, tokens, 0)?;
        let class = find_class_by_name(client, &query.name.unwrap())?;

        class.format(15)?;

        // This will hopefully be a head request in the future
        let objects = client.objects(class.id).find().execute()?;
        // Not sure why we need to reduce the padding, should be checked.
        append_key_value("Objects", objects.len(), 14)?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, CliCommand, Default)]
pub struct ClassDelete {
    #[option(
        short = "n",
        long = "name",
        help = "Name of the class",
        autocomplete = "classes"
    )]
    pub name: Option<String>,
}

impl CliCommand for ClassDelete {
    fn execute(
        &self,
        client: &SyncClient<Authenticated>,
        tokens: &CommandTokenizer,
    ) -> Result<(), AppError> {
        let mut query = self.new_from_tokens(tokens)?;
        query.name = classname_or_pos(&query, tokens, 0)?;

        let class = find_class_by_name(client, &query.name.unwrap())?;
        client.classes().delete(class.id)?;

        Ok(())
    }
}

impl GetClassname for &ClassDelete {
    fn classname(&self) -> Option<String> {
        self.name.clone()
    }
}

fn classname_or_pos<U>(
    query: U,
    tokens: &CommandTokenizer,
    pos: usize,
) -> Result<Option<String>, AppError>
where
    U: GetClassname,
{
    let pos0 = tokens.get_positionals().get(pos);
    if query.classname().is_none() {
        if pos0.is_none() {
            return Err(AppError::MissingOptions(vec!["name".to_string()]));
        }
        return Ok(pos0.cloned());
    };
    Ok(query.classname().clone())
}

#[derive(Debug, Serialize, Deserialize, Clone, CliCommand, Default)]
pub struct ClassList {
    #[option(short = "n", long = "name", help = "Name of the class")]
    pub name: Option<String>,
    #[option(short = "d", long = "description", help = "Description of the class")]
    pub description: Option<String>,
}

impl IntoResourceFilter<Class> for &ClassList {
    fn into_resource_filter(self) -> Vec<QueryFilter> {
        let mut filters = vec![];
        if let Some(name) = &self.name {
            filters.push(QueryFilter {
                key: "name".to_string(),
                value: name.clone(),
            });
        }
        if let Some(description) = &self.description {
            filters.push(QueryFilter {
                key: "description".to_string(),
                value: description.clone(),
            });
        }
        filters
    }
}

impl CliCommand for ClassList {
    fn execute(
        &self,
        client: &SyncClient<Authenticated>,
        tokens: &CommandTokenizer,
    ) -> Result<(), AppError> {
        let new = self.new_from_tokens(tokens)?;
        let classes = client.classes().filter(&new)?;
        classes.format()?;
        Ok(())
    }
}
