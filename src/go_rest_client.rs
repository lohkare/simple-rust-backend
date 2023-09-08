use reqwest::header::{ACCEPT, CONTENT_TYPE};
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use url::Url;
use crate::CONFIG;

/// A mod containing client implementation for the Go Rest -endpoints.

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct User {
    pub id: u32,
    name: String,
    email: String,
    gender: String,
    status: String
}

#[derive(Eq, PartialEq, Debug)]
pub enum GoRestError {
    UserNotFound(u32),
    RestError(StatusCode),
    JsonError(String),
    UrlError
}

const PATH: &str = "/public/v2/users";

/// Get a list of all users.
pub async fn get_users() -> Result<Vec<User>, GoRestError> {
    println!("Getting all users...");
    dbg!(get_users_implementation(CONFIG.go_rest.base_url.as_str(), CONFIG.go_rest.bearer_token.as_str()).await)
}

/// Get a list of all users.
///
/// # Arguments.
///
/// * `url` - Base url.
/// * `bearer_token` - Bearer token.
async fn get_users_implementation(url: &str, bearer_token: &str) -> Result<Vec<User>, GoRestError> {
    // Parse url and handle possible error.
    let url_result = Url::parse(url).and_then(
        |url| url.join(PATH)
    );
    if url_result.is_err() {
        return Err(GoRestError::UrlError);
    };

    let client = reqwest::Client::new();

    // Send request.
    let response = client.get(url_result.unwrap())
        .bearer_auth(bearer_token)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
    ;

    // Check that we got a response.
    // Default to 500 Internal Server Error if nothing was received.
    if let Err(e) = response {
        return Err(GoRestError::RestError(e.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)));
    };

    // Unwrap response.
    let response = response.unwrap();

    // Check for not OK status codes.
    match response.status() {
        // Do nothing if OK.
        StatusCode::OK => (),
        _ => return Err(GoRestError::RestError(response.status()))
    };

    // Deserialize received JSON as User.
    if let Ok(users) = serde_json::from_str(response.text().await.unwrap().as_str()) {
        Ok(users)
    } else {
        Err(GoRestError::JsonError(String::from("Error occurred when deserializing JSON to Vec<User>.")))
    }
}

/// Fetch a single user with given id.
///
/// # Arguments.
///
/// * `id` - User id.
pub async  fn get_user(id: u32) -> Result<User, GoRestError> {
    println!("Getting user with id {}...", &id);
    dbg!(get_user_implementation(id, CONFIG.go_rest.base_url.as_str(), CONFIG.go_rest.bearer_token.as_str()).await)
}

/// Fetch a single user with given id.
///
/// # Arguments.
///
/// * `id` - User id.
/// * `url` - Base url.
/// * `bearer_token` - Bearer token.
async fn get_user_implementation(id: u32, url: &str, bearer_token: &str) -> Result<User, GoRestError> {
    // Parse url and handle possible error.
    let url_result = Url::parse(dbg!(url)).and_then(
        |url| url.join(format!("{}/{}", PATH, id.to_string().as_str()).as_str())
    );
    if url_result.is_err() {
        return Err(GoRestError::UrlError);
    };

    let client = reqwest::Client::new();

    // Send request.
    let response = client.get(dbg!(url_result.unwrap()))
        .bearer_auth(bearer_token)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
    ;

    // Check that we got a response.
    // Default to 500 Internal Server Error if nothing was received.
    if let Err(e) = response {
        return Err(GoRestError::RestError(e.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)));
    };

    // Unwrap response.
    let response = dbg!(response.unwrap());

    // Check for not OK status codes.
    match response.status() {
        // Do nothing if OK.
        StatusCode::OK => (),
        StatusCode::NOT_FOUND => return Err(GoRestError::UserNotFound(id)),
        _ => return Err(GoRestError::RestError(response.status()))
    };

    // Deserialize received JSON as User.
    if let Ok(user) = serde_json::from_str(response.text().await.unwrap().as_str()) {
        Ok(user)
    } else {
        Err(GoRestError::JsonError(String::from("Error occurred when deserializing JSON to User.")))
    }
}

#[cfg(test)]
mod tests {
    use httpmock::Method::GET;
    use httpmock::MockServer;
    use reqwest::header::CONTENT_TYPE;
    use super::*;

    const TEST_USER_ID: u32 = 112233;

    #[tokio::test]
    async fn test_get_users_empty_url() {
        assert_eq!(
            Err(GoRestError::UrlError),
            get_users_implementation("", "").await
        );
    }

    #[tokio::test]
    async fn test_get_users_not_ok_status() {
        let mock_server = MockServer::start();

        let get_users_mock = mock_server.mock(|when, then| {
            when.method(GET)
                .path(PATH)
                .header(CONTENT_TYPE.as_str(), "application/json")
                .header(ACCEPT.as_str(), "application/json")
            ;
            then.status(StatusCode::BAD_REQUEST.as_u16());
        });

        assert_eq!(
            Err(GoRestError::RestError(StatusCode::BAD_REQUEST)),
            get_users_implementation(mock_server.url("").as_str(), "").await
        );

        get_users_mock.assert();
    }

    #[tokio::test]
    async fn test_get_users_faulty_json() {

        let mock_server = MockServer::start();

        let get_users_mock = mock_server.mock(|when, then| {
            when.method(GET)
                .path(PATH)
                .header_exists("Authorization")
                .header(CONTENT_TYPE.as_str(), "application/json")
                .header(ACCEPT.as_str(), "application/json")
            ;
            then.status(200)
                .header(CONTENT_TYPE.as_str(), "application/json")
                .body("TEST BODY")
            ;
        });

        assert_eq!(
            Err(GoRestError::JsonError(String::from("Error occurred when deserializing JSON to Vec<User>."))),
            get_users_implementation(mock_server.url("").as_str(), "").await
        );

        get_users_mock.assert();
    }

    #[tokio::test]
    async fn test_get_users() {
        let mock_server = MockServer::start();

        let test_user = User {
            id: TEST_USER_ID,
            name: String::from("TEST TESTER"),
            email: String::from("test@tester.com"),
            status: String::from("Just testing"),
            gender: String::from("Does it matter?")
        };
        let test_user_list = vec![test_user.clone()];

        let get_users_mock = mock_server.mock(|when, then| {
            when.method(GET)
                .path(String::from(PATH).as_str())
                .header_exists("Authorization")
                .header(CONTENT_TYPE.as_str(), "application/json")
                .header(ACCEPT.as_str(), "application/json")
            ;
            then.status(200)
                .header(CONTENT_TYPE.as_str(), "application/json")
                .body(serde_json::to_string(&test_user_list).unwrap())
            ;
        });

        let users_result =
            get_users_implementation(mock_server.url("").as_str(), "").await
        ;

        get_users_mock.assert();

        assert!(users_result.is_ok());

        let users_list = users_result.unwrap();
        assert!(!users_list.is_empty());
        assert_eq!(TEST_USER_ID, users_list.get(0).unwrap().id);

    }

    #[tokio::test]
    async fn test_get_user_empty_url() {
        assert_eq!(
            Err(GoRestError::UrlError),
            get_user_implementation(TEST_USER_ID, "", "").await
        );
    }

    #[tokio::test]
    async fn test_get_user_not_ok_status() {
        let mock_server = MockServer::start();

        let get_user_mock = mock_server.mock(|when, then| {
            when.method(GET)
                .path(format!("{}{}", PATH, TEST_USER_ID))
                .header(CONTENT_TYPE.as_str(), "application/json")
                .header(ACCEPT.as_str(), "application/json")
            ;
            then.status(StatusCode::BAD_REQUEST.as_u16());
        });

        assert_eq!(
            Err(GoRestError::RestError(StatusCode::BAD_REQUEST)),
            get_user_implementation(TEST_USER_ID, mock_server.url("").as_str(), "").await
        );

        get_user_mock.assert();
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let mock_server = MockServer::start();

        let get_user_mock = mock_server.mock(|when, then| {
            when.method(GET)
                .path(format!("{}{}", PATH, TEST_USER_ID))
                .header(CONTENT_TYPE.as_str(), "application/json")
                .header(ACCEPT.as_str(), "application/json")
            ;
            then.status(404);
        });

        assert_eq!(
            Err(GoRestError::UserNotFound(TEST_USER_ID)),
            get_user_implementation(TEST_USER_ID, mock_server.url("").as_str(), "").await
        );

        get_user_mock.assert();
    }

    #[tokio::test]
    async fn test_get_user_faulty_json() {

        let mock_server = MockServer::start();

        let get_user_mock = mock_server.mock(|when, then| {
            when.method(GET)
                .path(format!("{}{}", PATH, TEST_USER_ID))
                .header_exists("Authorization")
                .header(CONTENT_TYPE.as_str(), "application/json")
                .header(ACCEPT.as_str(), "application/json")
            ;
            then.status(200)
                .header(CONTENT_TYPE.as_str(), "application/json")
                .body("TEST BODY")
            ;
        });

        assert_eq!(
            Err(GoRestError::JsonError(String::from("Error occurred when deserializing JSON to User."))),
            get_user_implementation(TEST_USER_ID, mock_server.url("").as_str(), "").await
        );

        get_user_mock.assert();
    }


    #[tokio::test]
    async fn test_get_user() {
        let mock_server = MockServer::start();

        let test_user = User {
            id: TEST_USER_ID,
            name: String::from("TEST TESTER"),
            email: String::from("test@tester.com"),
            status: String::from("Just testing"),
            gender: String::from("Does it matter?")
        };

        let get_user_mock = mock_server.mock(|when, then| {
            when.method(GET)
                .path(format!("{}{}", PATH, TEST_USER_ID))
                .header_exists("Authorization")
                .header(CONTENT_TYPE.as_str(), "application/json")
                .header(ACCEPT.as_str(), "application/json")
            ;
            then.status(200)
                .header(CONTENT_TYPE.as_str(), "application/json")
                .body(serde_json::to_string(&test_user).unwrap())
            ;
        });

        let user_result =
            get_user_implementation(TEST_USER_ID, mock_server.url("").as_str(), "").await
        ;

        assert!(user_result.is_ok());
        assert_eq!(TEST_USER_ID, user_result.unwrap().id);

        get_user_mock.assert();
    }
}

