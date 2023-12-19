use std::future::{Ready, ready};
use actix_identity::IdentityExt;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpResponse};
use actix_web::body::{MessageBody};
use actix_web::web::Data;
use futures_util::future::LocalBoxFuture;
use crate::db::ConnPool;

pub struct AuthenticatedRoutes {
    admin: bool,
}

impl AuthenticatedRoutes {
    pub fn admin() -> Self {
        Self { admin: true }
    }

    #[allow(dead_code)]
    pub fn user() -> Self {
        Self { admin: false }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthenticatedRoutes
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static + MessageBody, HttpResponse<B>: From<HttpResponse>
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticatedRoutesMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticatedRoutesMiddleware { service, admin: self.admin }))
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct AuthenticatedRoutesMiddleware<S> {
    service: S,
    admin: bool
}

impl<S, B> Service<ServiceRequest> for AuthenticatedRoutesMiddleware<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static + MessageBody, HttpResponse<B>: From<HttpResponse>
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let identity = req.get_identity();


        if identity.is_err() {
            return unauthorized_response(req)
        }

        let pool = req.app_data::<Data<ConnPool>>().unwrap();
        let mut conn = pool.get().unwrap();

        let user = crate::db::users::get_user_by_email(&mut conn, &identity.unwrap().id().unwrap());

        if user.is_err() {
            return unauthorized_response(req)
        }

        let user = user.unwrap();

        if self.admin && !user.admin {
            return unauthorized_response(req)
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let response = fut.await?;
            Ok(response)
        })
    }
}

fn unauthorized_response<B>(req: ServiceRequest) -> LocalBoxFuture<'static, Result<ServiceResponse<B>, Error>>
    where
        B: 'static + MessageBody, HttpResponse<B>: From<HttpResponse>
{
    Box::pin(async move {
        let response : HttpResponse<B> = HttpResponse::Unauthorized().finish().into();
        let response = req.into_response::<B, HttpResponse<B>>(response);
        Ok(response)
    })
}