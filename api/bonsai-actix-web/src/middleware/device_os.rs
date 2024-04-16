use actix_web::{
    body::MessageBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use actix_web_lab::middleware::Next;
use tracing::{instrument, info};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DeviceOs {
    name: String,
}

impl DeviceOs {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}


#[instrument(skip(req, next))]
pub async fn device_os_handler(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    info!("start device_os_handler");

    {
        // ユーザーエージェントを取得
        let user_agent = req
            .headers()
            .get("User-Agent")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        // UAを保存
        let device_os = DeviceOs::new(user_agent);

        let _a = req.extensions_mut().insert(device_os);
        let a = req.extensions();
        let a = a.get::<DeviceOs>();
        // a.insert("device_os");
        info!("lab capture os{:?}", a)

    }

    let res = next.call(req).await?;
    info!("end device_os_handler");
    Ok(res)
}

// actix_web_lab無しで実装
// できなかった

use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};

pub struct CaptureOs;

// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for CaptureOs
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    // setting up the types for the middleware to work
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CaptureOsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    // this immediately returns the middleware
    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CaptureOsMiddleware { service }))
    }
}

#[derive(Debug,Clone)]
pub struct CaptureOsMiddleware<S> {
    /// The next service to call
    service: S,
}

// This future doesn't have the requirement of being `Send`.
// See: futures_util::future::LocalBoxFuture
type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

// `S`: type of the wrapped service
// `B`: type of the body - try to be generic over the body where possible
impl<S, B> Service<ServiceRequest> for CaptureOsMiddleware<S>
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

    #[instrument(skip(self, req), name="capture_os")]
    fn call(&self, req: ServiceRequest) -> Self::Future {
        info!("Capture OS middlerare Start");
        // spanの名前がcallにになってしまい，どのミドルウェアの中なのか分からない問題
        // せっかく構造化したロギングができるのに，その意味が薄れてしまう
        // instrumentマクロで，nameでスパン名を指定することで対策は可能

        // A more complex middleware, could return an error or an early response here.
        // 複雑なミドルウェアの場合，ここでエラーを返したり早期リターンをすることができる

        // スコープを切ることで，reqへの参照をもつ変数を破棄している
        // 後続のcallに渡すときに，参照をもった変数がいると生存期間の関係でエラーがでる
        let ua = {
            // ユーザーエージェントを取得
            let user_agent = req
                .headers()
                .get("User-Agent")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            // UAを保存
            let device_os = DeviceOs::new(user_agent);
    
            let _ = req.extensions_mut().insert(device_os);

            let ext = req.extensions();
            let ua = ext.get::<DeviceOs>();
            info!("Capture from response, UA = {:?}", ua);

            ua.unwrap().clone()
        };




        // we do not immediately await this, which means nothing happens
        // this future gets moved into a Box
        let fut = self.service.call(req);

        info!("Capture OS middlerare Finish");
        Box::pin(async move {
            // 実行される時間軸としては，Goのミドルウェアでのクロージャ内に相当すると思われる
            // ここの中ではreqをとれないので，クロージャ外で処理してから，クロージャ内にmoveしてあれこれが良さげ？
            info!("Closure start Capture from response, UA = {:?}\n", ua);
        

            // this future gets awaited now
            let res = fut.await?;

            // we can now do any work we need to after the request

            // ~~多分，パニックハンドラの場合，回復処理とかはここに書くのだと思う
            // futをawaitで待った後に実行される
            // すなわち，Goでいうdeferに書いた関数と同じタイミングで実行されると考えられる~~
            // 違うな，ServeHttpを呼んだあとの部分に相当する
            info!("Closure end Capture from response, UA = {:?}\n", ua);
            Ok(res)
        })
    }
}
