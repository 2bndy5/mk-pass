import re
import sys
import pytest
from mk_pass import generate_password, PasswordRequirements, main


def assert_password_satisfies_default(password: str):
    first = password[0]
    assert first.isalpha()
    assert len(password) == 16
    letters = len(re.findall("[a-zA-Z]", password))
    assert letters == 14
    numbers = len(re.findall("[0-9]", password))
    assert numbers == 1
    specials = len(password) - numbers - letters
    assert specials == 1


def test_password() -> None:
    config = PasswordRequirements()  # default
    password = generate_password(config)
    assert_password_satisfies_default(password)


def test_config() -> None:
    config = PasswordRequirements(numbers=15, specials=15)
    expected = PasswordRequirements(numbers=13, specials=1)
    validated = config.validate()
    assert validated == expected


def test_main(monkeypatch: pytest.MonkeyPatch, capfd: pytest.CaptureFixture) -> None:
    monkeypatch.setattr(sys, "argv", ["mk-pass"])
    main()
    (out, err) = capfd.readouterr()
    password = out.rstrip("\n")
    assert_password_satisfies_default(password)
