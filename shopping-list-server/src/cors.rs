use rocket::fairing::{self, Fairing};
use rocket::http::Method;
use rocket::response::{self, Responder, Response};
use std::collections::HashSet;

pub struct CORS<R> {
    responder: R,
    allow_origin: &'static str,
    allow_credentials: bool,
    allow_headers: HashSet<&'static str>,
    allow_methods: HashSet<Method>,
}

pub struct CorsFairing {
    allow_origin: &'static str,
}

impl CorsFairing {
    pub fn any() -> Self {
        Self { allow_origin: "*"}
    }
}

#[rocket::async_trait]
impl Fairing for CorsFairing {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "CORS",
            kind: fairing::Kind::Response,
        }
    }
    async fn on_response<'r>(&self, _req: &'r rocket::Request<'_>, res: &mut Response<'r>) {
        res.set_raw_header("Access-Control-Allow-Origin", self.allow_origin);
    }
}

pub type PreflightCORS = CORS<()>;

impl PreflightCORS {
    pub fn preflight(origin: &'static str) -> PreflightCORS {
        CORS::origin((), origin)
    }
}

impl<'r, 'o: 'r, R: Responder<'r, 'o>> CORS<R> {
    pub fn origin(responder: R, origin: &'static str) -> CORS<R> {
        CORS {
            responder,
            allow_origin: origin,
            allow_credentials: false,
            allow_headers: HashSet::new(),
            allow_methods: HashSet::new(),
        }
    }

    pub fn any(responder: R) -> CORS<R> {
        CORS::origin(responder, "*")
    }

    pub fn credentials(mut self, value: bool) -> CORS<R> {
        self.allow_credentials = value;
        self
    }

    pub fn methods(mut self, methods: Vec<Method>) -> CORS<R> {
        for method in methods {
            self.allow_methods.insert(method);
        }

        self
    }

    pub fn headers(mut self, headers: Vec<&'static str>) -> CORS<R> {
        for header in headers {
            self.allow_headers.insert(header);
        }

        self
    }

    // TODO: Add more builder methods to set the rest of the fields.
}

impl<'r, 'o: 'r, R: Responder<'r, 'o>> Responder<'r, 'o> for CORS<R> {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> response::Result<'o> {
        let mut response = Response::build_from(self.responder.respond_to(request)?)
            .raw_header("Access-Control-Allow-Origin", self.allow_origin)
            .finalize();

        response.set_raw_header(
            "Access-Control-Allow-Credentials",
            if self.allow_credentials {
                "true"
            } else {
                "false"
            },
        );

        if !self.allow_methods.is_empty() {
            let mut methods = String::with_capacity(self.allow_methods.len() * 7);
            for (i, method) in self.allow_methods.iter().enumerate() {
                if i != 0 {
                    methods.push_str(", ")
                }
                methods.push_str(method.as_str());
            }

            response.set_raw_header("Access-Control-Allow-Methods", methods);
        }

        if !self.allow_headers.is_empty() {
            let mut headers = String::with_capacity(self.allow_headers.len() * 15);
            for (i, header) in self.allow_headers.iter().enumerate() {
                if i != 0 {
                    headers.push_str(", ")
                }
                headers.push_str(header);
            }
            response.set_raw_header("Access-Control-Allow-Headers", headers);
        }

        Ok(response)
    }
}
