use std::fmt::Debug;
use std::task::{Context, Poll};

use axum::routing::Route;
use tower::{Layer, Service};
#[derive(Debug, Copy, Clone)]
pub struct BmbpAuthLayer {}

impl BmbpAuthLayer {
    pub fn new() -> Self {
        BmbpAuthLayer {}
    }
}

impl<S> Layer<S> for BmbpAuthLayer {
    type Service = BmbpAuthMiddleware<S>;
    fn layer(&self, inner: S) -> Self::Service {
        BmbpAuthMiddleware { service: inner }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct BmbpAuthMiddleware<S> {
    service: S,
}

impl<S, Request> Service<Request> for BmbpAuthMiddleware<S>
where
    S: Service<Request>,
    Request: Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        println!("从此处，做些处理");
        self.service.call(req)
    }
}
