use actix_web::{
    get,
    web::{self, ServiceConfig},
};
use actix_web_lab::middleware::from_fn;
use shuttle_actix_web::ShuttleActixWeb;

mod middleware;

use middleware::device_os::device_os_handler;

#[get("/")]
async fn hello_world() -> &'static str {
    println!("hello_world");
    "Hello World!"
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
                .wrap(from_fn(device_os_handler))
                .service(hello_world),
        );
    };
    Ok(config.into())
}
