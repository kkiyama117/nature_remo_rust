import logging


FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
LOGGER = logging.getLogger(__package__)

from nature_remo_api import _rust_api as inner

rust_all = inner.__all__

from .api import NatureRemoAPI
# from .api import NatureRemoLocalAPI

__all__ = rust_all + [
    "NatureRemoAPI",
    # "NatureRemoError",
]
