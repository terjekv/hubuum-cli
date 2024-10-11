use cli_command_derive::CliCommand;
use hubuum_client::{Authenticated, IntoResourceFilter, QueryFilter, SyncClient, User, UserPost};
use serde::{Deserialize, Serialize};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::errors::AppError;
use crate::formatting::{OutputFormatter, OutputFormatterWithPadding};
use crate::logger::with_timing;
use crate::output::{append_key_value, append_line};

use crate::tokenizer::CommandTokenizer;

use super::CliCommand;
use super::{CliCommandInfo, CliOption};

trait GetUsername {
    fn username(&self) -> Option<String>;
}

#[derive(Debug, Serialize, Deserialize, Clone, CliCommand, Default)]
pub struct UserNew {
    #[option(short = "u", long = "username", help = "Username of the user")]
    pub username: String,
    #[option(short = "e", long = "email", help = "Email address for the user")]
    pub email: Option<String>,
}

impl UserNew {
    fn into_post(self) -> UserPost {
        UserPost {
            username: self.username.clone(),
            email: self.email.clone(),
            password: generate_random_password(20),
        }
    }
}

impl CliCommand for UserNew {
    fn execute(
        &self,
        client: &SyncClient<Authenticated>,
        tokens: &CommandTokenizer,
    ) -> Result<(), AppError> {
        let new = self.new_from_tokens(tokens)?.into_post();
        let password = new.password.clone();

        let user = with_timing("Creating user", || client.users().create(new))?;

        user.format(15)?;
        append_key_value("Password", password, 15)?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, CliCommand, Default)]
pub struct UserDelete {
    #[option(short = "u", long = "username", help = "Username of the user")]
    pub username: Option<String>,
}

impl IntoResourceFilter<User> for &UserDelete {
    fn into_resource_filter(self) -> Vec<QueryFilter> {
        let mut filters = vec![];

        if let Some(username) = &self.username {
            filters.push(QueryFilter {
                key: "username".to_string(),
                value: username.clone(),
            });
        }

        filters
    }
}

impl GetUsername for &UserDelete {
    fn username(&self) -> Option<String> {
        self.username.clone()
    }
}

impl CliCommand for UserDelete {
    fn execute(
        &self,
        client: &SyncClient<Authenticated>,
        tokens: &CommandTokenizer,
    ) -> Result<(), AppError> {
        let mut query = self.new_from_tokens(tokens)?;

        query.username = username_or_pos(&query, tokens, 0)?;

        let user = with_timing("User get", || {
            client.users().filter_expecting_single_result(&query)
        })?;

        with_timing("User delete", || client.users().delete(user.id))?;
        append_line(format!("User '{}' deleted", user.username.clone()))?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, CliCommand, Default)]
pub struct UserInfo {
    #[option(short = "u", long = "username", help = "Username of the user")]
    pub username: Option<String>,
    #[option(short = "e", long = "email", help = "Email address for the user")]
    pub email: Option<String>,
    #[option(short = "C", long = "created-at", help = "Created at timestammp")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[option(short = "U", long = "updated-at", help = "Updated at timestamp")]
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl IntoResourceFilter<User> for &UserInfo {
    fn into_resource_filter(self) -> Vec<QueryFilter> {
        let mut filters = vec![];

        if let Some(username) = &self.username {
            filters.push(QueryFilter {
                key: "username".to_string(),
                value: username.clone(),
            });
        }

        if let Some(email) = &self.email {
            filters.push(QueryFilter {
                key: "email".to_string(),
                value: email.clone(),
            });
        }

        if let Some(created_at) = &self.created_at {
            filters.push(QueryFilter {
                key: "created_at".to_string(),
                value: created_at.to_string(),
            });
        }

        if let Some(updated_at) = &self.updated_at {
            filters.push(QueryFilter {
                key: "updated_at".to_string(),
                value: updated_at.to_string(),
            });
        }

        filters
    }
}

impl GetUsername for &UserInfo {
    fn username(&self) -> Option<String> {
        self.username.clone()
    }
}

impl CliCommand for UserInfo {
    fn execute(
        &self,
        client: &SyncClient<Authenticated>,
        tokens: &CommandTokenizer,
    ) -> Result<(), AppError> {
        let mut query = self.new_from_tokens(tokens)?;

        query.username = username_or_pos(&query, tokens, 0)?;

        with_timing("User get", || {
            client.users().filter_expecting_single_result(&query)
        })?
        .format(15)?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, CliCommand, Default)]
pub struct UserList {
    #[option(short = "u", long = "username", help = "Username of the user")]
    pub username: Option<String>,
    #[option(short = "e", long = "email", help = "Email address for the user")]
    pub email: Option<String>,
    #[option(short = "C", long = "created-at", help = "Created at timestammp")]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[option(short = "U", long = "updated-at", help = "Updated at timestamp")]
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl CliCommand for UserList {
    fn execute(
        &self,
        client: &SyncClient<Authenticated>,
        tokens: &CommandTokenizer,
    ) -> Result<(), AppError> {
        let _ = self.new_from_tokens(tokens)?;
        let users = with_timing("User list", || client.users().find().execute())?;
        users.format()?;

        Ok(())
    }
}

pub fn generate_random_password(length: usize) -> String {
    let mut rng = thread_rng();
    std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(length)
        .collect()
}

fn username_or_pos<U>(
    query: U,
    tokens: &CommandTokenizer,
    pos: usize,
) -> Result<Option<String>, AppError>
where
    U: GetUsername,
{
    let pos0 = tokens.get_positionals().get(pos);
    if query.username().is_none() {
        if pos0.is_none() {
            return Err(AppError::MissingOptions(vec!["username".to_string()]));
        }
        return Ok(pos0.cloned());
    };
    Ok(query.username().clone())
}
