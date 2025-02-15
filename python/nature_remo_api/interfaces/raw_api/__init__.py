from typing import TypeAlias, Mapping, Any

_HeadersMapping: TypeAlias = Mapping[str, str | bytes | None]


class Request:
    def __init__(self):
        pass

    @property
    def headers(self) -> _HeadersMapping | None:
        return None


class Response:
    def __init__(self):
        pass

    @property
    def status_code(self):
        raise NotImplementedError

    @property
    def ok(self):
        raise NotImplementedError

    def json(self):
        raise NotImplementedError


class RequestResponseProvider:
    """Wrapper of HTTP Library, like `requests`, `aiohttp` and so on"""

    @staticmethod
    def get(url: str, headers: _HeadersMapping | None) -> Any:
        raise NotImplementedError

    @staticmethod
    def post(url: str, headers: _HeadersMapping | None, data: Any) -> Any:
        raise NotImplementedError
