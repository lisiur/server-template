use axum::{
    body::Body,
    response::{IntoResponse, Response},
};
use axum_extra::extract::cookie::Cookie;
use http::{
    HeaderMap, HeaderValue, StatusCode,
    header::{IntoHeaderName, SET_COOKIE},
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct Null;

impl Serialize for Null {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_none() // Serializes as `null`
    }
}

#[derive(Serialize, ToSchema)]
pub struct ResponseJson<T: Serialize> {
    data: T,
}

pub type ResponseJsonNull = ResponseJson<Null>;

#[derive(Serialize)]
pub struct ResponseErrorJson {
    code: String,
    message: String,
}

impl ResponseErrorJson {
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}

#[derive(Default)]
pub struct ApiResponse {
    status: StatusCode,
    headers: HeaderMap,
    body: Option<Body>,
}

impl ApiResponse {
    pub fn json<T: Serialize + 'static>(data: T) -> Self {
        let mut response = Self::default();
        response.set_body_json(data);
        response
    }

    #[allow(unused)]
    pub fn text(data: String) -> Self {
        let mut response = Self::default();
        response.set_body_text(data);
        response
    }

    pub fn null() -> Self {
        Self::json(Null)
    }

    #[allow(dead_code)]
    pub fn insert_header<K: IntoHeaderName, V: Into<HeaderValue>>(
        &mut self,
        key: K,
        value: V,
    ) -> &mut Self {
        self.headers.insert(key, value.into());
        self
    }

    #[allow(dead_code)]
    pub fn append_header<K: IntoHeaderName, V: Into<HeaderValue>>(
        &mut self,
        key: K,
        value: V,
    ) -> &mut Self {
        self.headers.append(key, value.into());
        self
    }

    pub fn set_body_json<T: Serialize>(&mut self, body: T) -> &mut Self {
        self.body = Some(
            serde_json::to_string(&ResponseJson { data: body })
                .unwrap()
                .into(),
        );
        self
    }

    pub fn set_body_text(&mut self, body: String) -> &mut Self {
        self.body = Some(body.into());
        self
    }

    pub fn set_cookie(&mut self, cookie: Cookie) -> &mut Self {
        self.insert_header(
            SET_COOKIE,
            HeaderValue::from_str(&cookie.to_string()).unwrap(),
        );

        self
    }
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        let ApiResponse {
            status,
            headers,
            body: payload,
        } = self;

        let mut response = if let Some(payload) = payload {
            payload.into_response()
        } else {
            Response::default()
        };
        *response.status_mut() = status;
        response.headers_mut().extend(headers);

        response
    }
}

impl<T: Serialize + 'static> From<T> for ApiResponse {
    fn from(value: T) -> Self {
        Self::json(value)
    }
}

#[derive(Serialize, ToSchema)]
pub struct PaginatedData<T> {
    pub records: Vec<T>,
    pub total: i64,
}

impl<T, U: From<T>> From<(Vec<T>, i64)> for PaginatedData<U> {
    fn from(value: (Vec<T>, i64)) -> Self {
        Self {
            records: value.0.into_iter().map(|item| item.into()).collect(),
            total: value.1,
        }
    }
}
