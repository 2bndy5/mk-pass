import sys
import pytest
from mk_pass import (
    generate_password,
    PasswordRequirements,
    main,
    LOWERCASE,
    UPPERCASE,
    SPECIAL_CHARACTERS,
    NUMBERS,
)


def assert_password_satisfies_default(password: str):
    first = password[0]
    assert first in LOWERCASE or first in UPPERCASE
    assert len(password) == 16
    lowercase = sum([1 for x in password if x in LOWERCASE])
    uppercase = sum([1 for x in password if x in UPPERCASE])
    assert uppercase + lowercase == 14
    numbers = sum([1 for x in password if x in NUMBERS])
    assert numbers == 1
    specials = sum([1 for x in password if x in SPECIAL_CHARACTERS])
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
