import sys
import pytest
from mk_pass import (
    generate_password,
    PasswordRequirements,
    main,
    LOWERCASE,
    UPPERCASE,
    SPECIAL_CHARACTERS,
    DECIMAL,
)


def assert_password_is_expected(password: str, config: PasswordRequirements):
    first = password[0]
    assert first in LOWERCASE or first in UPPERCASE
    assert len(password) == config.length
    lowercase = sum([1 for x in password if x in LOWERCASE])
    uppercase = sum([1 for x in password if x in UPPERCASE])
    letters = uppercase + lowercase
    assert letters == (config.length - config.decimal - config.specials)
    decimal = sum([1 for x in password if x in DECIMAL])
    assert decimal == config.decimal
    specials = sum([1 for x in password if x in SPECIAL_CHARACTERS])
    assert specials == config.specials
    repeats = sum([1 for x in password if password.count(x) > 1])
    assert (repeats > 0) == config.allow_repeats


def test_password() -> None:
    config = PasswordRequirements()  # default
    password = generate_password(config)
    assert_password_is_expected(password, config)


def test_repeats() -> None:
    config = PasswordRequirements(decimal=18, specials=0, length=20, allow_repeats=True)
    password = generate_password(config)
    assert_password_is_expected(password, config)


def test_config() -> None:
    config = PasswordRequirements(decimal=15, specials=15)
    expected = PasswordRequirements(decimal=13, specials=1)
    validated = config.validate()
    assert validated == expected


def test_main(monkeypatch: pytest.MonkeyPatch, capfd: pytest.CaptureFixture) -> None:
    monkeypatch.setattr(sys, "argv", ["mk-pass"])
    main()
    (out, err) = capfd.readouterr()
    password = out.rstrip("\n")
    assert_password_is_expected(password, PasswordRequirements())
