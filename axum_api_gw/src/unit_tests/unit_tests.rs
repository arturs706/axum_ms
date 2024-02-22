#[cfg(test)]
mod tests {

    use crate::new_app;

    #[cfg(test)]
    use ::axum_test::TestServer;
    #[cfg(test)]
    use ::axum_test::TestServerConfig;

    #[cfg(test)]
    fn new_test_app() -> TestServer {
        let app = new_app();
        let config = TestServerConfig::builder()
            .save_cookies()
            .expect_success_by_default()
            .mock_transport()
            .build();

        TestServer::new_with_config(app, config).unwrap()
    }

    #[cfg(test)]
    mod test_post_login {
        use super::*;
        use axum::http::StatusCode;
        use serde_json::Value;

        #[tokio::test]
        async fn it_should_create_session_on_login() {
            let server = new_test_app();
            let response = server.get("/api/v1").await;
            assert_eq!(response.status_code(), StatusCode::OK);
            let body = response.into_bytes();
            let users: Vec<Value> = serde_json::from_slice(&body).unwrap();
            assert!(!users.is_empty());
            let first_user = &users[0];

            assert!(first_user.get("user_id").is_some());
            assert!(first_user.get("name").is_some());
            assert!(first_user.get("username").is_some());
            assert!(first_user.get("mob_phone").is_some());
            assert!(first_user.get("passwd").is_some());
            assert!(first_user.get("acc_level").is_some());
            assert!(first_user.get("status").is_some());
            assert!(first_user.get("a_created").is_some());
        }
    }
}
