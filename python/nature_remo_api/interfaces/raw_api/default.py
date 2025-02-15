import requests

from nature_remo_api import LOGGER
from . import *
from . import _HeadersMapping
from ..errors import RequestError


class DefaultResponse(Response):
    def __init__(self, resp: requests.models.Response):
        super().__init__()
        self.__inner = resp

    @property
    def ok(self):
        return self.__inner.ok

    @property
    def status_code(self):
        return self.__inner.status_code

    def json(self):
        LOGGER.debug(self.__inner.json())
        return self.__inner.json()


class DefaultRequestResponseProvider(RequestResponseProvider):
    @staticmethod
    def get(url: str, headers: _HeadersMapping | None):
        try:
            LOGGER.debug("SEND GET BY DEFAULT PROVIDER")
            LOGGER.debug(headers)
            resp = requests.get(url, headers=headers)
            LOGGER.debug(resp)
            LOGGER.debug("SEND GET BY DEFAULT PROVIDER FINISHED")
            return DefaultResponse(resp)

        except requests.RequestException as e:
            raise RequestError(e)

    # TODO: Add data type
    @staticmethod
    def post(
        url: str, headers: _HeadersMapping | None, data: Any
    ) -> DefaultResponse | None:
        try:
            LOGGER.debug("SEND POST BY DEFAULT PROVIDER")
            LOGGER.debug(headers)
            LOGGER.debug(data)
            resp = requests.post(url, headers=headers, data=data)
            LOGGER.debug(resp)
            LOGGER.debug("SEND POST BY DEFAULT PROVIDER FINISHED")
            return DefaultResponse(resp)
        except requests.RequestException as e:
            raise RequestError(e)
