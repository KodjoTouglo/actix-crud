use std::sync::Arc;
use chrono::{DateTime, Utc};
use models::entities::user::UserModel;
use actix_web::{cookie::Cookie, http::Uri, HttpRequest};

#[allow(dead_code)]
pub struct BackendContext {
    uri: Uri,
    http_request: Option<Arc<HttpRequest>>,
    response_cookies: Vec<Cookie<'static>>,
    current_time: Option<DateTime<Utc>>,
    authenticated_user: Option<Arc<UserModel>>,
}

impl BackendContext {
    pub fn from_user(user: UserModel) -> Self {
        let user = Arc::new(user);
        let mut context = Self {
            uri: Default::default(),
            http_request: Default::default(),
            response_cookies: Default::default(),
            current_time: Default::default(),
            authenticated_user: Default::default()
        };
        context.set_authenticated_user(&user);
        context
    }

    pub fn uri(&self) -> Uri{ self.uri.clone()}

    pub fn set_uri(&mut self, uri: Uri) {self.uri = uri}

    pub fn response_cookies(&self) -> &Vec<Cookie<'static>> {&self.response_cookies}

    pub fn set_response_cookies(&mut self, response_cookies: Vec<Cookie<'static>>) {
        self.response_cookies = response_cookies

    }

    pub fn authenticated_user(&self) -> &Arc<UserModel> {
        &self.authenticated_user.as_ref().unwrap()
    }

    pub fn set_authenticated_user(&mut self, user: &Arc<UserModel>) {
        self.authenticated_user = Some(Arc::clone(&user));
    }

    pub fn current_time(&self) -> DateTime<Utc> {
        match &self.current_time {
            Some(t) => t.clone(),
            None => Utc::now(),
        }
    }

    pub fn set_current_time(&mut self, current_time: DateTime<Utc>) {
        self.current_time = Some(current_time);
    }
}