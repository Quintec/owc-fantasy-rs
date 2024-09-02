use actix_session::SessionExt;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorForbidden, ErrorUnauthorized},
    middleware::Next,
    Error,
};

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    if req.path().starts_with("/api/auth") {
        return next.call(req).await;
    }

    let session = req.get_session();
    let user_id = session.get::<i32>("user_id").unwrap_or(None);
    let Some(user_id) = user_id else {
        return Err(ErrorUnauthorized("Unauthorized, please sign in"));
    };

    if !is_authorized(user_id, req.path()) {
        return Err(ErrorForbidden("Forbidden"));
    }

    let res = next.call(req).await?;
    Ok(res)
}

fn is_authorized(user_id: i32, path: &str) -> bool {
    // Implement your authorization logic here based on user ID and request path
    println!("User ID: {}, path: {}", user_id, path);
    true
}
