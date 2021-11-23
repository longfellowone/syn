#![allow(dead_code, unused_imports, unused_variables)]
use anyhow::{Context, Result};
use chrono::Utc;
use chrono_tz::Canada;
use reqwest::header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONTENT_TYPE, USER_AGENT};
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "https://ozzelectric.synerionenterprise.com";

pub struct Employee {
    pub username: String,
    pub password: String,
    pub device_unique_id: String,
}

pub struct Client {
    token: String,
}

impl Client {
    pub async fn new(employee: impl Into<Employee>) -> Result<Client> {
        let token = get_token(BASE_URL, employee.into())
            .await
            .context("unable to get token")?;

        println!("{}", token);

        Ok(Client { token })
    }

    pub async fn punchin(&self) -> Result<()> {
        unimplemented!()
    }

    pub async fn punchout(&self) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct TokenRequest {
    #[serde(rename = "UserName")]
    username: String,
    password: String,
    device_model: String,
    device_unique_id: String,
    replace_registered_device: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TokenResponse {
    token: String,
}

async fn get_token(base_url: &str, employee: Employee) -> Result<String> {
    let url = base_url.to_owned() + "/SynerionMobile/api/mobile/auth/login";

    let client = reqwest::Client::new();

    // create function and test
    // let (first, last) = s.split_at(2);
    let time = Utc::now().with_timezone(&Canada::Pacific);
    let start_time = time.timestamp_millis();
    let date_time: &str = &time.to_rfc3339()[..23];

    let token_request = TokenRequest {
        username: employee.username,
        password: employee.password,
        device_model: "iPhone13,3".to_string(),
        device_unique_id: employee.device_unique_id,
        replace_registered_device: false,
    };

    let json = serde_json::to_string(&token_request).context("failed to marshall token request")?;

    let response = client
        .post(url)
        .query(&[("CustomerId", "ozzelectric")])
        .header(ACCEPT, "application/json, text/plain, */*")
        .header(ACCEPT_ENCODING, "gzip,deflate,br")
        .header(ACCEPT_LANGUAGE, "en-ca")
        .header(
            USER_AGENT,
            "Synerion%20Mobile%20Pro/3.1.1 CFNetwork/1240.0.4 Darwin/20.6.0",
        )
        .header(CONTENT_TYPE, "application/json;charset=utf-8")
        .header("x-client-version", "2.35.0")
        .header("x-request-start-time", start_time)
        .header("client-date-time", date_time)
        .body(json)
        .send()
        .await
        .context("token request failed")?;

    let token = response
        .json::<TokenResponse>()
        .await
        .context("failed to unmarshall token response")?
        .token;

    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{
        body_string, header, header_exists, headers, method, path, query_param,
    };
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_token_for_user() {
        let mock_server = MockServer::start().await;

        let request_body = r#"{"UserName":"wrightm","Password":"Welcome1","DeviceModel":"iPhone13,3","DeviceUniqueId":"0AD03A66-92F7-4D3B-AA15-123C3FD633F7","ReplaceRegisteredDevice":false}"#;
        let response_body = r#"{"Token": "mytoken"}"#;

        let response_template =
            ResponseTemplate::new(200).set_body_string(response_body.to_string());

        Mock::given(method("POST"))
            .and(path("/SynerionMobile/api/mobile/auth/login"))
            .and(query_param("CustomerId", "ozzelectric"))
            .and(headers(
                "accept",
                vec!["application/json", "text/plain", "*/*"],
            ))
            .and(headers("accept-encoding", vec!["gzip", "deflate", "br"]))
            .and(header("accept-language", "en-ca"))
            .and(header(
                "user-agent",
                "Synerion%20Mobile%20Pro/3.1.1 CFNetwork/1240.0.4 Darwin/20.6.0",
            ))
            .and(header("content-type", "application/json;charset=utf-8"))
            .and(header("x-client-version", "2.35.0"))
            .and(header_exists("x-request-start-time"))
            .and(header_exists("client-date-time"))
            .and(body_string(request_body.to_string()))
            .respond_with(response_template)
            .expect(1)
            .mount(&mock_server)
            .await;

        let employee = Employee {
            username: "wrightm".to_string(),
            password: "Welcome1".to_string(),
            device_unique_id: "0AD03A66-92F7-4D3B-AA15-123C3FD633F7".to_string(),
        };

        let token = get_token(&mock_server.uri(), employee).await.unwrap();

        assert_eq!(token, "mytoken");
    }
}
