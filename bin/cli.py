import os

from nature_remo_api.api import NatureRemoAPIV1
from nature_remo_api.domain import UserProfileParam
from nature_remo_api.http import HTTPMethod
from nature_remo_api.interfaces.raw_api.default import DefaultRequestResponseProvider


def get_user():
    print("---------------------")
    # Get Env value
    access_token = os.environ.get("ACCESS_TOKEN")
    print(access_token)
    is_debug = os.environ.get('DEBUG', 'False').lower() == 'true'
    # Run command
    provider = DefaultRequestResponseProvider()
    api = NatureRemoAPIV1(access_token=access_token, provider=provider, debug=is_debug)
    user = api.get_user()
    print(repr(user))
    user = api.update_user(UserProfileParam(nickname="木山 航平",
                                            country="JP",
                                            temp_unit="c",
                                            # distance_unit="m"
                                            ))
    print(repr(user))


def get_devices():
    access_token = os.environ.get("ACCESS_TOKEN")
    is_debug = False
    # is_debug = True
    provider = DefaultRequestResponseProvider()
    api = NatureRemoAPIV1(access_token=access_token, provider=provider, debug=is_debug)
    user = api.get_user()
    print(user)
    devices = api.get_devices()
    for device in devices:
        print("---------------------------------")
        print(device)
        print("---------------------------------")
        print(repr(device))
        print("---------------------------------")
        print(f"users:{device.users}")


def update_devices():
    access_token = os.environ.get("ACCESS_TOKEN")
    is_debug = False
    provider = DefaultRequestResponseProvider()
    api = NatureRemoAPIV1(access_token=access_token, provider=provider, debug=is_debug)
    devices = api.get_devices()
    print(devices)
    first = devices.pop()
    _id = first.id
    updated = api.update_device(_id, "main")
    print(updated)


def get_appliances():
    print(HTTPMethod.GET)
    print("---------------------")
    access_token = os.environ.get("ACCESS_TOKEN")
    is_debug = False
    provider = DefaultRequestResponseProvider()
    api = NatureRemoAPIV1(access_token=access_token, provider=provider, debug=is_debug)
    appliances = api.get_appliances()
    print(appliances)


if __name__ == "__main__":
    # get_appliances()
    get_user()
