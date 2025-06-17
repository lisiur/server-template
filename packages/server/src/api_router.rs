use axum::{Router, handler::Handler};
use utoipa::{Path, openapi::HttpMethod};

pub struct ApiRouter<S = ()> {
    pub router: Router<S>,
}

impl<S: Clone + Send + Sync + 'static> ApiRouter<S> {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }
}

impl<S: Clone + Send + Sync + 'static> ApiRouter<S> {
    pub fn route<T, P>(self, _api_path: P, handler: impl Handler<T, S>) -> Self
    where
        P: Path,
        T: 'static,
    {
        let mut router = self;
        let path = P::path();
        let method = P::methods();
        for method in method.into_iter() {
            let method_router = match method {
                HttpMethod::Get => axum::routing::get(handler.clone()),
                HttpMethod::Post => axum::routing::post(handler.clone()),
                HttpMethod::Put => axum::routing::put(handler.clone()),
                HttpMethod::Delete => axum::routing::delete(handler.clone()),
                HttpMethod::Patch => axum::routing::patch(handler.clone()),
                HttpMethod::Head => axum::routing::head(handler.clone()),
                HttpMethod::Options => axum::routing::options(handler.clone()),
                HttpMethod::Trace => axum::routing::trace(handler.clone()),
            };
            router = Self {
                router: router.router.route(&path, method_router),
            }
        }

        router
    }
}

#[macro_export]
macro_rules! init_router {
    ($($handler:ident),*) => {
        pub(crate) fn init() -> axum::Router {
            let mut router = crate::api_router::ApiRouter::new();
            $(
                router = router.route(paste::paste! {[<__path_ $handler>]}, $handler);
            )*
            router.router
        }
    };
}
