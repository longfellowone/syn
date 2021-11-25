use anyhow::{Context, Error, Result};
use chrono::Utc;
use chrono_tz::Canada;
use rand::Rng;
use reqwest::header::{
    ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, AUTHORIZATION, CONTENT_TYPE, USER_AGENT,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

pub struct Client {
    base_url: String,
    token: String,
}

pub struct Employee {
    pub username: String,
    pub password: String,
    pub device_unique_id: String,
    pub new_device: bool,
}

pub enum PunchType {
    In = 1,
    Out = 2,
}

impl Client {
    pub async fn new(base_url: &str, employee: impl Into<Employee>) -> Result<Client> {
        let employee = employee.into();
        let token = get_token(base_url, employee)
            .await
            .context("unable to get token")?;

        Ok(Client {
            base_url: base_url.to_string(),
            token,
        })
    }

    pub async fn punch(&self, punch_type: PunchType) -> Result<()> {
        #[derive(Debug, Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct PunchinRequest {
            punch_type: i8,
            location: Location,
            daily_event_type: i8,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct Location {
            latitude: f64,
            longitude: f64,
        }

        // TODO: use reqwest::url
        let url = self.base_url.to_owned() + "/SynerionMobile/api/mobile/punches/punch";

        // TODO: move request client into Client::new()
        // TODO: add user agent when constructing new client
        let client = reqwest::Client::new();
        let (time_unix, date) = date_time();

        let mut rng = rand::thread_rng();

        let rand_lat = rng.gen_range(0.0000..0.00009999999999);
        let rand_long = rng.gen_range(0.0000..0.00009999999999);

        let punchin_request = PunchinRequest {
            punch_type: punch_type as i8,
            location: Location {
                latitude: 49.2312 + rand_lat,
                longitude: -123.1197 + rand_long,
            },
            daily_event_type: 0,
        };

        let token = format!("Basic {}", self.token);
        let json =
            serde_json::to_string(&punchin_request).context("failed to marshall token request")?;

        // TODO: reqwest::header::HeaderMap
        // TODO: add private headers function, exclude auth headers
        let response = client
            .post(url)
            .header(ACCEPT, "application/json, text/plain, */*")
            .header(ACCEPT_ENCODING, "gzip,deflate,br")
            .header(ACCEPT_LANGUAGE, "en-ca")
            .header(
                USER_AGENT,
                "Synerion%20Mobile%20Pro/3.1.1 CFNetwork/1240.0.4 Darwin/20.6.0",
            )
            .header(CONTENT_TYPE, "application/json;charset=utf-8")
            .header(AUTHORIZATION, token)
            .header("x-client-version", "2.35.0")
            .header("x-request-start-time", time_unix)
            .header("client-date-time", date)
            .body(json)
            .send()
            .await
            .context("punchin request failed")?;

        // TODO: confirm "Error" is none instead of checking status
        match response.status() {
            StatusCode::OK => Ok(()),
            _ => Err(Error::msg("punch status code not 200")),
        }
    }
}

async fn get_token(base_url: &str, employee: Employee) -> Result<String> {
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

    // TODO: use reqwest::url
    let url = base_url.to_owned() + "/SynerionMobile/api/mobile/auth/login";

    let client = reqwest::Client::new();
    let (time_unix, date) = date_time();

    let token_request = TokenRequest {
        username: employee.username,
        password: employee.password,
        device_model: "iPhone13,3".to_string(),
        device_unique_id: employee.device_unique_id,
        replace_registered_device: employee.new_device,
    };

    let json = serde_json::to_string(&token_request).context("failed to marshall token request")?;

    // TODO: reqwest::header::HeaderMap
    // TODO: set .headers after .json
    // TODO: revert back to using .json
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
        .header("x-request-start-time", time_unix)
        .header("client-date-time", date)
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

fn date_time() -> (i64, String) {
    let time = Utc::now().with_timezone(&Canada::Pacific);

    let start_time_unix = time.timestamp_millis();
    let date_time = &time.to_rfc3339()[..23];

    (start_time_unix, date_time.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{
        body_string, header, header_exists, headers, method, path, query_param,
    };
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn mock_client(base_url: &str) -> Client {
        Client {
            base_url: base_url.to_string(),
            token: "mytoken".to_string(),
        }
    }

    fn mock_employee() -> Employee {
        Employee {
            username: "wrightm".to_string(),
            password: "Welcome1".to_string(),
            device_unique_id: "0AD03A66-92F7-4D3B-AA15-123C3FD633F7".to_string(),
            new_device: false,
        }
    }

    #[tokio::test]
    async fn test_punch_has_correct_headers() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/SynerionMobile/api/mobile/punches/punch"))
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
            .and(header("authorization", "Basic mytoken"))
            .and(header_exists("x-request-start-time"))
            .and(header_exists("client-date-time"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let mock_client = mock_client(&mock_server.uri());

        mock_client.punch(PunchType::In).await.unwrap()
    }

    #[tokio::test]
    async fn test_punch_request_response() {
        let mock_server = MockServer::start().await;

        let request_body = r#"{"PunchType":1,"Location":{"Latitude":49.23122430964335,"Longitude":-123.11968088332243},"DailyEventType":0}"#;
        let response_body = r#"{"PunchDateTime":"2021-11-12T14:22:00Z","Location":{"Latitude":49.231224309643352,"Longitude":-123.11968088332243},"IsLocationValid":false,"IsValid":true,"Error":null,"EventMode":0,"EventType":0,"IsMobileWithoutLocation":false}"#;

        let response_template =
            ResponseTemplate::new(200).set_body_string(response_body.to_string());

        Mock::given(method("POST"))
            .and(body_string(request_body.to_string()))
            .respond_with(response_template)
            .expect(1)
            .mount(&mock_server)
            .await;

        let mock_client = mock_client(&mock_server.uri());

        mock_client.punch(PunchType::In).await.unwrap()
    }

    #[tokio::test]
    async fn test_get_token() {
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

        let mock_employee = mock_employee();

        let token = get_token(&mock_server.uri(), mock_employee).await.unwrap();

        assert_eq!(token, "mytoken");
    }
}
