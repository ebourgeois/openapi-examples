//! Main library entry point for addressbook implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{Ssl, SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use addressbook::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    #[allow(unused_mut)]
    let mut service =
        addressbook::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set certificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let tcp_listener = TcpListener::bind(&addr).await.unwrap();

            loop {
                if let Ok((tcp, _)) = tcp_listener.accept().await {
                    let ssl = Ssl::new(tls_acceptor.context()).unwrap();
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::SslStream::new(ssl, tcp).map_err(|_| ())?;
                        let service = service.await.map_err(|_| ())?;

                        Http::new()
                            .serve_connection(tls, service)
                            .await
                            .map_err(|_| ())
                    });
                }
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use addressbook::{
    Api,
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
    AddressbookPostResponse,
};
use addressbook::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    async fn addressbook_address_key_addrtype_delete(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyAddrtypeDeleteResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_addrtype_delete(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_addrtype_get(
        &self,
        city: String,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyAddrtypeGetResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_addrtype_get(\"{}\", \"{}\") - X-Span-ID: {:?}", city, address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_addrtype_head(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyAddrtypeHeadResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_addrtype_head(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_addrtype_put(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyAddrtypePutResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_addrtype_put(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_city_delete(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyCityDeleteResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_city_delete(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_city_get(
        &self,
        city: String,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyCityGetResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_city_get(\"{}\", \"{}\") - X-Span-ID: {:?}", city, address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_city_head(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyCityHeadResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_city_head(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_city_put(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyCityPutResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_city_put(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_country_delete(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyCountryDeleteResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_country_delete(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_country_get(
        &self,
        city: String,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyCountryGetResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_country_get(\"{}\", \"{}\") - X-Span-ID: {:?}", city, address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_country_head(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyCountryHeadResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_country_head(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_country_put(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyCountryPutResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_country_put(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_delete(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyDeleteResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_delete(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_get(
        &self,
        city: String,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyGetResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_get(\"{}\", \"{}\") - X-Span-ID: {:?}", city, address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_head(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyHeadResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_head(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_patch(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyPatchResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_patch(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_person_uuid_delete(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyPersonUuidDeleteResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_person_uuid_delete(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_person_uuid_get(
        &self,
        city: String,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyPersonUuidGetResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_person_uuid_get(\"{}\", \"{}\") - X-Span-ID: {:?}", city, address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_person_uuid_head(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyPersonUuidHeadResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_person_uuid_head(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_person_uuid_put(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyPersonUuidPutResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_person_uuid_put(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_put(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyPutResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_put(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_state_delete(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyStateDeleteResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_state_delete(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_state_get(
        &self,
        city: String,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyStateGetResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_state_get(\"{}\", \"{}\") - X-Span-ID: {:?}", city, address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_state_head(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyStateHeadResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_state_head(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_state_put(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyStatePutResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_state_put(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_street_delete(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyStreetDeleteResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_street_delete(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_street_get(
        &self,
        city: String,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyStreetGetResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_street_get(\"{}\", \"{}\") - X-Span-ID: {:?}", city, address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_street_head(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyStreetHeadResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_street_head(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_address_key_street_put(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyStreetPutResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_address_key_street_put(\"{}\") - X-Span-ID: {:?}", address_key, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_delete(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
        context: &C) -> Result<AddressbookDeleteResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_delete({:?}, {:?}) - X-Span-ID: {:?}", limit, offset, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_get(
        &self,
        city: String,
        limit: Option<i32>,
        offset: Option<i32>,
        context: &C) -> Result<AddressbookGetResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_get(\"{}\", {:?}, {:?}) - X-Span-ID: {:?}", city, limit, offset, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_head(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
        context: &C) -> Result<AddressbookHeadResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_head({:?}, {:?}) - X-Span-ID: {:?}", limit, offset, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_patch(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
        context: &C) -> Result<AddressbookPatchResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_patch({:?}, {:?}) - X-Span-ID: {:?}", limit, offset, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    async fn addressbook_post(
        &self,
        addressbook_post_request: models::AddressbookPostRequest,
        limit: Option<i32>,
        offset: Option<i32>,
        context: &C) -> Result<AddressbookPostResponse, ApiError>
    {
        let context = context.clone();
        info!("addressbook_post({:?}, {:?}, {:?}) - X-Span-ID: {:?}", addressbook_post_request, limit, offset, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

}
