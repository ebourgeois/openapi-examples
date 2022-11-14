#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::blacklisted_name)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "";
pub const API_VERSION: &str = "1.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyAddrtypeDeleteResponse {
    /// OK
    OK
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyAddrtypeGetResponse {
    /// OK
    OK
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyAddrtypeHeadResponse {
    /// Default HEAD response
    DefaultHEADResponse
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyAddrtypePutResponse {
    /// OK
    OK
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyCityDeleteResponse {
    /// OK
    OK
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyCityGetResponse {
    /// OK
    OK
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyCityHeadResponse {
    /// Default HEAD response
    DefaultHEADResponse
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyCityPutResponse {
    /// OK
    OK
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyCountryDeleteResponse {
    /// OK
    OK
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyCountryGetResponse {
    /// OK
    OK
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyCountryHeadResponse {
    /// Default HEAD response
    DefaultHEADResponse
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyCountryPutResponse {
    /// OK
    OK
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyDeleteResponse {
    /// OK
    OK
    (models::Addressbook)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyGetResponse {
    /// OK
    OK
    (models::Addressbook)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyHeadResponse {
    /// Default HEAD response
    DefaultHEADResponse
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyPatchResponse {
    /// OK
    OK
    (models::Addressbook)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyPersonUuidDeleteResponse {
    /// OK
    OK
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyPersonUuidGetResponse {
    /// OK
    OK
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyPersonUuidHeadResponse {
    /// Default HEAD response
    DefaultHEADResponse
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyPersonUuidPutResponse {
    /// OK
    OK
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyPutResponse {
    /// OK
    OK
    (models::Addressbook)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyStateDeleteResponse {
    /// OK
    OK
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyStateGetResponse {
    /// OK
    OK
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyStateHeadResponse {
    /// Default HEAD response
    DefaultHEADResponse
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyStatePutResponse {
    /// OK
    OK
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyStreetDeleteResponse {
    /// OK
    OK
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyStreetGetResponse {
    /// OK
    OK
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyStreetHeadResponse {
    /// Default HEAD response
    DefaultHEADResponse
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookAddressKeyStreetPutResponse {
    /// OK
    OK
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookDeleteResponse {
    /// OK
    OK
    (models::Addressbook)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookGetResponse {
    /// OK
    OK
    (Vec<models::Addressbook>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookHeadResponse {
    /// Default HEAD response
    DefaultHEADResponse
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookPatchResponse {
    /// OK
    OK
    (models::Addressbook)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AddressbookPostResponse {
    /// CREATED
    CREATED
    (models::Addressbook)
}

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    async fn addressbook_address_key_addrtype_delete(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyAddrtypeDeleteResponse, ApiError>;

    async fn addressbook_address_key_addrtype_get(
        &self,
        city: String,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyAddrtypeGetResponse, ApiError>;

    async fn addressbook_address_key_addrtype_head(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyAddrtypeHeadResponse, ApiError>;

    async fn addressbook_address_key_addrtype_put(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyAddrtypePutResponse, ApiError>;

    async fn addressbook_address_key_city_delete(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyCityDeleteResponse, ApiError>;

    async fn addressbook_address_key_city_get(
        &self,
        city: String,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyCityGetResponse, ApiError>;

    async fn addressbook_address_key_city_head(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyCityHeadResponse, ApiError>;

    async fn addressbook_address_key_city_put(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyCityPutResponse, ApiError>;

    async fn addressbook_address_key_country_delete(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyCountryDeleteResponse, ApiError>;

    async fn addressbook_address_key_country_get(
        &self,
        city: String,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyCountryGetResponse, ApiError>;

    async fn addressbook_address_key_country_head(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyCountryHeadResponse, ApiError>;

    async fn addressbook_address_key_country_put(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyCountryPutResponse, ApiError>;

    async fn addressbook_address_key_delete(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyDeleteResponse, ApiError>;

    async fn addressbook_address_key_get(
        &self,
        city: String,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyGetResponse, ApiError>;

    async fn addressbook_address_key_head(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyHeadResponse, ApiError>;

    async fn addressbook_address_key_patch(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyPatchResponse, ApiError>;

    async fn addressbook_address_key_person_uuid_delete(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyPersonUuidDeleteResponse, ApiError>;

    async fn addressbook_address_key_person_uuid_get(
        &self,
        city: String,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyPersonUuidGetResponse, ApiError>;

    async fn addressbook_address_key_person_uuid_head(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyPersonUuidHeadResponse, ApiError>;

    async fn addressbook_address_key_person_uuid_put(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyPersonUuidPutResponse, ApiError>;

    async fn addressbook_address_key_put(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyPutResponse, ApiError>;

    async fn addressbook_address_key_state_delete(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyStateDeleteResponse, ApiError>;

    async fn addressbook_address_key_state_get(
        &self,
        city: String,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyStateGetResponse, ApiError>;

    async fn addressbook_address_key_state_head(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyStateHeadResponse, ApiError>;

    async fn addressbook_address_key_state_put(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyStatePutResponse, ApiError>;

    async fn addressbook_address_key_street_delete(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyStreetDeleteResponse, ApiError>;

    async fn addressbook_address_key_street_get(
        &self,
        city: String,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyStreetGetResponse, ApiError>;

    async fn addressbook_address_key_street_head(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyStreetHeadResponse, ApiError>;

    async fn addressbook_address_key_street_put(
        &self,
        address_key: String,
        context: &C) -> Result<AddressbookAddressKeyStreetPutResponse, ApiError>;

    async fn addressbook_delete(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
        context: &C) -> Result<AddressbookDeleteResponse, ApiError>;

    async fn addressbook_get(
        &self,
        city: String,
        limit: Option<i32>,
        offset: Option<i32>,
        context: &C) -> Result<AddressbookGetResponse, ApiError>;

    async fn addressbook_head(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
        context: &C) -> Result<AddressbookHeadResponse, ApiError>;

    async fn addressbook_patch(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
        context: &C) -> Result<AddressbookPatchResponse, ApiError>;

    async fn addressbook_post(
        &self,
        addressbook_post_request: models::AddressbookPostRequest,
        limit: Option<i32>,
        offset: Option<i32>,
        context: &C) -> Result<AddressbookPostResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    async fn addressbook_address_key_addrtype_delete(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyAddrtypeDeleteResponse, ApiError>;

    async fn addressbook_address_key_addrtype_get(
        &self,
        city: String,
        address_key: String,
        ) -> Result<AddressbookAddressKeyAddrtypeGetResponse, ApiError>;

    async fn addressbook_address_key_addrtype_head(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyAddrtypeHeadResponse, ApiError>;

    async fn addressbook_address_key_addrtype_put(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyAddrtypePutResponse, ApiError>;

    async fn addressbook_address_key_city_delete(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyCityDeleteResponse, ApiError>;

    async fn addressbook_address_key_city_get(
        &self,
        city: String,
        address_key: String,
        ) -> Result<AddressbookAddressKeyCityGetResponse, ApiError>;

    async fn addressbook_address_key_city_head(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyCityHeadResponse, ApiError>;

    async fn addressbook_address_key_city_put(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyCityPutResponse, ApiError>;

    async fn addressbook_address_key_country_delete(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyCountryDeleteResponse, ApiError>;

    async fn addressbook_address_key_country_get(
        &self,
        city: String,
        address_key: String,
        ) -> Result<AddressbookAddressKeyCountryGetResponse, ApiError>;

    async fn addressbook_address_key_country_head(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyCountryHeadResponse, ApiError>;

    async fn addressbook_address_key_country_put(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyCountryPutResponse, ApiError>;

    async fn addressbook_address_key_delete(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyDeleteResponse, ApiError>;

    async fn addressbook_address_key_get(
        &self,
        city: String,
        address_key: String,
        ) -> Result<AddressbookAddressKeyGetResponse, ApiError>;

    async fn addressbook_address_key_head(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyHeadResponse, ApiError>;

    async fn addressbook_address_key_patch(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyPatchResponse, ApiError>;

    async fn addressbook_address_key_person_uuid_delete(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyPersonUuidDeleteResponse, ApiError>;

    async fn addressbook_address_key_person_uuid_get(
        &self,
        city: String,
        address_key: String,
        ) -> Result<AddressbookAddressKeyPersonUuidGetResponse, ApiError>;

    async fn addressbook_address_key_person_uuid_head(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyPersonUuidHeadResponse, ApiError>;

    async fn addressbook_address_key_person_uuid_put(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyPersonUuidPutResponse, ApiError>;

    async fn addressbook_address_key_put(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyPutResponse, ApiError>;

    async fn addressbook_address_key_state_delete(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyStateDeleteResponse, ApiError>;

    async fn addressbook_address_key_state_get(
        &self,
        city: String,
        address_key: String,
        ) -> Result<AddressbookAddressKeyStateGetResponse, ApiError>;

    async fn addressbook_address_key_state_head(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyStateHeadResponse, ApiError>;

    async fn addressbook_address_key_state_put(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyStatePutResponse, ApiError>;

    async fn addressbook_address_key_street_delete(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyStreetDeleteResponse, ApiError>;

    async fn addressbook_address_key_street_get(
        &self,
        city: String,
        address_key: String,
        ) -> Result<AddressbookAddressKeyStreetGetResponse, ApiError>;

    async fn addressbook_address_key_street_head(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyStreetHeadResponse, ApiError>;

    async fn addressbook_address_key_street_put(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyStreetPutResponse, ApiError>;

    async fn addressbook_delete(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
        ) -> Result<AddressbookDeleteResponse, ApiError>;

    async fn addressbook_get(
        &self,
        city: String,
        limit: Option<i32>,
        offset: Option<i32>,
        ) -> Result<AddressbookGetResponse, ApiError>;

    async fn addressbook_head(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
        ) -> Result<AddressbookHeadResponse, ApiError>;

    async fn addressbook_patch(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
        ) -> Result<AddressbookPatchResponse, ApiError>;

    async fn addressbook_post(
        &self,
        addressbook_post_request: models::AddressbookPostRequest,
        limit: Option<i32>,
        offset: Option<i32>,
        ) -> Result<AddressbookPostResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    async fn addressbook_address_key_addrtype_delete(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyAddrtypeDeleteResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_addrtype_delete(address_key, &context).await
    }

    async fn addressbook_address_key_addrtype_get(
        &self,
        city: String,
        address_key: String,
        ) -> Result<AddressbookAddressKeyAddrtypeGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_addrtype_get(city, address_key, &context).await
    }

    async fn addressbook_address_key_addrtype_head(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyAddrtypeHeadResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_addrtype_head(address_key, &context).await
    }

    async fn addressbook_address_key_addrtype_put(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyAddrtypePutResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_addrtype_put(address_key, &context).await
    }

    async fn addressbook_address_key_city_delete(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyCityDeleteResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_city_delete(address_key, &context).await
    }

    async fn addressbook_address_key_city_get(
        &self,
        city: String,
        address_key: String,
        ) -> Result<AddressbookAddressKeyCityGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_city_get(city, address_key, &context).await
    }

    async fn addressbook_address_key_city_head(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyCityHeadResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_city_head(address_key, &context).await
    }

    async fn addressbook_address_key_city_put(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyCityPutResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_city_put(address_key, &context).await
    }

    async fn addressbook_address_key_country_delete(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyCountryDeleteResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_country_delete(address_key, &context).await
    }

    async fn addressbook_address_key_country_get(
        &self,
        city: String,
        address_key: String,
        ) -> Result<AddressbookAddressKeyCountryGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_country_get(city, address_key, &context).await
    }

    async fn addressbook_address_key_country_head(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyCountryHeadResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_country_head(address_key, &context).await
    }

    async fn addressbook_address_key_country_put(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyCountryPutResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_country_put(address_key, &context).await
    }

    async fn addressbook_address_key_delete(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyDeleteResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_delete(address_key, &context).await
    }

    async fn addressbook_address_key_get(
        &self,
        city: String,
        address_key: String,
        ) -> Result<AddressbookAddressKeyGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_get(city, address_key, &context).await
    }

    async fn addressbook_address_key_head(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyHeadResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_head(address_key, &context).await
    }

    async fn addressbook_address_key_patch(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyPatchResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_patch(address_key, &context).await
    }

    async fn addressbook_address_key_person_uuid_delete(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyPersonUuidDeleteResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_person_uuid_delete(address_key, &context).await
    }

    async fn addressbook_address_key_person_uuid_get(
        &self,
        city: String,
        address_key: String,
        ) -> Result<AddressbookAddressKeyPersonUuidGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_person_uuid_get(city, address_key, &context).await
    }

    async fn addressbook_address_key_person_uuid_head(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyPersonUuidHeadResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_person_uuid_head(address_key, &context).await
    }

    async fn addressbook_address_key_person_uuid_put(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyPersonUuidPutResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_person_uuid_put(address_key, &context).await
    }

    async fn addressbook_address_key_put(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyPutResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_put(address_key, &context).await
    }

    async fn addressbook_address_key_state_delete(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyStateDeleteResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_state_delete(address_key, &context).await
    }

    async fn addressbook_address_key_state_get(
        &self,
        city: String,
        address_key: String,
        ) -> Result<AddressbookAddressKeyStateGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_state_get(city, address_key, &context).await
    }

    async fn addressbook_address_key_state_head(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyStateHeadResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_state_head(address_key, &context).await
    }

    async fn addressbook_address_key_state_put(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyStatePutResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_state_put(address_key, &context).await
    }

    async fn addressbook_address_key_street_delete(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyStreetDeleteResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_street_delete(address_key, &context).await
    }

    async fn addressbook_address_key_street_get(
        &self,
        city: String,
        address_key: String,
        ) -> Result<AddressbookAddressKeyStreetGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_street_get(city, address_key, &context).await
    }

    async fn addressbook_address_key_street_head(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyStreetHeadResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_street_head(address_key, &context).await
    }

    async fn addressbook_address_key_street_put(
        &self,
        address_key: String,
        ) -> Result<AddressbookAddressKeyStreetPutResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_address_key_street_put(address_key, &context).await
    }

    async fn addressbook_delete(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
        ) -> Result<AddressbookDeleteResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_delete(limit, offset, &context).await
    }

    async fn addressbook_get(
        &self,
        city: String,
        limit: Option<i32>,
        offset: Option<i32>,
        ) -> Result<AddressbookGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_get(city, limit, offset, &context).await
    }

    async fn addressbook_head(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
        ) -> Result<AddressbookHeadResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_head(limit, offset, &context).await
    }

    async fn addressbook_patch(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
        ) -> Result<AddressbookPatchResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_patch(limit, offset, &context).await
    }

    async fn addressbook_post(
        &self,
        addressbook_post_request: models::AddressbookPostRequest,
        limit: Option<i32>,
        offset: Option<i32>,
        ) -> Result<AddressbookPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().addressbook_post(addressbook_post_request, limit, offset, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
