use futures::{future, future::BoxFuture, Stream, stream, future::FutureExt, stream::TryStreamExt};
use hyper::{Request, Response, StatusCode, Body, HeaderMap};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use log::warn;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};
use swagger::{ApiError, BodyExt, Has, RequestParser, XSpanIdString};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use url::form_urlencoded;

#[allow(unused_imports)]
use crate::models;
use crate::header;

pub use crate::context;

type ServiceFuture = BoxFuture<'static, Result<Response<Body>, crate::ServiceError>>;

use crate::{Api,
     AddressbookAddressKeyAddrtypeDeleteResponse,
     AddressbookAddressKeyAddrtypeGetResponse,
     AddressbookAddressKeyAddrtypeHeadResponse,
     AddressbookAddressKeyAddrtypePutResponse,
     AddressbookAddressKeyCityDeleteResponse,
     AddressbookAddressKeyCityGetResponse,
     AddressbookAddressKeyCityHeadResponse,
     AddressbookAddressKeyCityPutResponse,
     AddressbookAddressKeyCountryDeleteResponse,
     AddressbookAddressKeyCountryGetResponse,
     AddressbookAddressKeyCountryHeadResponse,
     AddressbookAddressKeyCountryPutResponse,
     AddressbookAddressKeyDeleteResponse,
     AddressbookAddressKeyGetResponse,
     AddressbookAddressKeyHeadResponse,
     AddressbookAddressKeyPatchResponse,
     AddressbookAddressKeyPersonUuidDeleteResponse,
     AddressbookAddressKeyPersonUuidGetResponse,
     AddressbookAddressKeyPersonUuidHeadResponse,
     AddressbookAddressKeyPersonUuidPutResponse,
     AddressbookAddressKeyPutResponse,
     AddressbookAddressKeyStateDeleteResponse,
     AddressbookAddressKeyStateGetResponse,
     AddressbookAddressKeyStateHeadResponse,
     AddressbookAddressKeyStatePutResponse,
     AddressbookAddressKeyStreetDeleteResponse,
     AddressbookAddressKeyStreetGetResponse,
     AddressbookAddressKeyStreetHeadResponse,
     AddressbookAddressKeyStreetPutResponse,
     AddressbookDeleteResponse,
     AddressbookGetResponse,
     AddressbookHeadResponse,
     AddressbookPatchResponse,
     AddressbookPostResponse
};

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/addressbook$",
            r"^/addressbook/(?P<address_key>[^/?#]*)$",
            r"^/addressbook/(?P<address_key>[^/?#]*)/addrtype$",
            r"^/addressbook/(?P<address_key>[^/?#]*)/city$",
            r"^/addressbook/(?P<address_key>[^/?#]*)/country$",
            r"^/addressbook/(?P<address_key>[^/?#]*)/person_uuid$",
            r"^/addressbook/(?P<address_key>[^/?#]*)/state$",
            r"^/addressbook/(?P<address_key>[^/?#]*)/street$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_ADDRESSBOOK: usize = 0;
    pub(crate) static ID_ADDRESSBOOK_ADDRESS_KEY: usize = 1;
    lazy_static! {
        pub static ref REGEX_ADDRESSBOOK_ADDRESS_KEY: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/addressbook/(?P<address_key>[^/?#]*)$")
                .expect("Unable to create regex for ADDRESSBOOK_ADDRESS_KEY");
    }
    pub(crate) static ID_ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE: usize = 2;
    lazy_static! {
        pub static ref REGEX_ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/addressbook/(?P<address_key>[^/?#]*)/addrtype$")
                .expect("Unable to create regex for ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE");
    }
    pub(crate) static ID_ADDRESSBOOK_ADDRESS_KEY_CITY: usize = 3;
    lazy_static! {
        pub static ref REGEX_ADDRESSBOOK_ADDRESS_KEY_CITY: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/addressbook/(?P<address_key>[^/?#]*)/city$")
                .expect("Unable to create regex for ADDRESSBOOK_ADDRESS_KEY_CITY");
    }
    pub(crate) static ID_ADDRESSBOOK_ADDRESS_KEY_COUNTRY: usize = 4;
    lazy_static! {
        pub static ref REGEX_ADDRESSBOOK_ADDRESS_KEY_COUNTRY: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/addressbook/(?P<address_key>[^/?#]*)/country$")
                .expect("Unable to create regex for ADDRESSBOOK_ADDRESS_KEY_COUNTRY");
    }
    pub(crate) static ID_ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID: usize = 5;
    lazy_static! {
        pub static ref REGEX_ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/addressbook/(?P<address_key>[^/?#]*)/person_uuid$")
                .expect("Unable to create regex for ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID");
    }
    pub(crate) static ID_ADDRESSBOOK_ADDRESS_KEY_STATE: usize = 6;
    lazy_static! {
        pub static ref REGEX_ADDRESSBOOK_ADDRESS_KEY_STATE: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/addressbook/(?P<address_key>[^/?#]*)/state$")
                .expect("Unable to create regex for ADDRESSBOOK_ADDRESS_KEY_STATE");
    }
    pub(crate) static ID_ADDRESSBOOK_ADDRESS_KEY_STREET: usize = 7;
    lazy_static! {
        pub static ref REGEX_ADDRESSBOOK_ADDRESS_KEY_STREET: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/addressbook/(?P<address_key>[^/?#]*)/street$")
                .expect("Unable to create regex for ADDRESSBOOK_ADDRESS_KEY_STREET");
    }
}

pub struct MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C, Target> hyper::service::Service<Target> for MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    type Response = Service<T, C>;
    type Error = crate::ServiceError;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, target: Target) -> Self::Future {
        futures::future::ok(Service::new(
            self.api_impl.clone(),
        ))
    }
}

fn method_not_allowed() -> Result<Response<Body>, crate::ServiceError> {
    Ok(
        Response::builder().status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .expect("Unable to create Method Not Allowed response")
    )
}

pub struct Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C> Clone for Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker,
        }
    }
}

impl<T, C> hyper::service::Service<(Request<Body>, C)> for Service<T, C> where
    T: Api<C> + Clone + Send + Sync + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    type Response = Response<Body>;
    type Error = crate::ServiceError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.api_impl.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<Body>, C)) -> Self::Future { async fn run<T, C>(mut api_impl: T, req: (Request<Body>, C)) -> Result<Response<Body>, crate::ServiceError> where
        T: Api<C> + Clone + Send + 'static,
        C: Has<XSpanIdString>  + Send + Sync + 'static
    {
        let (request, context) = req;
        let (parts, body) = request.into_parts();
        let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

        match method {

            // AddressbookAddressKeyAddrtypeDelete - DELETE /addressbook/{address_key}/addrtype
            hyper::Method::DELETE if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_addrtype_delete(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyAddrtypeDeleteResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE_DELETE_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyAddrtypeGet - GET /addressbook/{address_key}/addrtype
            hyper::Method::GET if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_city = query_params.iter().filter(|e| e.0 == "city").map(|e| e.1.to_owned())
                    .next();
                let param_city = match param_city {
                    Some(param_city) => {
                        let param_city =
                            <String as std::str::FromStr>::from_str
                                (&param_city);
                        match param_city {
                            Ok(param_city) => Some(param_city),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter city - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter city")),
                        }
                    },
                    None => None,
                };
                let param_city = match param_city {
                    Some(param_city) => param_city,
                    None => return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Missing required query parameter city"))
                        .expect("Unable to create Bad Request response for missing query parameter city")),
                };

                                let result = api_impl.addressbook_address_key_addrtype_get(
                                            param_city,
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyAddrtypeGetResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE_GET_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyAddrtypeHead - HEAD /addressbook/{address_key}/addrtype
            hyper::Method::HEAD if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_addrtype_head(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyAddrtypeHeadResponse::DefaultHEADResponse
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyAddrtypePut - PUT /addressbook/{address_key}/addrtype
            hyper::Method::PUT if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_addrtype_put(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyAddrtypePutResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE_PUT_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyCityDelete - DELETE /addressbook/{address_key}/city
            hyper::Method::DELETE if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_CITY) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_CITY
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_CITY in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_CITY.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_city_delete(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyCityDeleteResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_CITY_DELETE_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyCityGet - GET /addressbook/{address_key}/city
            hyper::Method::GET if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_CITY) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_CITY
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_CITY in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_CITY.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_city = query_params.iter().filter(|e| e.0 == "city").map(|e| e.1.to_owned())
                    .next();
                let param_city = match param_city {
                    Some(param_city) => {
                        let param_city =
                            <String as std::str::FromStr>::from_str
                                (&param_city);
                        match param_city {
                            Ok(param_city) => Some(param_city),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter city - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter city")),
                        }
                    },
                    None => None,
                };
                let param_city = match param_city {
                    Some(param_city) => param_city,
                    None => return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Missing required query parameter city"))
                        .expect("Unable to create Bad Request response for missing query parameter city")),
                };

                                let result = api_impl.addressbook_address_key_city_get(
                                            param_city,
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyCityGetResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_CITY_GET_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyCityHead - HEAD /addressbook/{address_key}/city
            hyper::Method::HEAD if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_CITY) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_CITY
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_CITY in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_CITY.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_city_head(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyCityHeadResponse::DefaultHEADResponse
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyCityPut - PUT /addressbook/{address_key}/city
            hyper::Method::PUT if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_CITY) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_CITY
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_CITY in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_CITY.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_city_put(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyCityPutResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_CITY_PUT_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyCountryDelete - DELETE /addressbook/{address_key}/country
            hyper::Method::DELETE if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_COUNTRY) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_COUNTRY
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_COUNTRY in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_COUNTRY.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_country_delete(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyCountryDeleteResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_COUNTRY_DELETE_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyCountryGet - GET /addressbook/{address_key}/country
            hyper::Method::GET if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_COUNTRY) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_COUNTRY
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_COUNTRY in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_COUNTRY.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_city = query_params.iter().filter(|e| e.0 == "city").map(|e| e.1.to_owned())
                    .next();
                let param_city = match param_city {
                    Some(param_city) => {
                        let param_city =
                            <String as std::str::FromStr>::from_str
                                (&param_city);
                        match param_city {
                            Ok(param_city) => Some(param_city),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter city - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter city")),
                        }
                    },
                    None => None,
                };
                let param_city = match param_city {
                    Some(param_city) => param_city,
                    None => return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Missing required query parameter city"))
                        .expect("Unable to create Bad Request response for missing query parameter city")),
                };

                                let result = api_impl.addressbook_address_key_country_get(
                                            param_city,
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyCountryGetResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_COUNTRY_GET_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyCountryHead - HEAD /addressbook/{address_key}/country
            hyper::Method::HEAD if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_COUNTRY) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_COUNTRY
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_COUNTRY in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_COUNTRY.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_country_head(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyCountryHeadResponse::DefaultHEADResponse
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyCountryPut - PUT /addressbook/{address_key}/country
            hyper::Method::PUT if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_COUNTRY) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_COUNTRY
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_COUNTRY in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_COUNTRY.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_country_put(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyCountryPutResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_COUNTRY_PUT_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyDelete - DELETE /addressbook/{address_key}
            hyper::Method::DELETE if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_delete(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyDeleteResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_DELETE_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyGet - GET /addressbook/{address_key}
            hyper::Method::GET if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_city = query_params.iter().filter(|e| e.0 == "city").map(|e| e.1.to_owned())
                    .next();
                let param_city = match param_city {
                    Some(param_city) => {
                        let param_city =
                            <String as std::str::FromStr>::from_str
                                (&param_city);
                        match param_city {
                            Ok(param_city) => Some(param_city),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter city - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter city")),
                        }
                    },
                    None => None,
                };
                let param_city = match param_city {
                    Some(param_city) => param_city,
                    None => return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Missing required query parameter city"))
                        .expect("Unable to create Bad Request response for missing query parameter city")),
                };

                                let result = api_impl.addressbook_address_key_get(
                                            param_city,
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyGetResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_GET_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyHead - HEAD /addressbook/{address_key}
            hyper::Method::HEAD if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_head(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyHeadResponse::DefaultHEADResponse
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyPatch - PATCH /addressbook/{address_key}
            hyper::Method::PATCH if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_patch(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyPatchResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_PATCH_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyPersonUuidDelete - DELETE /addressbook/{address_key}/person_uuid
            hyper::Method::DELETE if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_person_uuid_delete(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyPersonUuidDeleteResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID_DELETE_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyPersonUuidGet - GET /addressbook/{address_key}/person_uuid
            hyper::Method::GET if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_city = query_params.iter().filter(|e| e.0 == "city").map(|e| e.1.to_owned())
                    .next();
                let param_city = match param_city {
                    Some(param_city) => {
                        let param_city =
                            <String as std::str::FromStr>::from_str
                                (&param_city);
                        match param_city {
                            Ok(param_city) => Some(param_city),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter city - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter city")),
                        }
                    },
                    None => None,
                };
                let param_city = match param_city {
                    Some(param_city) => param_city,
                    None => return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Missing required query parameter city"))
                        .expect("Unable to create Bad Request response for missing query parameter city")),
                };

                                let result = api_impl.addressbook_address_key_person_uuid_get(
                                            param_city,
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyPersonUuidGetResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID_GET_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyPersonUuidHead - HEAD /addressbook/{address_key}/person_uuid
            hyper::Method::HEAD if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_person_uuid_head(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyPersonUuidHeadResponse::DefaultHEADResponse
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyPersonUuidPut - PUT /addressbook/{address_key}/person_uuid
            hyper::Method::PUT if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_person_uuid_put(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyPersonUuidPutResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID_PUT_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyPut - PUT /addressbook/{address_key}
            hyper::Method::PUT if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_put(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyPutResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_PUT_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyStateDelete - DELETE /addressbook/{address_key}/state
            hyper::Method::DELETE if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_STATE) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_STATE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_STATE in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_STATE.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_state_delete(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyStateDeleteResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_STATE_DELETE_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyStateGet - GET /addressbook/{address_key}/state
            hyper::Method::GET if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_STATE) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_STATE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_STATE in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_STATE.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_city = query_params.iter().filter(|e| e.0 == "city").map(|e| e.1.to_owned())
                    .next();
                let param_city = match param_city {
                    Some(param_city) => {
                        let param_city =
                            <String as std::str::FromStr>::from_str
                                (&param_city);
                        match param_city {
                            Ok(param_city) => Some(param_city),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter city - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter city")),
                        }
                    },
                    None => None,
                };
                let param_city = match param_city {
                    Some(param_city) => param_city,
                    None => return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Missing required query parameter city"))
                        .expect("Unable to create Bad Request response for missing query parameter city")),
                };

                                let result = api_impl.addressbook_address_key_state_get(
                                            param_city,
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyStateGetResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_STATE_GET_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyStateHead - HEAD /addressbook/{address_key}/state
            hyper::Method::HEAD if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_STATE) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_STATE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_STATE in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_STATE.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_state_head(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyStateHeadResponse::DefaultHEADResponse
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyStatePut - PUT /addressbook/{address_key}/state
            hyper::Method::PUT if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_STATE) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_STATE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_STATE in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_STATE.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_state_put(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyStatePutResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_STATE_PUT_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyStreetDelete - DELETE /addressbook/{address_key}/street
            hyper::Method::DELETE if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_STREET) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_STREET
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_STREET in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_STREET.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_street_delete(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyStreetDeleteResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_STREET_DELETE_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyStreetGet - GET /addressbook/{address_key}/street
            hyper::Method::GET if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_STREET) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_STREET
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_STREET in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_STREET.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_city = query_params.iter().filter(|e| e.0 == "city").map(|e| e.1.to_owned())
                    .next();
                let param_city = match param_city {
                    Some(param_city) => {
                        let param_city =
                            <String as std::str::FromStr>::from_str
                                (&param_city);
                        match param_city {
                            Ok(param_city) => Some(param_city),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter city - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter city")),
                        }
                    },
                    None => None,
                };
                let param_city = match param_city {
                    Some(param_city) => param_city,
                    None => return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Missing required query parameter city"))
                        .expect("Unable to create Bad Request response for missing query parameter city")),
                };

                                let result = api_impl.addressbook_address_key_street_get(
                                            param_city,
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyStreetGetResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_STREET_GET_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyStreetHead - HEAD /addressbook/{address_key}/street
            hyper::Method::HEAD if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_STREET) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_STREET
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_STREET in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_STREET.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_street_head(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyStreetHeadResponse::DefaultHEADResponse
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookAddressKeyStreetPut - PUT /addressbook/{address_key}/street
            hyper::Method::PUT if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_STREET) => {
                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_STREET
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE ADDRESSBOOK_ADDRESS_KEY_STREET in set but failed match against \"{}\"", path, paths::REGEX_ADDRESSBOOK_ADDRESS_KEY_STREET.as_str())
                    );

                let param_address_key = match percent_encoding::percent_decode(path_params["address_key"].as_bytes()).decode_utf8() {
                    Ok(param_address_key) => match param_address_key.parse::<String>() {
                        Ok(param_address_key) => param_address_key,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter address_key: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["address_key"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.addressbook_address_key_street_put(
                                            param_address_key,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookAddressKeyStreetPutResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_ADDRESS_KEY_STREET_PUT_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookDelete - DELETE /addressbook
            hyper::Method::DELETE if path.matched(paths::ID_ADDRESSBOOK) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .next();
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .next();
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.addressbook_delete(
                                            param_limit,
                                            param_offset,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookDeleteResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_DELETE_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookGet - GET /addressbook
            hyper::Method::GET if path.matched(paths::ID_ADDRESSBOOK) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_city = query_params.iter().filter(|e| e.0 == "city").map(|e| e.1.to_owned())
                    .next();
                let param_city = match param_city {
                    Some(param_city) => {
                        let param_city =
                            <String as std::str::FromStr>::from_str
                                (&param_city);
                        match param_city {
                            Ok(param_city) => Some(param_city),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter city - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter city")),
                        }
                    },
                    None => None,
                };
                let param_city = match param_city {
                    Some(param_city) => param_city,
                    None => return Ok(Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(Body::from("Missing required query parameter city"))
                        .expect("Unable to create Bad Request response for missing query parameter city")),
                };
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .next();
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .next();
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.addressbook_get(
                                            param_city,
                                            param_limit,
                                            param_offset,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookGetResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_GET_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookHead - HEAD /addressbook
            hyper::Method::HEAD if path.matched(paths::ID_ADDRESSBOOK) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .next();
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .next();
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.addressbook_head(
                                            param_limit,
                                            param_offset,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookHeadResponse::DefaultHEADResponse
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookPatch - PATCH /addressbook
            hyper::Method::PATCH if path.matched(paths::ID_ADDRESSBOOK) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .next();
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .next();
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.addressbook_patch(
                                            param_limit,
                                            param_offset,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookPatchResponse::OK
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_PATCH_OK"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // AddressbookPost - POST /addressbook
            hyper::Method::POST if path.matched(paths::ID_ADDRESSBOOK) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .next();
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };
                let param_offset = query_params.iter().filter(|e| e.0 == "offset").map(|e| e.1.to_owned())
                    .next();
                let param_offset = match param_offset {
                    Some(param_offset) => {
                        let param_offset =
                            <i32 as std::str::FromStr>::from_str
                                (&param_offset);
                        match param_offset {
                            Ok(param_offset) => Some(param_offset),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter offset - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter offset")),
                        }
                    },
                    None => None,
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_addressbook_post_request: Option<models::AddressbookPostRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_addressbook_post_request) => param_addressbook_post_request,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter AddressbookPostRequest - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter AddressbookPostRequest due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_addressbook_post_request = match param_addressbook_post_request {
                                    Some(param_addressbook_post_request) => param_addressbook_post_request,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter AddressbookPostRequest"))
                                                        .expect("Unable to create Bad Request response for missing body parameter AddressbookPostRequest")),
                                };

                                let result = api_impl.addressbook_post(
                                            param_addressbook_post_request,
                                            param_limit,
                                            param_offset,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                AddressbookPostResponse::CREATED
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for ADDRESSBOOK_POST_CREATED"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter AddressbookPostRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter AddressbookPostRequest")),
                        }
            },

            _ if path.matched(paths::ID_ADDRESSBOOK) => method_not_allowed(),
            _ if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY) => method_not_allowed(),
            _ if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE) => method_not_allowed(),
            _ if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_CITY) => method_not_allowed(),
            _ if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_COUNTRY) => method_not_allowed(),
            _ if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID) => method_not_allowed(),
            _ if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_STATE) => method_not_allowed(),
            _ if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_STREET) => method_not_allowed(),
            _ => Ok(Response::builder().status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response"))
        }
    } Box::pin(run(self.api_impl.clone(), req)) }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Option<&'static str> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match *request.method() {
            // AddressbookAddressKeyAddrtypeDelete - DELETE /addressbook/{address_key}/addrtype
            hyper::Method::DELETE if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE) => Some("AddressbookAddressKeyAddrtypeDelete"),
            // AddressbookAddressKeyAddrtypeGet - GET /addressbook/{address_key}/addrtype
            hyper::Method::GET if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE) => Some("AddressbookAddressKeyAddrtypeGet"),
            // AddressbookAddressKeyAddrtypeHead - HEAD /addressbook/{address_key}/addrtype
            hyper::Method::HEAD if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE) => Some("AddressbookAddressKeyAddrtypeHead"),
            // AddressbookAddressKeyAddrtypePut - PUT /addressbook/{address_key}/addrtype
            hyper::Method::PUT if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_ADDRTYPE) => Some("AddressbookAddressKeyAddrtypePut"),
            // AddressbookAddressKeyCityDelete - DELETE /addressbook/{address_key}/city
            hyper::Method::DELETE if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_CITY) => Some("AddressbookAddressKeyCityDelete"),
            // AddressbookAddressKeyCityGet - GET /addressbook/{address_key}/city
            hyper::Method::GET if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_CITY) => Some("AddressbookAddressKeyCityGet"),
            // AddressbookAddressKeyCityHead - HEAD /addressbook/{address_key}/city
            hyper::Method::HEAD if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_CITY) => Some("AddressbookAddressKeyCityHead"),
            // AddressbookAddressKeyCityPut - PUT /addressbook/{address_key}/city
            hyper::Method::PUT if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_CITY) => Some("AddressbookAddressKeyCityPut"),
            // AddressbookAddressKeyCountryDelete - DELETE /addressbook/{address_key}/country
            hyper::Method::DELETE if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_COUNTRY) => Some("AddressbookAddressKeyCountryDelete"),
            // AddressbookAddressKeyCountryGet - GET /addressbook/{address_key}/country
            hyper::Method::GET if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_COUNTRY) => Some("AddressbookAddressKeyCountryGet"),
            // AddressbookAddressKeyCountryHead - HEAD /addressbook/{address_key}/country
            hyper::Method::HEAD if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_COUNTRY) => Some("AddressbookAddressKeyCountryHead"),
            // AddressbookAddressKeyCountryPut - PUT /addressbook/{address_key}/country
            hyper::Method::PUT if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_COUNTRY) => Some("AddressbookAddressKeyCountryPut"),
            // AddressbookAddressKeyDelete - DELETE /addressbook/{address_key}
            hyper::Method::DELETE if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY) => Some("AddressbookAddressKeyDelete"),
            // AddressbookAddressKeyGet - GET /addressbook/{address_key}
            hyper::Method::GET if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY) => Some("AddressbookAddressKeyGet"),
            // AddressbookAddressKeyHead - HEAD /addressbook/{address_key}
            hyper::Method::HEAD if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY) => Some("AddressbookAddressKeyHead"),
            // AddressbookAddressKeyPatch - PATCH /addressbook/{address_key}
            hyper::Method::PATCH if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY) => Some("AddressbookAddressKeyPatch"),
            // AddressbookAddressKeyPersonUuidDelete - DELETE /addressbook/{address_key}/person_uuid
            hyper::Method::DELETE if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID) => Some("AddressbookAddressKeyPersonUuidDelete"),
            // AddressbookAddressKeyPersonUuidGet - GET /addressbook/{address_key}/person_uuid
            hyper::Method::GET if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID) => Some("AddressbookAddressKeyPersonUuidGet"),
            // AddressbookAddressKeyPersonUuidHead - HEAD /addressbook/{address_key}/person_uuid
            hyper::Method::HEAD if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID) => Some("AddressbookAddressKeyPersonUuidHead"),
            // AddressbookAddressKeyPersonUuidPut - PUT /addressbook/{address_key}/person_uuid
            hyper::Method::PUT if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_PERSON_UUID) => Some("AddressbookAddressKeyPersonUuidPut"),
            // AddressbookAddressKeyPut - PUT /addressbook/{address_key}
            hyper::Method::PUT if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY) => Some("AddressbookAddressKeyPut"),
            // AddressbookAddressKeyStateDelete - DELETE /addressbook/{address_key}/state
            hyper::Method::DELETE if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_STATE) => Some("AddressbookAddressKeyStateDelete"),
            // AddressbookAddressKeyStateGet - GET /addressbook/{address_key}/state
            hyper::Method::GET if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_STATE) => Some("AddressbookAddressKeyStateGet"),
            // AddressbookAddressKeyStateHead - HEAD /addressbook/{address_key}/state
            hyper::Method::HEAD if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_STATE) => Some("AddressbookAddressKeyStateHead"),
            // AddressbookAddressKeyStatePut - PUT /addressbook/{address_key}/state
            hyper::Method::PUT if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_STATE) => Some("AddressbookAddressKeyStatePut"),
            // AddressbookAddressKeyStreetDelete - DELETE /addressbook/{address_key}/street
            hyper::Method::DELETE if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_STREET) => Some("AddressbookAddressKeyStreetDelete"),
            // AddressbookAddressKeyStreetGet - GET /addressbook/{address_key}/street
            hyper::Method::GET if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_STREET) => Some("AddressbookAddressKeyStreetGet"),
            // AddressbookAddressKeyStreetHead - HEAD /addressbook/{address_key}/street
            hyper::Method::HEAD if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_STREET) => Some("AddressbookAddressKeyStreetHead"),
            // AddressbookAddressKeyStreetPut - PUT /addressbook/{address_key}/street
            hyper::Method::PUT if path.matched(paths::ID_ADDRESSBOOK_ADDRESS_KEY_STREET) => Some("AddressbookAddressKeyStreetPut"),
            // AddressbookDelete - DELETE /addressbook
            hyper::Method::DELETE if path.matched(paths::ID_ADDRESSBOOK) => Some("AddressbookDelete"),
            // AddressbookGet - GET /addressbook
            hyper::Method::GET if path.matched(paths::ID_ADDRESSBOOK) => Some("AddressbookGet"),
            // AddressbookHead - HEAD /addressbook
            hyper::Method::HEAD if path.matched(paths::ID_ADDRESSBOOK) => Some("AddressbookHead"),
            // AddressbookPatch - PATCH /addressbook
            hyper::Method::PATCH if path.matched(paths::ID_ADDRESSBOOK) => Some("AddressbookPatch"),
            // AddressbookPost - POST /addressbook
            hyper::Method::POST if path.matched(paths::ID_ADDRESSBOOK) => Some("AddressbookPost"),
            _ => None,
        }
    }
}
