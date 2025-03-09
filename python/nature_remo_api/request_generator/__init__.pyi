from nature_remo_api.domain import UserProfileParam
from nature_remo_api.http import HTTPMethod

class NatureRemoRequest:
    endpoint: str
    http_method: HTTPMethod
    headers: dict[str, str]
    data: object | None

def get_user_request(
    access_token: str,
) -> NatureRemoRequest:
    pass

def post_user_request(access_token: str, user: UserProfileParam) -> NatureRemoRequest:
    pass
