# addressbook_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**addressbook_address_key_addrtype_delete**](addressbook_api.md#addressbook_address_key_addrtype_delete) | **DELETE** /addressbook/{address_key}/addrtype | 
**addressbook_address_key_addrtype_get**](addressbook_api.md#addressbook_address_key_addrtype_get) | **GET** /addressbook/{address_key}/addrtype | 
**addressbook_address_key_addrtype_head**](addressbook_api.md#addressbook_address_key_addrtype_head) | **HEAD** /addressbook/{address_key}/addrtype | 
**addressbook_address_key_addrtype_put**](addressbook_api.md#addressbook_address_key_addrtype_put) | **PUT** /addressbook/{address_key}/addrtype | 
**addressbook_address_key_city_delete**](addressbook_api.md#addressbook_address_key_city_delete) | **DELETE** /addressbook/{address_key}/city | 
**addressbook_address_key_city_get**](addressbook_api.md#addressbook_address_key_city_get) | **GET** /addressbook/{address_key}/city | 
**addressbook_address_key_city_head**](addressbook_api.md#addressbook_address_key_city_head) | **HEAD** /addressbook/{address_key}/city | 
**addressbook_address_key_city_put**](addressbook_api.md#addressbook_address_key_city_put) | **PUT** /addressbook/{address_key}/city | 
**addressbook_address_key_country_delete**](addressbook_api.md#addressbook_address_key_country_delete) | **DELETE** /addressbook/{address_key}/country | 
**addressbook_address_key_country_get**](addressbook_api.md#addressbook_address_key_country_get) | **GET** /addressbook/{address_key}/country | 
**addressbook_address_key_country_head**](addressbook_api.md#addressbook_address_key_country_head) | **HEAD** /addressbook/{address_key}/country | 
**addressbook_address_key_country_put**](addressbook_api.md#addressbook_address_key_country_put) | **PUT** /addressbook/{address_key}/country | 
**addressbook_address_key_delete**](addressbook_api.md#addressbook_address_key_delete) | **DELETE** /addressbook/{address_key} | 
**addressbook_address_key_get**](addressbook_api.md#addressbook_address_key_get) | **GET** /addressbook/{address_key} | 
**addressbook_address_key_head**](addressbook_api.md#addressbook_address_key_head) | **HEAD** /addressbook/{address_key} | 
**addressbook_address_key_patch**](addressbook_api.md#addressbook_address_key_patch) | **PATCH** /addressbook/{address_key} | 
**addressbook_address_key_person_uuid_delete**](addressbook_api.md#addressbook_address_key_person_uuid_delete) | **DELETE** /addressbook/{address_key}/person_uuid | 
**addressbook_address_key_person_uuid_get**](addressbook_api.md#addressbook_address_key_person_uuid_get) | **GET** /addressbook/{address_key}/person_uuid | 
**addressbook_address_key_person_uuid_head**](addressbook_api.md#addressbook_address_key_person_uuid_head) | **HEAD** /addressbook/{address_key}/person_uuid | 
**addressbook_address_key_person_uuid_put**](addressbook_api.md#addressbook_address_key_person_uuid_put) | **PUT** /addressbook/{address_key}/person_uuid | 
**addressbook_address_key_put**](addressbook_api.md#addressbook_address_key_put) | **PUT** /addressbook/{address_key} | 
**addressbook_address_key_state_delete**](addressbook_api.md#addressbook_address_key_state_delete) | **DELETE** /addressbook/{address_key}/state | 
**addressbook_address_key_state_get**](addressbook_api.md#addressbook_address_key_state_get) | **GET** /addressbook/{address_key}/state | 
**addressbook_address_key_state_head**](addressbook_api.md#addressbook_address_key_state_head) | **HEAD** /addressbook/{address_key}/state | 
**addressbook_address_key_state_put**](addressbook_api.md#addressbook_address_key_state_put) | **PUT** /addressbook/{address_key}/state | 
**addressbook_address_key_street_delete**](addressbook_api.md#addressbook_address_key_street_delete) | **DELETE** /addressbook/{address_key}/street | 
**addressbook_address_key_street_get**](addressbook_api.md#addressbook_address_key_street_get) | **GET** /addressbook/{address_key}/street | 
**addressbook_address_key_street_head**](addressbook_api.md#addressbook_address_key_street_head) | **HEAD** /addressbook/{address_key}/street | 
**addressbook_address_key_street_put**](addressbook_api.md#addressbook_address_key_street_put) | **PUT** /addressbook/{address_key}/street | 
**addressbook_delete**](addressbook_api.md#addressbook_delete) | **DELETE** /addressbook | 
**addressbook_get**](addressbook_api.md#addressbook_get) | **GET** /addressbook | 
**addressbook_head**](addressbook_api.md#addressbook_head) | **HEAD** /addressbook | 
**addressbook_patch**](addressbook_api.md#addressbook_patch) | **PATCH** /addressbook | 
**addressbook_post**](addressbook_api.md#addressbook_post) | **POST** /addressbook | 


# **addressbook_address_key_addrtype_delete**
> String addressbook_address_key_addrtype_delete(address_key)


delete operation for /addressbook/{address_key}/addrtype

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_addrtype_get**
> String addressbook_address_key_addrtype_get(city, address_key)


get operation for /addressbook/{address_key}/addrtype

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **city** | **String**|  | 
  **address_key** | **String**|  | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_addrtype_head**
> addressbook_address_key_addrtype_head(address_key)


head operation for /addressbook/{address_key}/addrtype

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_addrtype_put**
> String addressbook_address_key_addrtype_put(address_key)


put operation for /addressbook/{address_key}/addrtype

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_city_delete**
> String addressbook_address_key_city_delete(address_key)


delete operation for /addressbook/{address_key}/city

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_city_get**
> String addressbook_address_key_city_get(city, address_key)


get operation for /addressbook/{address_key}/city

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **city** | **String**|  | 
  **address_key** | **String**|  | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_city_head**
> addressbook_address_key_city_head(address_key)


head operation for /addressbook/{address_key}/city

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_city_put**
> String addressbook_address_key_city_put(address_key)


put operation for /addressbook/{address_key}/city

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_country_delete**
> String addressbook_address_key_country_delete(address_key)


delete operation for /addressbook/{address_key}/country

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_country_get**
> String addressbook_address_key_country_get(city, address_key)


get operation for /addressbook/{address_key}/country

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **city** | **String**|  | 
  **address_key** | **String**|  | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_country_head**
> addressbook_address_key_country_head(address_key)


head operation for /addressbook/{address_key}/country

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_country_put**
> String addressbook_address_key_country_put(address_key)


put operation for /addressbook/{address_key}/country

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_delete**
> models::Addressbook addressbook_address_key_delete(address_key)


delete operation for /addressbook/{address_key}

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

[**models::Addressbook**](addressbook.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_get**
> models::Addressbook addressbook_address_key_get(city, address_key)


get operation for /addressbook/{address_key}

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **city** | **String**|  | 
  **address_key** | **String**|  | 

### Return type

[**models::Addressbook**](addressbook.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_head**
> addressbook_address_key_head(address_key)


head operation for /addressbook/{address_key}

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_patch**
> models::Addressbook addressbook_address_key_patch(address_key)


patch operation for /addressbook/{address_key}

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

[**models::Addressbook**](addressbook.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_person_uuid_delete**
> String addressbook_address_key_person_uuid_delete(address_key)


delete operation for /addressbook/{address_key}/person_uuid

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_person_uuid_get**
> String addressbook_address_key_person_uuid_get(city, address_key)


get operation for /addressbook/{address_key}/person_uuid

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **city** | **String**|  | 
  **address_key** | **String**|  | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_person_uuid_head**
> addressbook_address_key_person_uuid_head(address_key)


head operation for /addressbook/{address_key}/person_uuid

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_person_uuid_put**
> String addressbook_address_key_person_uuid_put(address_key)


put operation for /addressbook/{address_key}/person_uuid

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_put**
> models::Addressbook addressbook_address_key_put(address_key)


put operation for /addressbook/{address_key}

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

[**models::Addressbook**](addressbook.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_state_delete**
> String addressbook_address_key_state_delete(address_key)


delete operation for /addressbook/{address_key}/state

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_state_get**
> String addressbook_address_key_state_get(city, address_key)


get operation for /addressbook/{address_key}/state

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **city** | **String**|  | 
  **address_key** | **String**|  | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_state_head**
> addressbook_address_key_state_head(address_key)


head operation for /addressbook/{address_key}/state

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_state_put**
> String addressbook_address_key_state_put(address_key)


put operation for /addressbook/{address_key}/state

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_street_delete**
> String addressbook_address_key_street_delete(address_key)


delete operation for /addressbook/{address_key}/street

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_street_get**
> String addressbook_address_key_street_get(city, address_key)


get operation for /addressbook/{address_key}/street

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **city** | **String**|  | 
  **address_key** | **String**|  | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_street_head**
> addressbook_address_key_street_head(address_key)


head operation for /addressbook/{address_key}/street

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_address_key_street_put**
> String addressbook_address_key_street_put(address_key)


put operation for /addressbook/{address_key}/street

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **address_key** | **String**|  | 

### Return type

[**String**](string.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_delete**
> models::Addressbook addressbook_delete(optional)


delete operation for /addressbook

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **i32**| Limit the number of responses back | 
 **offset** | **i32**| The offset to start returning resources | 

### Return type

[**models::Addressbook**](addressbook.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_get**
> Vec<models::Addressbook> addressbook_get(city, optional)


List all addresses in this addressbook

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **city** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **city** | **String**|  | 
 **limit** | **i32**| Limit the number of responses back | 
 **offset** | **i32**| The offset to start returning resources | 

### Return type

[**Vec<models::Addressbook>**](addressbook.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_head**
> addressbook_head(optional)


Determine the existence and size of addresses in this addressbook

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **i32**| Limit the number of responses back | 
 **offset** | **i32**| The offset to start returning resources | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_patch**
> models::Addressbook addressbook_patch(optional)


Patch one or more addresses in this addressbook

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **i32**| Limit the number of responses back | 
 **offset** | **i32**| The offset to start returning resources | 

### Return type

[**models::Addressbook**](addressbook.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addressbook_post**
> models::Addressbook addressbook_post(addressbook_post_request, optional)


Create a new address in this addressbook, a new address key will be created

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **addressbook_post_request** | [**AddressbookPostRequest**](AddressbookPostRequest.md)| The request body for /addressbook | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **addressbook_post_request** | [**AddressbookPostRequest**](AddressbookPostRequest.md)| The request body for /addressbook | 
 **limit** | **i32**| Limit the number of responses back | 
 **offset** | **i32**| The offset to start returning resources | 

### Return type

[**models::Addressbook**](addressbook.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

