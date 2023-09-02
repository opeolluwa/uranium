/// the multiplexers is adapated from axum example
/// <https://github.com/tokio-rs/axum/blob/axum-v0.6.20/examples/rest-grpc-multiplex/src/main.rs>
///
/// The multiplexer shall allow request handlers to be consumed by http server and clients
/// FOr example
///
/// REST ->  curl http://0.0.0.0:4564
/// gPRC -. grpcurl -plaintext -import-path ./proto -proto uranium.proto '[::1]:50051' uranium.Uranium/HealthCheck
///
/// would both return "Service health!"
///
///
use axum::{body::BoxBody, http::header::CONTENT_TYPE, response::IntoResponse};
use futures::{future::BoxFuture, ready};
use hyper::{Body, Request, Response};
use std::{
    convert::Infallible,
    task::{Context, Poll},
};
use tower::Service;

#[derive(Debug, Default)]
pub struct Multiplexers<A, B> {
    rest: A,
    grpc: B,
    rest_ready: bool,
    grpc_ready: bool,
}

impl<A, B> Multiplexers<A, B> {
    pub fn new(rest: A, grpc: B) -> Self {
        Self {
            rest,
            grpc,
            rest_ready: false,
            grpc_ready: false,
        }
    }

    pub fn rest_ready(&mut self) {
        self.rest_ready = true;
    }

    pub fn grpc_ready(&mut self) {
        self.grpc_ready = true;
    }
}

/// implement Clone trait for Multiplexers
impl<A: Clone + std::default::Default, B: Clone + std::default::Default> Clone
    for Multiplexers<A, B>
{
    fn clone(&self) -> Self {
        Self {
            rest: self.rest.clone(),
            grpc: self.grpc.clone(),
            /* result the grpc_ready and rest_ready to false  // because the cloned services probably wont be ready */
            ..Default::default() // rest_ready: false,
                                 // grpc_ready: false,
        }
    }
}

impl<A, B> Service<Request<Body>> for Multiplexers<A, B>
where
    A: Service<Request<Body>, Error = Infallible>,
    A::Response: IntoResponse,
    A::Future: Send + 'static,
    B: Service<Request<Body>>,
    B::Response: IntoResponse,
    B::Future: Send + 'static,
{
    type Response = Response<BoxBody>;
    type Error = B::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // drive readiness for each inner service and record which is ready
        loop {
            match (self.rest_ready, self.grpc_ready) {
                (true, true) => {
                    return Ok(()).into();
                }
                (false, _) => {
                    ready!(self.rest.poll_ready(cx)).map_err(|err| match err {})?;
                    self.rest_ready = true;
                }
                (_, false) => {
                    ready!(self.grpc.poll_ready(cx))?;
                    self.grpc_ready = true;
                }
            }
        }
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        // require users to call `poll_ready` first, if they don't we're allowed to panic
        // as per the `tower::Service` contract
        assert!(
            self.grpc_ready,
            "grpc service not ready. Did you forget to call `poll_ready`?"
        );
        assert!(
            self.rest_ready,
            "rest service not ready. Did you forget to call `poll_ready`?"
        );

        // if we get a grpc request call the grpc service, otherwise call the rest service
        // when calling a service it becomes not-ready so we have drive readiness again
        if is_grpc_request(&req) {
            self.grpc_ready = false;
            let future = self.grpc.call(req);
            Box::pin(async move {
                let res = future.await?;
                Ok(res.into_response())
            })
        } else {
            self.rest_ready = false;
            let future = self.rest.call(req);
            Box::pin(async move {
                let res = future.await.map_err(|err| match err {})?;
                Ok(res.into_response())
            })
        }
    }
}

fn is_grpc_request<B>(req: &Request<B>) -> bool {
    req.headers()
        .get(CONTENT_TYPE)
        .map(|content_type| content_type.as_bytes())
        .filter(|content_type| content_type.starts_with(b"application/grpc"))
        .is_some()
}
