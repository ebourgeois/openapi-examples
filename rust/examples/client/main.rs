#![allow(missing_docs, unused_variables, trivial_casts)]


#[allow(unused_imports)]
use futures::{future, Stream, stream};
#[allow(unused_imports)]
use addressbook::{Api, ApiNoContext, Client, ContextWrapperExt, models,
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
use clap::{App, Arg};

#[allow(unused_imports)]
use log::info;

// swagger::Has may be unused if there are no examples
#[allow(unused_imports)]
use swagger::{AuthData, ContextBuilder, EmptyContext, Has, Push, XSpanIdString};

type ClientContext = swagger::make_context_ty!(ContextBuilder, EmptyContext, Option<AuthData>, XSpanIdString);

// rt may be unused if there are no examples
#[allow(unused_mut)]
fn main() {
    env_logger::init();

    let matches = App::new("client")
        .arg(Arg::with_name("operation")
            .help("Sets the operation to run")
            .possible_values(&[
                "AddressbookAddressKeyAddrtypeDelete",
                "AddressbookAddressKeyAddrtypeGet",
                "AddressbookAddressKeyAddrtypeHead",
                "AddressbookAddressKeyAddrtypePut",
                "AddressbookAddressKeyCityDelete",
                "AddressbookAddressKeyCityGet",
                "AddressbookAddressKeyCityHead",
                "AddressbookAddressKeyCityPut",
                "AddressbookAddressKeyCountryDelete",
                "AddressbookAddressKeyCountryGet",
                "AddressbookAddressKeyCountryHead",
                "AddressbookAddressKeyCountryPut",
                "AddressbookAddressKeyDelete",
                "AddressbookAddressKeyGet",
                "AddressbookAddressKeyHead",
                "AddressbookAddressKeyPatch",
                "AddressbookAddressKeyPersonUuidDelete",
                "AddressbookAddressKeyPersonUuidGet",
                "AddressbookAddressKeyPersonUuidHead",
                "AddressbookAddressKeyPersonUuidPut",
                "AddressbookAddressKeyPut",
                "AddressbookAddressKeyStateDelete",
                "AddressbookAddressKeyStateGet",
                "AddressbookAddressKeyStateHead",
                "AddressbookAddressKeyStatePut",
                "AddressbookAddressKeyStreetDelete",
                "AddressbookAddressKeyStreetGet",
                "AddressbookAddressKeyStreetHead",
                "AddressbookAddressKeyStreetPut",
                "AddressbookDelete",
                "AddressbookGet",
                "AddressbookHead",
                "AddressbookPatch",
            ])
            .required(true)
            .index(1))
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .arg(Arg::with_name("host")
            .long("host")
            .takes_value(true)
            .default_value("localhost")
            .help("Hostname to contact"))
        .arg(Arg::with_name("port")
            .long("port")
            .takes_value(true)
            .default_value("8080")
            .help("Port to contact"))
        .get_matches();

    let is_https = matches.is_present("https");
    let base_url = format!("{}://{}:{}",
                           if is_https { "https" } else { "http" },
                           matches.value_of("host").unwrap(),
                           matches.value_of("port").unwrap());

    let context: ClientContext =
        swagger::make_context!(ContextBuilder, EmptyContext, None as Option<AuthData>, XSpanIdString::default());

    let mut client : Box<dyn ApiNoContext<ClientContext>> = if matches.is_present("https") {
        // Using Simple HTTPS
        let client = Box::new(Client::try_new_https(&base_url)
            .expect("Failed to create HTTPS client"));
        Box::new(client.with_context(context))
    } else {
        // Using HTTP
        let client = Box::new(Client::try_new_http(
            &base_url)
            .expect("Failed to create HTTP client"));
        Box::new(client.with_context(context))
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    match matches.value_of("operation") {
        Some("AddressbookAddressKeyAddrtypeDelete") => {
            let result = rt.block_on(client.addressbook_address_key_addrtype_delete(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyAddrtypeGet") => {
            let result = rt.block_on(client.addressbook_address_key_addrtype_get(
                  "city_example".to_string(),
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyAddrtypeHead") => {
            let result = rt.block_on(client.addressbook_address_key_addrtype_head(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyAddrtypePut") => {
            let result = rt.block_on(client.addressbook_address_key_addrtype_put(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyCityDelete") => {
            let result = rt.block_on(client.addressbook_address_key_city_delete(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyCityGet") => {
            let result = rt.block_on(client.addressbook_address_key_city_get(
                  "city_example".to_string(),
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyCityHead") => {
            let result = rt.block_on(client.addressbook_address_key_city_head(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyCityPut") => {
            let result = rt.block_on(client.addressbook_address_key_city_put(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyCountryDelete") => {
            let result = rt.block_on(client.addressbook_address_key_country_delete(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyCountryGet") => {
            let result = rt.block_on(client.addressbook_address_key_country_get(
                  "city_example".to_string(),
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyCountryHead") => {
            let result = rt.block_on(client.addressbook_address_key_country_head(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyCountryPut") => {
            let result = rt.block_on(client.addressbook_address_key_country_put(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyDelete") => {
            let result = rt.block_on(client.addressbook_address_key_delete(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyGet") => {
            let result = rt.block_on(client.addressbook_address_key_get(
                  "city_example".to_string(),
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyHead") => {
            let result = rt.block_on(client.addressbook_address_key_head(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyPatch") => {
            let result = rt.block_on(client.addressbook_address_key_patch(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyPersonUuidDelete") => {
            let result = rt.block_on(client.addressbook_address_key_person_uuid_delete(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyPersonUuidGet") => {
            let result = rt.block_on(client.addressbook_address_key_person_uuid_get(
                  "city_example".to_string(),
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyPersonUuidHead") => {
            let result = rt.block_on(client.addressbook_address_key_person_uuid_head(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyPersonUuidPut") => {
            let result = rt.block_on(client.addressbook_address_key_person_uuid_put(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyPut") => {
            let result = rt.block_on(client.addressbook_address_key_put(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyStateDelete") => {
            let result = rt.block_on(client.addressbook_address_key_state_delete(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyStateGet") => {
            let result = rt.block_on(client.addressbook_address_key_state_get(
                  "city_example".to_string(),
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyStateHead") => {
            let result = rt.block_on(client.addressbook_address_key_state_head(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyStatePut") => {
            let result = rt.block_on(client.addressbook_address_key_state_put(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyStreetDelete") => {
            let result = rt.block_on(client.addressbook_address_key_street_delete(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyStreetGet") => {
            let result = rt.block_on(client.addressbook_address_key_street_get(
                  "city_example".to_string(),
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyStreetHead") => {
            let result = rt.block_on(client.addressbook_address_key_street_head(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookAddressKeyStreetPut") => {
            let result = rt.block_on(client.addressbook_address_key_street_put(
                  "address_key_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookDelete") => {
            let result = rt.block_on(client.addressbook_delete(
                  Some(56),
                  Some(56)
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookGet") => {
            let result = rt.block_on(client.addressbook_get(
                  "city_example".to_string(),
                  Some(56),
                  Some(56)
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookHead") => {
            let result = rt.block_on(client.addressbook_head(
                  Some(56),
                  Some(56)
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AddressbookPatch") => {
            let result = rt.block_on(client.addressbook_patch(
                  Some(56),
                  Some(56)
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        /* Disabled because there's no example.
        Some("AddressbookPost") => {
            let result = rt.block_on(client.addressbook_post(
                  ???,
                  Some(56),
                  Some(56)
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        */
        _ => {
            panic!("Invalid operation provided")
        }
    }
}
