use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Defines a security scheme that can be used by the operations.
/// Supported schemes are HTTP authentication, an API key (either as a
/// header or as a query parameter), OAuth2's common flows (implicit, password,
/// application and access code) as defined in RFC6749, and OpenID Connect Discovery.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum SecurityScheme {
    #[serde(rename = "apiKey")]
    APIKey {
        #[serde(rename = "in")]
        location: APIKeyLocation,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    #[serde(rename = "http")]
    HTTP {
        scheme: String,
        #[serde(rename = "bearerFormat")]
        bearer_format: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    #[serde(rename = "oauth2")]
    OAuth2 {
        flows: OAuth2Flows,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    #[serde(rename = "openIdConnect")]
    OpenIDConnect {
        #[serde(rename = "openIdConnectUrl")]
        open_id_connect_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum APIKeyLocation {
    #[serde(rename = "query")]
    Query,
    #[serde(rename = "header")]
    Header,
    #[serde(rename = "cookie")]
    Cookie,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2Flows {
    #[serde(flatten)]
    pub implicit: Option<OAuth2Flow>,
    #[serde(flatten)]
    pub password: Option<OAuth2Flow>,
    #[serde(flatten)]
    pub client_credentials: Option<OAuth2Flow>,
    #[serde(flatten)]
    pub authorization_code: Option<OAuth2Flow>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OAuth2Flow {
    #[serde(rename = "implicit")]
    Implicit {
        #[serde(rename = "authorizationUrl")]
        authorization_url: String,
        #[serde(rename = "refreshUrl")]
        refresh_url: Option<String>,
        #[serde(default)]
        scopes: IndexMap<String, String>,
    },
    #[serde(rename = "password")]
    Password {
        #[serde(rename = "refreshUrl")]
        refresh_url: Option<String>,
        #[serde(rename = "tokenUrl")]
        token_url: String,
        #[serde(default)]
        scopes: IndexMap<String, String>,
    },
    #[serde(rename = "clientCredentials")]
    ClientCredentials {
        #[serde(rename = "refreshUrl")]
        refresh_url: Option<String>,
        #[serde(rename = "tokenUrl")]
        token_url: String,
        #[serde(default)]
        scopes: IndexMap<String, String>,
    },
    #[serde(rename = "authorizationCode")]
    AuthorizationCode {
        #[serde(rename = "authorizationUrl")]
        authorization_url: String,
        #[serde(rename = "tokenUrl")]
        token_url: String,
        #[serde(rename = "refreshUrl")]
        refresh_url: Option<String>,
        #[serde(default)]
        scopes: IndexMap<String, String>,
    },
}
