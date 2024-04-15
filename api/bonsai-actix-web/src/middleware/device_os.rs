use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};

use actix_web::{
    body::MessageBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web::ServiceConfig,
    Error, HttpMessage,
};
use actix_web_lab::middleware::Next;

pub async fn device_os_handler(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    println!("start device_os_handler");
    // ユーザーエージェントを取得
    let user_agent = req.headers().get("User-Agent").unwrap().to_str().unwrap();
    println!("User-Agent: {}", user_agent);
    let res = next.call(req).await?;
    println!("end device_os_handler");
    Ok(res)
}

// actix_web_lab無しで実装

pub struct DeviceOs;

impl<S, B> Transform<S, ServiceConfig> for DeviceOs
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = DeviceOsMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(DeviceOsMiddleware { service }))
    }
}

pub struct DeviceOsMiddleware<S> {
    service: S,
}
// This future doesn't have the requirement of being `Send`.
// See: futures_util::future::LocalBoxFuture
type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

// `S`: type of the wrapped service
// `B`: type of the body - try to be generic over the body where possible
impl<S, B> Service<ServiceRequest> for DeviceOsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    // This service is ready when its next service is ready
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        // A more complex middleware, could return an error or an early response here.

        // we do not immediately await this, which means nothing happens
        // this future gets moved into a Box
        let fut = self.service.call(req);

        Box::pin(async move {
            // this future gets awaited now
            let res = fut.await?;

            // we can now do any work we need to after the request
            println!("Hi from response");
            Ok(res)
        })
    }
}
