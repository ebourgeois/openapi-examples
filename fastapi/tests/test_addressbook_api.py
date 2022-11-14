# coding: utf-8

from fastapi.testclient import TestClient


from addressbook.models.addressbook import Addressbook  # noqa: F401
from addressbook.models.addressbook_post_request import AddressbookPostRequest  # noqa: F401


def test_addressbook_address_key_addrtype_delete(client: TestClient):
    """Test case for addressbook_address_key_addrtype_delete

    
    """

    headers = {
    }
    response = client.request(
        "DELETE",
        "/addressbook/{address_key}/addrtype".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_addrtype_get(client: TestClient):
    """Test case for addressbook_address_key_addrtype_get

    
    """
    params = [("city", 'city_example')]
    headers = {
    }
    response = client.request(
        "GET",
        "/addressbook/{address_key}/addrtype".format(address_key='address_key_example'),
        headers=headers,
        params=params,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_addrtype_head(client: TestClient):
    """Test case for addressbook_address_key_addrtype_head

    
    """

    headers = {
    }
    response = client.request(
        "HEAD",
        "/addressbook/{address_key}/addrtype".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_addrtype_put(client: TestClient):
    """Test case for addressbook_address_key_addrtype_put

    
    """

    headers = {
    }
    response = client.request(
        "PUT",
        "/addressbook/{address_key}/addrtype".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_city_delete(client: TestClient):
    """Test case for addressbook_address_key_city_delete

    
    """

    headers = {
    }
    response = client.request(
        "DELETE",
        "/addressbook/{address_key}/city".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_city_get(client: TestClient):
    """Test case for addressbook_address_key_city_get

    
    """
    params = [("city", 'city_example')]
    headers = {
    }
    response = client.request(
        "GET",
        "/addressbook/{address_key}/city".format(address_key='address_key_example'),
        headers=headers,
        params=params,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_city_head(client: TestClient):
    """Test case for addressbook_address_key_city_head

    
    """

    headers = {
    }
    response = client.request(
        "HEAD",
        "/addressbook/{address_key}/city".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_city_put(client: TestClient):
    """Test case for addressbook_address_key_city_put

    
    """

    headers = {
    }
    response = client.request(
        "PUT",
        "/addressbook/{address_key}/city".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_country_delete(client: TestClient):
    """Test case for addressbook_address_key_country_delete

    
    """

    headers = {
    }
    response = client.request(
        "DELETE",
        "/addressbook/{address_key}/country".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_country_get(client: TestClient):
    """Test case for addressbook_address_key_country_get

    
    """
    params = [("city", 'city_example')]
    headers = {
    }
    response = client.request(
        "GET",
        "/addressbook/{address_key}/country".format(address_key='address_key_example'),
        headers=headers,
        params=params,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_country_head(client: TestClient):
    """Test case for addressbook_address_key_country_head

    
    """

    headers = {
    }
    response = client.request(
        "HEAD",
        "/addressbook/{address_key}/country".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_country_put(client: TestClient):
    """Test case for addressbook_address_key_country_put

    
    """

    headers = {
    }
    response = client.request(
        "PUT",
        "/addressbook/{address_key}/country".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_delete(client: TestClient):
    """Test case for addressbook_address_key_delete

    
    """

    headers = {
    }
    response = client.request(
        "DELETE",
        "/addressbook/{address_key}".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_get(client: TestClient):
    """Test case for addressbook_address_key_get

    
    """
    params = [("city", 'city_example')]
    headers = {
    }
    response = client.request(
        "GET",
        "/addressbook/{address_key}".format(address_key='address_key_example'),
        headers=headers,
        params=params,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_head(client: TestClient):
    """Test case for addressbook_address_key_head

    
    """

    headers = {
    }
    response = client.request(
        "HEAD",
        "/addressbook/{address_key}".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_patch(client: TestClient):
    """Test case for addressbook_address_key_patch

    
    """

    headers = {
    }
    response = client.request(
        "PATCH",
        "/addressbook/{address_key}".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_person_uuid_delete(client: TestClient):
    """Test case for addressbook_address_key_person_uuid_delete

    
    """

    headers = {
    }
    response = client.request(
        "DELETE",
        "/addressbook/{address_key}/person_uuid".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_person_uuid_get(client: TestClient):
    """Test case for addressbook_address_key_person_uuid_get

    
    """
    params = [("city", 'city_example')]
    headers = {
    }
    response = client.request(
        "GET",
        "/addressbook/{address_key}/person_uuid".format(address_key='address_key_example'),
        headers=headers,
        params=params,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_person_uuid_head(client: TestClient):
    """Test case for addressbook_address_key_person_uuid_head

    
    """

    headers = {
    }
    response = client.request(
        "HEAD",
        "/addressbook/{address_key}/person_uuid".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_person_uuid_put(client: TestClient):
    """Test case for addressbook_address_key_person_uuid_put

    
    """

    headers = {
    }
    response = client.request(
        "PUT",
        "/addressbook/{address_key}/person_uuid".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_put(client: TestClient):
    """Test case for addressbook_address_key_put

    
    """

    headers = {
    }
    response = client.request(
        "PUT",
        "/addressbook/{address_key}".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_state_delete(client: TestClient):
    """Test case for addressbook_address_key_state_delete

    
    """

    headers = {
    }
    response = client.request(
        "DELETE",
        "/addressbook/{address_key}/state".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_state_get(client: TestClient):
    """Test case for addressbook_address_key_state_get

    
    """
    params = [("city", 'city_example')]
    headers = {
    }
    response = client.request(
        "GET",
        "/addressbook/{address_key}/state".format(address_key='address_key_example'),
        headers=headers,
        params=params,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_state_head(client: TestClient):
    """Test case for addressbook_address_key_state_head

    
    """

    headers = {
    }
    response = client.request(
        "HEAD",
        "/addressbook/{address_key}/state".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_state_put(client: TestClient):
    """Test case for addressbook_address_key_state_put

    
    """

    headers = {
    }
    response = client.request(
        "PUT",
        "/addressbook/{address_key}/state".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_street_delete(client: TestClient):
    """Test case for addressbook_address_key_street_delete

    
    """

    headers = {
    }
    response = client.request(
        "DELETE",
        "/addressbook/{address_key}/street".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_street_get(client: TestClient):
    """Test case for addressbook_address_key_street_get

    
    """
    params = [("city", 'city_example')]
    headers = {
    }
    response = client.request(
        "GET",
        "/addressbook/{address_key}/street".format(address_key='address_key_example'),
        headers=headers,
        params=params,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_street_head(client: TestClient):
    """Test case for addressbook_address_key_street_head

    
    """

    headers = {
    }
    response = client.request(
        "HEAD",
        "/addressbook/{address_key}/street".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_address_key_street_put(client: TestClient):
    """Test case for addressbook_address_key_street_put

    
    """

    headers = {
    }
    response = client.request(
        "PUT",
        "/addressbook/{address_key}/street".format(address_key='address_key_example'),
        headers=headers,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_delete(client: TestClient):
    """Test case for addressbook_delete

    
    """
    params = [("limit", 56),     ("offset", 56)]
    headers = {
    }
    response = client.request(
        "DELETE",
        "/addressbook",
        headers=headers,
        params=params,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_get(client: TestClient):
    """Test case for addressbook_get

    
    """
    params = [("city", 'city_example'),     ("limit", 56),     ("offset", 56)]
    headers = {
    }
    response = client.request(
        "GET",
        "/addressbook",
        headers=headers,
        params=params,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_head(client: TestClient):
    """Test case for addressbook_head

    
    """
    params = [("limit", 56),     ("offset", 56)]
    headers = {
    }
    response = client.request(
        "HEAD",
        "/addressbook",
        headers=headers,
        params=params,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_patch(client: TestClient):
    """Test case for addressbook_patch

    
    """
    params = [("limit", 56),     ("offset", 56)]
    headers = {
    }
    response = client.request(
        "PATCH",
        "/addressbook",
        headers=headers,
        params=params,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200


def test_addressbook_post(client: TestClient):
    """Test case for addressbook_post

    
    """
    addressbook_post_request = addressbook.AddressbookPostRequest()
    params = [("limit", 56),     ("offset", 56)]
    headers = {
    }
    response = client.request(
        "POST",
        "/addressbook",
        headers=headers,
        json=addressbook_post_request,
        params=params,
    )

    # uncomment below to assert the status code of the HTTP response
    #assert response.status_code == 200

