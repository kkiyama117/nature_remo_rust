import nature_remo_api
from nature_remo_api import inner

def test_normal_hello():
    print(inner.hello())
    assert (inner.add(), 3)

def test_import_domain():
    temp = inner.Method.Get
    print(f"Get Method: {temp}")
    assert True