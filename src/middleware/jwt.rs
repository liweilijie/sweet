use crate::util::jwt::decode_token;
use axum::body;
use axum::body::BoxBody;
use axum::http::{Request, Response, StatusCode};
use regex::Regex;
use serde_json::json;
use std::collections::HashMap;
use tower_http::auth::AuthorizeRequest;
use tracing::debug;

const AUTH_HEADER: &'static str = "Authorization";

#[derive(Debug, Clone)]
pub struct JWT {
    unless: HashMap<String, String>,
    reject_reason: Option<String>,
}

impl JWT {
    pub fn new(unless: HashMap<String, String>) -> Self {
        JWT {
            unless,
            reject_reason: None,
        }
    }
}

fn is_method_allowed(method: &str, methods: &str) -> bool {
    for m in methods.split("|") {
        if method == m.to_uppercase() {
            return true;
        }
    }

    false
}

fn match_rules(rules: HashMap<String, String>, uri: String, method: String) -> bool {
    for (rule, methods) in rules {
        debug!("{rule} {uri}");
        let reg = Regex::new(&rule).unwrap();
        if reg.is_match(&uri) {
            return is_method_allowed(&method, &methods);
        }
    }
    false
}

impl<B> AuthorizeRequest<B> for JWT {
    type ResponseBody = BoxBody;

    fn authorize(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
        if match_rules(
            self.unless.to_owned(),
            request.uri().to_string(),
            request.method().to_string(),
        ) {
            return Ok(());
        }

        if let Some(auth_string) = request.headers().get(AUTH_HEADER) {
            let auth_str = auth_string.to_str().unwrap();
            if auth_str.starts_with("Bearer ") {
                let auth_str = auth_str.replace("Bearer ", "");
                let decoded = decode_token(&auth_str);
                match decoded {
                    Ok(token_data) => {
                        let auth = token_data.claims.auth;
                        request.extensions_mut().insert(auth);
                    }
                    Err(e) => {
                        self.reject_reason =
                            Some(format!("请求头 Authorization 解析错误: {:?}", e));
                    }
                }
            } else {
                self.reject_reason = Some("请求头 Authorization 不是 Bearer 开头".to_string());
            }
        } else {
            self.reject_reason = Some("请求头 Authorization 不能为空".to_string());
        }

        if let Some(reject_reason) = self.reject_reason.clone() {
            let json_body = json!({"code":-1, "message": reject_reason}).to_string();
            let body = body::boxed(body::Full::from(json_body));
            let unauthorized_response = Response::builder()
                .status(StatusCode::OK)
                .body(body)
                .unwrap();
            Err(unauthorized_response)
        } else {
            Ok(())
        }
    }
}
