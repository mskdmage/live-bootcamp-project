pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    InvalidInput,
    IncorrectCredentials,
    UnexpectedError,
    MissingToken,
    InvalidToken,
}
