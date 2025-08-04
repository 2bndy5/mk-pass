from typing import NamedTuple

def main() -> None: ...

class PasswordRequirements(NamedTuple):
    length: int = 16
    numbers: int = 1
    specials: int = 1
    first_is_letter: bool = True

    def validate(self) -> "PasswordRequirements": ...

def generate_password(config: PasswordRequirements) -> str: ...
