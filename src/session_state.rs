use std::future::{ready, Ready};

use actix_session::{Session, SessionExt};
use actix_web::{dev::Payload, FromRequest, HttpRequest};
use uuid::Uuid;

pub struct TypedSesion(Session);

impl TypedSesion {
    const USER_ID_KEY: &'static str = "user_id";

    pub fn renew(&self) {
        self.0.renew();
    }

    pub fn insert_user_id(&self, user_id: Uuid) -> Result<(), serde_json::Error> {
        self.0.insert(Self::USER_ID_KEY, user_id)
    }

    pub fn get_user_id(&self) -> Result<Option<Uuid>, serde_json::Error> {
        self.0.get(Self::USER_ID_KEY)
    }

    pub fn log_out(self) {
        self.0.purge()
    }
}

impl FromRequest for TypedSesion {
    // Return the same error returned by the implementation of 'FromRequest' for 'Session'
    type Error = <Session as FromRequest>::Error;

    type Future = Ready<Result<TypedSesion, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(Ok(TypedSesion(req.get_session())))
    }
}
