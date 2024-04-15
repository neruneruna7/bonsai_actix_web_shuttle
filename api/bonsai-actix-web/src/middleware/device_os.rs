use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, Error, HttpMessage};
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