use reqwest::{header::{HeaderMap, ACCEPT, CONTENT_TYPE}, Method};
use serde::{Deserialize, Serialize};
use crate::serializers::from_string_to_authorization_token;


use super::lib::RestMarketDataRequest;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginMessage{
    pub member_id: String,
    #[serde(deserialize_with="from_string_to_authorization_token")]
    pub token: String
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginMessageRequestParams{
    pub endpoint: String,
    pub email: String,
    pub password: String
}

impl RestMarketDataRequest for LoginMessageRequestParams{
    fn get_request_url(&self) -> String{
        let endpoint = self.endpoint.clone();
        format!("https://{endpoint}/trade-api/v2/login")
    }
    fn get_request_method(&self) -> Method{
        Method::POST
    }
    fn get_request_headers(&self) -> HeaderMap{
        let mut map = HeaderMap::new();
        map.insert(ACCEPT, "application/json".parse().unwrap());
        map.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        map
    }
    fn get_request_body(&self) -> Option<String>{
        let email = self.email.clone();
        let password = self.password.clone();
        let body = format!(
                r#"
{{
     "email": "{email}",
     "password": "{password}"
}}
                "#
        );
        Some(body)
    }
}