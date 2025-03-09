import os

from nature_remo_api import NatureRemoError
from nature_remo_api.api import NatureRemoAPIV1
from nature_remo_api.interfaces.raw_api.default import DefaultRequestResponseProvider


async def main_wait():
    from nature_remo_api import tester

    print("wait_1")
    await tester.rust_sleep()
    print("wait_2")
    await tester.call_rust_sleep()
    print("wait_fin")


def test_import_domain():
    from nature_remo_api.http import HTTPMethod

    temp = HTTPMethod.GET
    print(f"Get Method: {temp}")
    # asyncio.run(main_wait())
    from nature_remo_api import http, tester

    temp = http.HTTPMethod.GET
    print(f"Get Method: {temp}")
    assert (tester.add(1, 2), 1 + 2)


def test_get_user():
    from nature_remo_api import domain

    access_token = os.environ.get("ACCESS_TOKEN")
    # is_debug = False
    is_debug = True
    assert len(access_token) != 0
    provider = DefaultRequestResponseProvider()
    api = NatureRemoAPIV1(access_token=access_token, provider=provider, debug=is_debug)
    user = api.get_user()
    print(user)
    assert isinstance(user, domain.User)


def test_get_devices():
    from nature_remo_api import domain

    access_token = os.environ.get("ACCESS_TOKEN")
    # is_debug = False
    is_debug = True
    assert len(access_token) != 0
    provider = DefaultRequestResponseProvider()
    api = NatureRemoAPIV1(access_token=access_token, provider=provider, debug=is_debug)
    devices = api.get_devices()
    print(f"Devices:{devices}")
    for device in devices:
        print(device)
        assert isinstance(device, domain.Device)
