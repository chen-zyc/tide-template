use crate::prelude::*;
use crate::web::State;
use tide::{Request, Response, StatusCode};

pub fn register_routers(app: &mut tide::Server<State>)
where
    State: Clone + Send + Sync + 'static,
{
    app.at("/index").get(index);
}

async fn index(_req: Request<State>) -> Result {
    info!("call index router");
    Ok(Response::builder(StatusCode::Ok)
        .body("hello world!")
        .build())
}
