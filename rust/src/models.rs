#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Addressbook {
    /// The address type, e.g. work or home
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "addrtype")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub addrtype: Option<String>,

    /// The city of this address
    #[serde(rename = "city")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub city: Option<String>,

    /// The country of this address
    #[serde(rename = "country")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub country: Option<String>,

    /// reference to the person's UUID, for this address
    #[serde(rename = "person_uuid")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub person_uuid: Option<String>,

    /// The state of this address
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    /// The street and civic number of this address
    #[serde(rename = "street")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub street: Option<String>,

}

impl Addressbook {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Addressbook {
        Addressbook {
            addrtype: None,
            city: None,
            country: None,
            person_uuid: None,
            state: None,
            street: None,
        }
    }
}

/// Converts the Addressbook value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Addressbook {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.addrtype.as_ref().map(|addrtype| {
                vec![
                    "addrtype".to_string(),
                    addrtype.to_string(),
                ].join(",")
            }),


            self.city.as_ref().map(|city| {
                vec![
                    "city".to_string(),
                    city.to_string(),
                ].join(",")
            }),


            self.country.as_ref().map(|country| {
                vec![
                    "country".to_string(),
                    country.to_string(),
                ].join(",")
            }),


            self.person_uuid.as_ref().map(|person_uuid| {
                vec![
                    "person_uuid".to_string(),
                    person_uuid.to_string(),
                ].join(",")
            }),


            self.state.as_ref().map(|state| {
                vec![
                    "state".to_string(),
                    state.to_string(),
                ].join(",")
            }),


            self.street.as_ref().map(|street| {
                vec![
                    "street".to_string(),
                    street.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Addressbook value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Addressbook {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub addrtype: Vec<String>,
            pub city: Vec<String>,
            pub country: Vec<String>,
            pub person_uuid: Vec<String>,
            pub state: Vec<String>,
            pub street: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Addressbook".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "addrtype" => intermediate_rep.addrtype.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "city" => intermediate_rep.city.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "country" => intermediate_rep.country.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "person_uuid" => intermediate_rep.person_uuid.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "state" => intermediate_rep.state.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "street" => intermediate_rep.street.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Addressbook".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Addressbook {
            addrtype: intermediate_rep.addrtype.into_iter().next(),
            city: intermediate_rep.city.into_iter().next(),
            country: intermediate_rep.country.into_iter().next(),
            person_uuid: intermediate_rep.person_uuid.into_iter().next(),
            state: intermediate_rep.state.into_iter().next(),
            street: intermediate_rep.street.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Addressbook> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Addressbook>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Addressbook>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Addressbook - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Addressbook> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Addressbook as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Addressbook - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AddressbookPostRequest {
    /// The address type, e.g. work or home
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "addrtype")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub addrtype: Option<String>,

    /// The city of this address
    #[serde(rename = "city")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub city: Option<String>,

    /// The country of this address
    #[serde(rename = "country")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub country: Option<String>,

    /// reference to the person's UUID, for this address
    #[serde(rename = "person_uuid")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub person_uuid: Option<String>,

    /// The state of this address
    #[serde(rename = "state")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<String>,

    /// The street and civic number of this address
    #[serde(rename = "street")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub street: Option<String>,

}

impl AddressbookPostRequest {
    #[allow(clippy::new_without_default)]
    pub fn new() -> AddressbookPostRequest {
        AddressbookPostRequest {
            addrtype: None,
            city: None,
            country: None,
            person_uuid: None,
            state: None,
            street: None,
        }
    }
}

/// Converts the AddressbookPostRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AddressbookPostRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.addrtype.as_ref().map(|addrtype| {
                vec![
                    "addrtype".to_string(),
                    addrtype.to_string(),
                ].join(",")
            }),


            self.city.as_ref().map(|city| {
                vec![
                    "city".to_string(),
                    city.to_string(),
                ].join(",")
            }),


            self.country.as_ref().map(|country| {
                vec![
                    "country".to_string(),
                    country.to_string(),
                ].join(",")
            }),


            self.person_uuid.as_ref().map(|person_uuid| {
                vec![
                    "person_uuid".to_string(),
                    person_uuid.to_string(),
                ].join(",")
            }),


            self.state.as_ref().map(|state| {
                vec![
                    "state".to_string(),
                    state.to_string(),
                ].join(",")
            }),


            self.street.as_ref().map(|street| {
                vec![
                    "street".to_string(),
                    street.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AddressbookPostRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AddressbookPostRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub addrtype: Vec<String>,
            pub city: Vec<String>,
            pub country: Vec<String>,
            pub person_uuid: Vec<String>,
            pub state: Vec<String>,
            pub street: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AddressbookPostRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "addrtype" => intermediate_rep.addrtype.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "city" => intermediate_rep.city.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "country" => intermediate_rep.country.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "person_uuid" => intermediate_rep.person_uuid.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "state" => intermediate_rep.state.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "street" => intermediate_rep.street.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AddressbookPostRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AddressbookPostRequest {
            addrtype: intermediate_rep.addrtype.into_iter().next(),
            city: intermediate_rep.city.into_iter().next(),
            country: intermediate_rep.country.into_iter().next(),
            person_uuid: intermediate_rep.person_uuid.into_iter().next(),
            state: intermediate_rep.state.into_iter().next(),
            street: intermediate_rep.street.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AddressbookPostRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AddressbookPostRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AddressbookPostRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AddressbookPostRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AddressbookPostRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AddressbookPostRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AddressbookPostRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

