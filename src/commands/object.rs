use cli_command_derive::CliCommand;

use hubuum_client::{
    Authenticated, IntoResourceFilter, Object, ObjectPost, QueryFilter, SyncClient,
};
use serde::{Deserialize, Serialize};

use super::CliCommand;
use super::{CliCommandInfo, CliOption};

use crate::errors::AppError;
use crate::formatting::{OutputFormatter, OutputFormatterWithPadding};
use crate::output::append_key_value;
use crate::tokenizer::CommandTokenizer;

trait GetObjectname {
    fn objectname(&self) -> Option<String>;
}

#[derive(Debug, Serialize, Deserialize, Clone, CliCommand, Default)]
#[command_info(
    about = "Create a object class",
    long_about = "Create a new object in a specific class with the specified properties.",
    examples = r#"-n MyObject -c MyClaass -N namespace_1 -d "My object description"
--name MyObject --class MyClass --namespace namespace_1 --description 'My object' --data '{\"key\": \"val\"}'"#
)]
pub struct ObjectNew {
    #[option(short = "n", long = "name", help = "Name of the object")]
    pub name: String,
    #[option(
        short = "c",
        long = "class",
        help = "Name of the class the object belongs to"
    )]
    pub class: String,
    #[option(short = "N", long = "namespace", help = "Namespace name")]
    pub namespace: String,
    #[option(short = "d", long = "description", help = "Description of the class")]
    pub description: String,
    #[option(
        short = "D",
        long = "data",
        help = "JSON data for the object the class"
    )]
    pub data: Option<serde_json::Value>,
}

impl CliCommand for ObjectNew {
    fn execute(
        &self,
        client: &SyncClient<Authenticated>,
        tokens: &CommandTokenizer,
    ) -> Result<(), AppError> {
        let new = &self.new_from_tokens(tokens)?;
        let namespace = client
            .namespaces()
            .find()
            .add_filter_name_exact(&new.namespace)
            .execute_expecting_single_result()?;

        let class = client
            .classes()
            .find()
            .add_filter_name_exact(&new.class)
            .execute_expecting_single_result()?;

        let result = client.objects(class.id).create(ObjectPost {
            name: new.name.clone(),
            hubuum_class_id: class.id,
            namespace_id: namespace.id,
            description: new.description.clone(),
            data: new.data.clone(),
        })?;

        result.format(15)?;

        Ok(())
    }
}

impl IntoResourceFilter<Object> for &ObjectInfo {
    fn into_resource_filter(self) -> Vec<QueryFilter> {
        let mut filters = vec![];
        if let Some(name) = &self.name {
            filters.push(QueryFilter {
                key: "name".to_string(),
                value: name.clone(),
            });
        }
        if let Some(id) = &self.id {
            filters.push(QueryFilter {
                key: "id".to_string(),
                value: id.to_string(),
            });
        }

        filters
    }
}

impl GetObjectname for &ObjectInfo {
    fn objectname(&self) -> Option<String> {
        self.name.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, CliCommand, Default)]
pub struct ObjectInfo {
    #[option(short = "i", long = "id", help = "ID of the object")]
    pub id: Option<i32>,
    #[option(short = "n", long = "name", help = "Name of the object")]
    pub name: Option<String>,
}

impl CliCommand for ObjectInfo {
    fn execute(
        &self,
        client: &SyncClient<Authenticated>,
        tokens: &CommandTokenizer,
    ) -> Result<(), AppError> {
        let mut query = self.new_from_tokens(tokens)?;
        query.name = objectname_or_pos(&query, tokens, 0)?;
        let class = client
            .classes()
            .find()
            .add_filter(
                "name",
                hubuum_client::FilterOperator::Equals { is_negated: false },
                query.name.clone().unwrap(),
            )
            .execute_expecting_single_result()?;

        class.format(15)?;

        let objects = client.objects(class.id).find().execute()?;
        append_key_value("Objects:", objects.len(), 15)?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, CliCommand, Default)]
pub struct ObjectDelete {
    #[option(short = "i", long = "id", help = "ID of the objet")]
    pub id: Option<i32>,
    #[option(short = "n", long = "name", help = "Name of the object")]
    pub name: Option<String>,
    #[option(short = "c", long = "class", help = "Class of the object")]
    pub class: Option<String>,
}

impl CliCommand for ObjectDelete {
    fn execute(
        &self,
        client: &SyncClient<Authenticated>,
        tokens: &CommandTokenizer,
    ) -> Result<(), AppError> {
        let mut query = self.new_from_tokens(tokens)?;
        query.name = objectname_or_pos(&query, tokens, 1)?;

        let query_clone = query.clone();

        let class = if query.class.is_some() {
            client
                .classes()
                .find()
                .add_filter_name_exact(&query.class.unwrap())
                .execute_expecting_single_result()?
        } else {
            return Err(AppError::MissingOptions(vec!["class".to_string()]));
        };

        let object = client
            .objects(class.id)
            .filter_expecting_single_result(&query_clone)?;
        client.objects(class.id).delete(object.id)?;

        Ok(())
    }
}

impl IntoResourceFilter<Object> for &ObjectDelete {
    fn into_resource_filter(self) -> Vec<QueryFilter> {
        let mut filters = vec![];
        if let Some(name) = &self.name {
            filters.push(QueryFilter {
                key: "name".to_string(),
                value: name.clone(),
            });
        }
        if let Some(id) = &self.id {
            filters.push(QueryFilter {
                key: "id".to_string(),
                value: id.to_string(),
            });
        }

        filters
    }
}

impl GetObjectname for &ObjectDelete {
    fn objectname(&self) -> Option<String> {
        self.name.clone()
    }
}

fn objectname_or_pos<U>(
    query: U,
    tokens: &CommandTokenizer,
    pos: usize,
) -> Result<Option<String>, AppError>
where
    U: GetObjectname,
{
    let pos0 = tokens.get_positionals().get(pos);
    if query.objectname().is_none() {
        if pos0.is_none() {
            return Err(AppError::MissingOptions(vec!["name".to_string()]));
        }
        return Ok(pos0.cloned());
    };
    Ok(query.objectname().clone())
}

#[derive(Debug, Serialize, Deserialize, Clone, CliCommand, Default)]
pub struct ObjectList {
    #[option(short = "c", long = "class", help = "Name of the class")]
    pub class: String,
    #[option(short = "n", long = "name", help = "Name of the object")]
    pub name: Option<String>,
    #[option(short = "d", long = "description", help = "Description of the class")]
    pub description: Option<String>,
}

impl IntoResourceFilter<Object> for &ObjectList {
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

impl CliCommand for ObjectList {
    fn execute(
        &self,
        client: &SyncClient<Authenticated>,
        tokens: &CommandTokenizer,
    ) -> Result<(), AppError> {
        let new: ObjectList = self.new_from_tokens(tokens)?;

        let class = client
            .classes()
            .find()
            .add_filter_name_exact(&new.class)
            .execute_expecting_single_result()?;

        let objects = client.objects(class.id).filter(&new)?;

        let class_ids = objects
            .iter()
            .map(|o| o.hubuum_class_id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let namespace_ids = objects
            .iter()
            .map(|o| o.namespace_id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let _classes = client
            .classes()
            .find()
            .add_filter(
                "id",
                hubuum_client::FilterOperator::Equals { is_negated: false },
                class_ids,
            )
            .execute()?;

        let _namespaces = client
            .namespaces()
            .find()
            .add_filter(
                "id",
                hubuum_client::FilterOperator::Equals { is_negated: false },
                namespace_ids,
            )
            .execute()?;

        objects.format()?;
        Ok(())
    }
}