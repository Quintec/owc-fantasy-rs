use actix_session::SessionExt;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorForbidden, ErrorInternalServerError, ErrorUnauthorized},
    middleware::Next,
    Error,
};

const ADMIN_ID: i32 = 15458667;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    if req.path().starts_with("/api/auth") {
        return next.call(req).await;
    }

    let session = req.get_session();
    let user_id = session.get::<i32>("user_id").unwrap_or(None);
    if user_id.is_none() {
        return Err(ErrorUnauthorized("Unauthorized, please sign in"));
    };

    let res = next.call(req).await?;
    Ok(res)
}

pub async fn same_id_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let session = req.get_session();
    let user_id = session.get::<i32>("user_id").unwrap_or(None);
    let Some(user_id) = user_id else {
        return Err(ErrorUnauthorized("Unauthorized, please sign in"));
    };

    let Some(path_id) = req.match_info().get("id") else {
        return Err(ErrorInternalServerError("Expected id in path"));
    };

    let Ok(path_id) = path_id.parse::<i32>() else {
        return Err(ErrorInternalServerError("Error parsing path id"));
    };

    if user_id != path_id && user_id != ADMIN_ID {
        return Err(ErrorForbidden("Forbidden"));
    }

    let res = next.call(req).await?;
    Ok(res)
}

pub async fn admin_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let session = req.get_session();
    let user_id = session.get::<i32>("user_id").unwrap_or(None);
    let Some(user_id) = user_id else {
        return Err(ErrorUnauthorized("Unauthorized, please sign in"));
    };

    if user_id != ADMIN_ID {
        return Err(ErrorForbidden("Forbidden"));
    }

    let res = next.call(req).await?;
    Ok(res)
}
