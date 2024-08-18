use cli_command_derive::CliCommand;
use hubuum_client::{Authenticated, IntoResourceFilter, SyncClient, User, UserGet, UserPost};
use serde::{Deserialize, Serialize};

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::errors::AppError;
use crate::formatting::OutputFormatter;
use crate::logger::with_timing;
use crate::output::{add_warning, append_key_value, append_line};
use crate::traits::SingleItemOrWarning;

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
    fn into_post(&self) -> UserPost {
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
    fn into_resource_filter(self) -> UserGet {
        UserGet {
            id: None,
            username: self.username.clone(),
            email: None,
            created_at: None,
            updated_at: None,
        }
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

        let user =
            with_timing("User get", || client.users().filter(&query))?.single_item_or_warning()?;

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
    fn into_resource_filter(self) -> UserGet {
        UserGet {
            id: None,
            username: self.username.clone(),
            email: self.email.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
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

        with_timing("User get", || client.users().filter(&query))?
            .single_item_or_warning()?
            .format(15)?;

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

fn single_user_or_warning(users: Vec<User>) -> Result<User, AppError> {
    if users.is_empty() {
        add_warning("User not found")?;
        return Err(AppError::Quiet);
    } else if users.len() > 1 {
        add_warning("Multiple users found.")?;
        return Err(AppError::Quiet);
    }
    Ok(users[0].clone())
}
