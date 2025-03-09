import requests

from nature_remo_api import NatureRemoError, domain
from nature_remo_api.http import HTTPMethod
from nature_remo_api.request_generator import (  # type: ignore
    NatureRemoRequest,
    get_user_request,
    post_user_request,
)
from . import LOGGER, enable_debug_mode
from .interfaces.errors import RequestError
from .interfaces.raw_api import RequestResponseProvider, Response


def build_error_message(resp: requests.models.Response) -> str:
    error = resp.json()
    return f"HTTP Status Code: {resp.status_code}, Nature Remo Code: {error['code']}, Message: {error['message']}"


class NatureRemoAPIV1:
    def __init__(
        self,
        access_token: str,
        debug: bool = False,
        provider: RequestResponseProvider = None,
    ):
        if debug:
            enable_debug_mode()
        self.access_token = access_token
        self.api_version = 1
        self.base_url = f"https://api.nature.global/{self.api_version}"
        if provider is not None:
            self.provider = provider
        else:
            raise NatureRemoError(
                "No provider. Please use and add some HTTP client. Default is `DefaultRequestResponseProvider`"
            )

    def __request(self, meta: NatureRemoRequest, data: dict = None) -> Response:
        headers = meta.headers
        # headers = {
        #     "Accept": "application/json",
        #     "Authorization": f"Bearer {self.access_token}",
        #     "User-Agent": f"nature-remo/{__version__} ({__URL__})",
        # }
        url = f"{self.base_url}/{meta.endpoint}"

        LOGGER.debug("Call `__request`")
        try:
            if meta.http_method == HTTPMethod.GET:
                return self.provider.get(url, headers=headers)
            else:
                return self.provider.post(url, headers=headers, data=meta.data)
        except RequestError as e:
            raise NatureRemoError(e)

    def __get_json(self, resp: Response):
        self.__set_rate_limit(resp)
        if resp.ok:
            LOGGER.debug(f"Resp:{resp.json()}")
            return resp.json()
        else:
            # Response has error
            LOGGER.error(f"Resp:{resp.json()}")
            raise NatureRemoError(build_error_message(resp))

    def __request_and_get_json(self, meta: NatureRemoRequest):
        return self.__get_json(self.__request(meta))

    def __set_rate_limit(self, resp):
        # TODO: implement
        pass

    def get_appliances(self) -> list[domain.Appliance]:
        """Fetch the list of appliances.

        Returns:
            A list of Appliance objects.
        """
        try:
            meta = NatureRemoRequest("/appliances", HTTPMethod.GET)
            json = self.__request_and_get_json(meta)
            return [domain.Appliance.from_json(data) for data in json]
        except NatureRemoError as e:
            raise e

    def get_devices(self) -> list[domain.Device]:
        """Fetch the list of Remo devices the user has access to.

        Returns:
            A List of Device objects.
        """
        try:
            meta = CommandMetaData("/devices", HTTPMethod.GET)
            json = self.__request_and_get_json(meta)
            return [domain.Device.from_json(data) for data in json]
        except NatureRemoError as e:
            raise e

    def update_device(self, device_id: str, name: str) -> domain.Device:
        """Update Remo.

        Args:
            device_id: Device ID.
            name: Device name to Update.
        """
        try:
            meta = CommandMetaData(f"/devices/{device_id}", HTTPMethod.POST)
            json = self.__request_and_get_json(meta, {"name": name})
            return domain.Device.from_json(json)
        except NatureRemoError as e:
            raise e

    def get_user(self) -> domain.User:
        """Fetch the authenticated user's information.

        Returns:
            A User object.
        """
        try:
            meta = get_user_request(self.access_token)
            json = self.__request_and_get_json(meta)
            # return domain.User.from_json(json)
            return domain.User(**json)
        except NatureRemoError as e:
            raise e

    def update_user(self, user: domain.UserProfileParam) -> domain.User:
        """Update authenticated user's information.

        Args:
            nickname: User's nickname.

        Returns:
            A User object.
            :param user: User for update.
        """
        try:
            meta = post_user_request(self.access_token, user)
            json = self.__request_and_get_json(meta)
            return domain.User(**json)
        except NatureRemoError as e:
            raise e
