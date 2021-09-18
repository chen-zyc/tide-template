use crate::config::*;
use crate::prelude::*;
use crate::routes;
use std::str::FromStr;
use tide::http::headers::{self, HeaderName};
use tide::utils::async_trait;
use tide::{Middleware, Next, Request};
use tide_tracing_middleware::TracingMiddleware;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct State {}

impl State {
    fn new() -> Self {
        Self {}
    }
}

pub async fn init() -> Result<()> {
    let address: String = get_config(CONFIG_KEY_SERVER_ADDRESS);
    let log_format: String = get_config(CONFIG_KEY_ACCESS_LOG_FORMAT);
    let request_id_hdr: String =
        get_config_default(CONFIG_KEY_REQUEST_ID_HEADER, String::default());
    let request_id_header = if !request_id_hdr.is_empty() {
        Some(HeaderName::from_string(request_id_hdr)?)
    } else {
        None
    };
    let pkg_version: String = get_config_default("CARGO_PKG_VERSION", "0.1.0".to_string());
    let server_name: String = get_config_default(CONFIG_KEY_SERVER_NAME, String::default());
    let server_name = if !server_name.is_empty() {
        Some(format!("{} {}", server_name, pkg_version))
    } else {
        None
    };

    let mut app = tide::with_state(State::new());
    // 生成 request id.
    app.with(tide::utils::Before(|mut req: Request<State>| async {
        let request_id = Uuid::new_v4().to_simple().to_string();
        req.set_ext(RequestId(request_id));
        req
    }));
    app.with(init_tracing_middleware(&log_format));
    app.with(CommonMiddleware::new(request_id_header, server_name));

    routes::register_routers(&mut app);

    app.listen(address).await?;

    Ok(())
}

fn init_tracing_middleware<State>(format: &str) -> TracingMiddleware<State>
where
    State: Clone + Send + Sync + 'static,
{
    TracingMiddleware::new(format)
        .custom_request_replace("ALL_REQ_HEADERS", |req| headers_str(req.iter()))
        .custom_response_replace("ALL_RES_HEADERS", |res| headers_str(res.iter()))
        .gen_tracing_span(|req| {
            if let Some(id) = req.ext::<RequestId>() {
                tracing::info_span!("R", "{}", &id.0)
            } else {
                tracing::info_span!("R", "{}", Uuid::new_v4().to_simple().to_string())
            }
        })
}

fn headers_str(iter: headers::Iter<'_>) -> String {
    let pairs: Vec<String> = iter.map(|(k, v)| format!("{}:{}", k, v)).collect();
    "{".to_owned() + &pairs.join(",") + "}"
}

struct CommonMiddleware {
    request_id_hdr: Option<HeaderName>,
    server_hdr: HeaderName,
    server_name: Option<String>,
}

impl CommonMiddleware {
    fn new(request_id_hdr: Option<HeaderName>, server_name: Option<String>) -> Self {
        Self {
            request_id_hdr,
            server_hdr: HeaderName::from_str("server").unwrap(),
            server_name,
        }
    }
}

#[derive(Clone, Debug)]
struct RequestId(String);

#[async_trait]
impl<State> Middleware<State> for CommonMiddleware
where
    State: Clone + Send + Sync + 'static,
{
    async fn handle(&self, request: Request<State>, next: Next<'_, State>) -> Result {
        let req_id = request.ext::<RequestId>().map(|id| id.clone());

        let mut res = next.run(request).await;

        if let Some(name) = &self.server_name {
            res.insert_header(&self.server_hdr, name);
        }
        if let Some(hdr) = &self.request_id_hdr {
            if let Some(id) = req_id {
                res.insert_header(hdr, id.0);
            }
        }

        Ok(res)
    }
}
