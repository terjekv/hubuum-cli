use cli_command_derive::CliCommand;
use hubuum_client::{Authenticated, Class, ClassGet, ClassPost, IntoResourceFilter, SyncClient};
use serde::{Deserialize, Serialize};

use super::CliCommand;
use super::{CliCommandInfo, CliOption};

use crate::errors::AppError;
use crate::formatting::OutputFormatter;
use crate::tokenizer::CommandTokenizer;
use crate::traits::SingleItemOrWarning;

trait GetClassname {
    fn classname(&self) -> Option<String>;
}

#[derive(Debug, Serialize, Deserialize, Clone, CliCommand, Default)]
#[command_info(
    about = "Create a new class",
    long_about = "Create a new class with the specified properties.",
    examples = r#"-n MyClass -i 1 -d "My class description"
--name MyClass --namespace-id 1 --description 'My class' --schema '{\"type\": \"object\"}'"#
)]
pub struct ClassNew {
    #[option(short = "n", long = "name", help = "Name of the class")]
    pub name: String,
    #[option(short = "i", long = "namespace-id", help = "Namespace ID")]
    pub namespace_id: i32,
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
        println!("Creating new class: {:?}", new);

        let result = client.classes().create(ClassPost {
            name: new.name.clone(),
            namespace_id: new.namespace_id,
            description: new.description.clone(),
            json_schema: new.json_schema.clone(),
            validate_schema: new.validate_schema,
        })?;

        result.format(15)?;

        Ok(())
    }
}

impl IntoResourceFilter<Class> for &ClassInfo {
    fn into_resource_filter(self) -> ClassGet {
        ClassGet {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            namespace_id: None,
            json_schema: None,
            validate_schema: None,
            created_at: None,
            updated_at: None,
        }
    }
}

impl GetClassname for &ClassInfo {
    fn classname(&self) -> Option<String> {
        self.name.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, CliCommand, Default)]
pub struct ClassInfo {
    #[option(short = "i", long = "id", help = "ID of the class")]
    pub id: Option<i32>,
    #[option(short = "n", long = "name", help = "Name of the class")]
    pub name: Option<String>,
    #[option(short = "d", long = "description", help = "Description of the class")]
    pub description: Option<String>,
}
impl CliCommand for ClassInfo {
    fn execute(
        &self,
        client: &SyncClient<Authenticated>,
        tokens: &CommandTokenizer,
    ) -> Result<(), AppError> {
        let mut query = self.new_from_tokens(tokens)?;
        query.name = classname_or_pos(&query, tokens, 0)?;
        client
            .classes()
            .filter(&query)?
            .single_item_or_warning()?
            .format(15)?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, CliCommand, Default)]
pub struct ClassDelete {
    #[option(short = "i", long = "id", help = "ID of the class")]
    pub id: Option<i32>,
    #[option(short = "n", long = "name", help = "Name of the class")]
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

        let class = client.classes().filter(&query)?.single_item_or_warning()?;
        client.classes().delete(class.id)?;

        Ok(())
    }
}

impl IntoResourceFilter<Class> for &ClassDelete {
    fn into_resource_filter(self) -> ClassGet {
        ClassGet {
            id: self.id,
            name: self.name.clone(),
            description: None,
            namespace_id: None,
            json_schema: None,
            validate_schema: None,
            created_at: None,
            updated_at: None,
        }
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
