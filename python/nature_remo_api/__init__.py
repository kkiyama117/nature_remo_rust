import logging
from http.client import HTTPConnection

LOGGER = logging.getLogger(__package__)

# To merge python and rust
# See [https://www.maturin.rs/project_layout]
from nature_remo_api import _rust_api
from nature_remo_api._rust_api import *

__doc__ = _rust_api.__doc__
if hasattr(_rust_api, "__all__"):
    __all__ = _rust_api.__all__
else:
    __all__ = []


def enable_debug_mode():
    import logging

    _format = (
        "%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s"
    )
    HTTPConnection.debuglevel = 1
    logging.basicConfig(format=_format)
    LOGGER.setLevel(logging.DEBUG)


__all__ += ["enable_debug_mode", "interfaces", "api"]
