# coding: utf-8

from typing import Dict, List  # noqa: F401

from fastapi import (  # noqa: F401
    APIRouter,
    Body,
    Cookie,
    Depends,
    Form,
    Header,
    Path,
    Query,
    Response,
    Security,
    status,
)

from addressbook.models.extra_models import TokenModel  # noqa: F401
from addressbook.models.addressbook import Addressbook
from addressbook.models.addressbook_post_request import AddressbookPostRequest


router = APIRouter()


@router.delete(
    "/addressbook/{address_key}/addrtype",
    responses={
        200: {"model": str, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_addrtype_delete(
    address_key: str = Path(None, description=""),
) -> str:
    """delete operation for /addressbook/{address_key}/addrtype"""
    ...


@router.get(
    "/addressbook/{address_key}/addrtype",
    responses={
        200: {"model": str, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_addrtype_get(
    city: str = Query(None, description=""),
    address_key: str = Path(None, description=""),
) -> str:
    """get operation for /addressbook/{address_key}/addrtype"""
    ...


@router.head(
    "/addressbook/{address_key}/addrtype",
    responses={
        200: {"description": "Default HEAD response"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_addrtype_head(
    address_key: str = Path(None, description=""),
) -> None:
    """head operation for /addressbook/{address_key}/addrtype"""
    ...


@router.put(
    "/addressbook/{address_key}/addrtype",
    responses={
        200: {"model": str, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_addrtype_put(
    address_key: str = Path(None, description=""),
) -> str:
    """put operation for /addressbook/{address_key}/addrtype"""
    ...


@router.delete(
    "/addressbook/{address_key}/city",
    responses={
        200: {"model": str, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_city_delete(
    address_key: str = Path(None, description=""),
) -> str:
    """delete operation for /addressbook/{address_key}/city"""
    ...


@router.get(
    "/addressbook/{address_key}/city",
    responses={
        200: {"model": str, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_city_get(
    city: str = Query(None, description=""),
    address_key: str = Path(None, description=""),
) -> str:
    """get operation for /addressbook/{address_key}/city"""
    ...


@router.head(
    "/addressbook/{address_key}/city",
    responses={
        200: {"description": "Default HEAD response"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_city_head(
    address_key: str = Path(None, description=""),
) -> None:
    """head operation for /addressbook/{address_key}/city"""
    ...


@router.put(
    "/addressbook/{address_key}/city",
    responses={
        200: {"model": str, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_city_put(
    address_key: str = Path(None, description=""),
) -> str:
    """put operation for /addressbook/{address_key}/city"""
    ...


@router.delete(
    "/addressbook/{address_key}/country",
    responses={
        200: {"model": str, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_country_delete(
    address_key: str = Path(None, description=""),
) -> str:
    """delete operation for /addressbook/{address_key}/country"""
    ...


@router.get(
    "/addressbook/{address_key}/country",
    responses={
        200: {"model": str, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_country_get(
    city: str = Query(None, description=""),
    address_key: str = Path(None, description=""),
) -> str:
    """get operation for /addressbook/{address_key}/country"""
    ...


@router.head(
    "/addressbook/{address_key}/country",
    responses={
        200: {"description": "Default HEAD response"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_country_head(
    address_key: str = Path(None, description=""),
) -> None:
    """head operation for /addressbook/{address_key}/country"""
    ...


@router.put(
    "/addressbook/{address_key}/country",
    responses={
        200: {"model": str, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_country_put(
    address_key: str = Path(None, description=""),
) -> str:
    """put operation for /addressbook/{address_key}/country"""
    ...


@router.delete(
    "/addressbook/{address_key}",
    responses={
        200: {"model": Addressbook, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_delete(
    address_key: str = Path(None, description=""),
) -> Addressbook:
    """delete operation for /addressbook/{address_key}"""
    ...


@router.get(
    "/addressbook/{address_key}",
    responses={
        200: {"model": Addressbook, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_get(
    city: str = Query(None, description=""),
    address_key: str = Path(None, description=""),
) -> Addressbook:
    """get operation for /addressbook/{address_key}"""
    ...


@router.head(
    "/addressbook/{address_key}",
    responses={
        200: {"description": "Default HEAD response"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_head(
    address_key: str = Path(None, description=""),
) -> None:
    """head operation for /addressbook/{address_key}"""
    ...


@router.patch(
    "/addressbook/{address_key}",
    responses={
        200: {"model": Addressbook, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_patch(
    address_key: str = Path(None, description=""),
) -> Addressbook:
    """patch operation for /addressbook/{address_key}"""
    ...


@router.delete(
    "/addressbook/{address_key}/person_uuid",
    responses={
        200: {"model": str, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_person_uuid_delete(
    address_key: str = Path(None, description=""),
) -> str:
    """delete operation for /addressbook/{address_key}/person_uuid"""
    ...


@router.get(
    "/addressbook/{address_key}/person_uuid",
    responses={
        200: {"model": str, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_person_uuid_get(
    city: str = Query(None, description=""),
    address_key: str = Path(None, description=""),
) -> str:
    """get operation for /addressbook/{address_key}/person_uuid"""
    ...


@router.head(
    "/addressbook/{address_key}/person_uuid",
    responses={
        200: {"description": "Default HEAD response"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_person_uuid_head(
    address_key: str = Path(None, description=""),
) -> None:
    """head operation for /addressbook/{address_key}/person_uuid"""
    ...


@router.put(
    "/addressbook/{address_key}/person_uuid",
    responses={
        200: {"model": str, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_person_uuid_put(
    address_key: str = Path(None, description=""),
) -> str:
    """put operation for /addressbook/{address_key}/person_uuid"""
    ...


@router.put(
    "/addressbook/{address_key}",
    responses={
        200: {"model": Addressbook, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_put(
    address_key: str = Path(None, description=""),
) -> Addressbook:
    """put operation for /addressbook/{address_key}"""
    ...


@router.delete(
    "/addressbook/{address_key}/state",
    responses={
        200: {"model": str, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_state_delete(
    address_key: str = Path(None, description=""),
) -> str:
    """delete operation for /addressbook/{address_key}/state"""
    ...


@router.get(
    "/addressbook/{address_key}/state",
    responses={
        200: {"model": str, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_state_get(
    city: str = Query(None, description=""),
    address_key: str = Path(None, description=""),
) -> str:
    """get operation for /addressbook/{address_key}/state"""
    ...


@router.head(
    "/addressbook/{address_key}/state",
    responses={
        200: {"description": "Default HEAD response"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_state_head(
    address_key: str = Path(None, description=""),
) -> None:
    """head operation for /addressbook/{address_key}/state"""
    ...


@router.put(
    "/addressbook/{address_key}/state",
    responses={
        200: {"model": str, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_state_put(
    address_key: str = Path(None, description=""),
) -> str:
    """put operation for /addressbook/{address_key}/state"""
    ...


@router.delete(
    "/addressbook/{address_key}/street",
    responses={
        200: {"model": str, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_street_delete(
    address_key: str = Path(None, description=""),
) -> str:
    """delete operation for /addressbook/{address_key}/street"""
    ...


@router.get(
    "/addressbook/{address_key}/street",
    responses={
        200: {"model": str, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_street_get(
    city: str = Query(None, description=""),
    address_key: str = Path(None, description=""),
) -> str:
    """get operation for /addressbook/{address_key}/street"""
    ...


@router.head(
    "/addressbook/{address_key}/street",
    responses={
        200: {"description": "Default HEAD response"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_street_head(
    address_key: str = Path(None, description=""),
) -> None:
    """head operation for /addressbook/{address_key}/street"""
    ...


@router.put(
    "/addressbook/{address_key}/street",
    responses={
        200: {"model": str, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_address_key_street_put(
    address_key: str = Path(None, description=""),
) -> str:
    """put operation for /addressbook/{address_key}/street"""
    ...


@router.delete(
    "/addressbook",
    responses={
        200: {"model": Addressbook, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_delete(
    limit: int = Query(None, description="Limit the number of responses back"),
    offset: int = Query(None, description="The offset to start returning resources"),
) -> Addressbook:
    """delete operation for /addressbook"""
    ...


@router.get(
    "/addressbook",
    responses={
        200: {"model": List[Addressbook], "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_get(
    city: str = Query(None, description=""),
    limit: int = Query(None, description="Limit the number of responses back"),
    offset: int = Query(None, description="The offset to start returning resources"),
) -> List[Addressbook]:
    """List all addresses in this addressbook"""
    ...


@router.head(
    "/addressbook",
    responses={
        200: {"description": "Default HEAD response"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_head(
    limit: int = Query(None, description="Limit the number of responses back"),
    offset: int = Query(None, description="The offset to start returning resources"),
) -> None:
    """Determine the existence and size of addresses in this addressbook"""
    ...


@router.patch(
    "/addressbook",
    responses={
        200: {"model": Addressbook, "description": "OK"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_patch(
    limit: int = Query(None, description="Limit the number of responses back"),
    offset: int = Query(None, description="The offset to start returning resources"),
) -> Addressbook:
    """Patch one or more addresses in this addressbook"""
    ...


@router.post(
    "/addressbook",
    responses={
        201: {"model": Addressbook, "description": "CREATED"},
    },
    tags=["addressbook"],
    response_model_by_alias=True,
)
async def addressbook_post(
    addressbook_post_request: AddressbookPostRequest = Body(None, description="The request body for /addressbook"),
    limit: int = Query(None, description="Limit the number of responses back"),
    offset: int = Query(None, description="The offset to start returning resources"),
) -> Addressbook:
    """Create a new address in this addressbook, a new address key will be created"""
    ...
