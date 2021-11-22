use anyhow::{Context, Result};
use chrono::Utc;
use chrono_tz::Canada;
use reqwest::header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONTENT_TYPE, USER_AGENT};
use serde::{Deserialize, Serialize};

const BASE_URI: &str = "https://ozzelectric.synerionenterprise.com";

pub struct Client {
    token: String,
}

impl Client {
    pub async fn new(username: String, password: String) -> Result<Client> {
        let token = get_token(username, password).await?;

        println!("{}", token);

        Ok(Client { token })
    }
}

async fn get_token(username: String, password: String) -> Result<String> {
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

    let url = BASE_URI.to_owned() + "/SynerionMobile/api/mobile/auth/login_99999999999";

    let client = reqwest::Client::new();

    // create function and test
    // let (first, last) = s.split_at(2);
    let time = Utc::now().with_timezone(&Canada::Pacific);
    let start_time = time.timestamp_millis();
    let date_time: &str = &time.to_rfc3339()[..23];

    let token_request = TokenRequest {
        username,
        password,
        device_model: "iPhone13,3".to_string(),
        device_unique_id: "0AD03A66-92F7-4D3B-AA15-123C3FD633F7".to_string(),
        replace_registered_device: false,
    };

    let json = serde_json::to_string(&token_request).expect("my error");

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
        .context("test1")?;

    let token = response
        .json::<TokenResponse>()
        .await
        .context("test2")?
        .token;

    Ok(token)
}

//
// use anyhow::Result;
// use chrono::Utc;
// use chrono_tz::Canada;
// use reqwest::header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONTENT_TYPE, USER_AGENT};
// use serde::{Deserialize, Serialize};
//
// pub struct SynClient {
//     name: String,
//     password: String,
//     pub token: String,
// }
//
// impl SynClient {
//     // let mut user = SynClient::new("wrightm", "Welcome1");
//     // user.token("https://ozzelectric.synerionenterprise.com")
//     // .await
//     // .unwrap();
//     //
//     // println!("{}", user.token);
//
//     pub fn new(name: &str, password: &str) -> Self {
//         SynClient {
//             name: name.to_owned(),
//             password: password.to_owned(),
//             token: "".to_string(),
//         }
//     }
//
//     async fn token(&mut self, url: &str) -> Result<()> {
//         #[derive(Debug, Serialize)]
//         #[serde(rename_all = "PascalCase")]
//         struct TokenRequest<'a> {
//             user_name: &'a str,
//             password: &'a str,
//             device_model: String,
//             device_unique_id: String,
//             replace_registered_device: bool,
//         }
//
//         #[derive(Debug, Deserialize)]
//         #[serde(rename_all = "PascalCase")]
//         struct TokenResponse {
//             token: String,
//         }
//
//         // https://ozzelectric.synerionenterprise.com
//         let url = url.to_owned() + "/SynerionMobile/api/mobile/auth/login";
//
//         let client = reqwest::Client::new();
//
//         // create function and test
//         // let (first, last) = s.split_at(2);
//         let time = Utc::now().with_timezone(&Canada::Pacific);
//         let start_time = time.timestamp_millis();
//         let date_time: &str = &time.to_rfc3339()[..23];
//
//         let token_request = TokenRequest {
//             user_name: &self.name,
//             password: &self.password,
//             device_model: "iPhone13,3".to_string(),
//             device_unique_id: "0AD03A66-92F7-4D3B-AA15-123C3FD633F7".to_string(),
//             replace_registered_device: false,
//         };
//
//         let json = serde_json::to_string(&token_request).expect("my error");
//
//         let response = client
//             .post(url)
//             .query(&[("CustomerId", "ozzelectric")])
//             .header(ACCEPT, "application/json, text/plain, */*")
//             .header(ACCEPT_ENCODING, "gzip,deflate,br")
//             .header(ACCEPT_LANGUAGE, "en-ca")
//             .header(
//                 USER_AGENT,
//                 "Synerion%20Mobile%20Pro/3.1.1 CFNetwork/1240.0.4 Darwin/20.6.0",
//             )
//             .header(CONTENT_TYPE, "application/json;charset=utf-8")
//             .header("x-client-version", "2.35.0")
//             .header("x-request-start-time", start_time)
//             .header("client-date-time", date_time)
//             .body(json)
//             .send()
//             .await?;
//
//         self.token = response.json::<TokenResponse>().await?.token;
//
//         Ok(())
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use serde_json::json;
//     use wiremock::matchers::{
//         body_string, header, header_exists, headers, method, path, query_param,
//     };
//     use wiremock::{Mock, MockServer, ResponseTemplate};
//
//     #[tokio::test]
//     async fn test_get_token_for_user() {
//         let mut user = SynClient::new("wrightm", "Welcome1");
//
//         let token_request = r#"{"UserName":"wrightm","Password":"Welcome1","DeviceModel":"iPhone13,3","DeviceUniqueId":"0AD03A66-92F7-4D3B-AA15-123C3FD633F7","ReplaceRegisteredDevice":false}"#;
//         let token = json!({"Token": "mytoken"});
//
//         let response_template = ResponseTemplate::new(200).set_body_json(token);
//
//         let mock_server = MockServer::start().await;
//
//         Mock::given(method("POST"))
//             .and(path("/SynerionMobile/api/mobile/auth/login"))
//             .and(query_param("CustomerId", "ozzelectric"))
//             .and(headers(
//                 "accept",
//                 vec!["application/json", "text/plain", "*/*"],
//             ))
//             .and(headers("accept-encoding", vec!["gzip", "deflate", "br"]))
//             .and(header("accept-language", "en-ca"))
//             .and(header(
//                 "user-agent",
//                 "Synerion%20Mobile%20Pro/3.1.1 CFNetwork/1240.0.4 Darwin/20.6.0",
//             ))
//             .and(header("content-type", "application/json;charset=utf-8"))
//             .and(header("x-client-version", "2.35.0"))
//             .and(header_exists("x-request-start-time"))
//             .and(header_exists("client-date-time"))
//             .and(body_string(token_request.to_string()))
//             .respond_with(response_template)
//             .expect(1)
//             .mount(&mock_server)
//             .await;
//
//         user.token(&mock_server.uri()).await.unwrap();
//
//         assert_eq!(user.token, "mytoken");
//     }
// }
