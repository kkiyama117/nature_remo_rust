from dataclasses import dataclass
from datetime import datetime
from enum import auto

from . import LOGGER
from .__version__ import __url__
from .__version__ import __version__

BASE_URL = "https://api.nature.global"


def enable_debug_mode():
    import logging
    from http.client import HTTPConnection

    HTTPConnection.debuglevel = 1
    logging.basicConfig()
    logging.getLogger().setLevel(logging.DEBUG)


class NatureRemoAPI:
    pass
