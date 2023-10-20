import electora_core


def test_greet_py():
    result = electora_core.greet_py("Python")
    assert result == "Hello, Python!"

