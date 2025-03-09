"""THIS IS WRAPPER OF NATURE_REMO_API rust_api"""

import logging

from .http import RateLimit

class NatureRemoAPIClient:
    access_token: str
    base_url: str
    last_rate_limit: RateLimit

LOGGER: logging.Logger
