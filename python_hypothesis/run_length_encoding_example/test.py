from run_length_encoding import encode, decode
from hypothesis import example, given, strategies as st


@given(st.text())
@example('')
def test_decode_inverts_encode(s):
    assert decode(encode(s)) == s
