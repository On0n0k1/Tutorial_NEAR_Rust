use near_sdk::{
    AccountId,
    borsh::{ self, BorshSerialize },
    FunctionError,
};

#[derive(FunctionError, BorshSerialize)]
pub enum Errors{
    AccountIsAlreadyRegistered(AccountId),
    AccountIsNotRegistered(AccountId),
    CharacterNotFound(String),
    CharacterAlreadyExists(String),
    InvalidChapterValidation,
    InvalidCharacterName(String),
    InvalidClassName(String),
    UserNotRegistered(AccountId),
}


impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Errors::AccountIsAlreadyRegistered(user) => write!(f, "Username {} is already registered in the database.", user),
            Errors::AccountIsNotRegistered(user) => write!(f, "Tried to update {}, but account is not registered. This a server error, not a user error. Please report it.", user),
            Errors::CharacterNotFound(name) => write!(f, "Character with name {} not found in current account.", name),
            Errors::CharacterAlreadyExists(name) => write!(f, "A character with name {} already exists in this account.", name),
            Errors::InvalidChapterValidation => write!(f, "Failed to validate chapter report"),
            Errors::InvalidCharacterName(name) => write!(f, "Character name starts with an invalid character ({}).", name),
            Errors::InvalidClassName(name) => write!(f, "Invalid name ({}) for character class.", name),
            Errors::UserNotRegistered(user) => write!(f, "User {} needs to create an account before using this service.", user),
        }
    }
}