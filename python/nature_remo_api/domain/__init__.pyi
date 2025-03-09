from dataclasses import dataclass

@dataclass
class User:
    id: str
    nickname: str
    country: str | None
    distance_unit: str | None
    temp_unit: str | None
    superuser: bool | None

@dataclass
class UserProfileParam:
    nickname: str | None
    country: str | None
    distance_unit: str | None
    temp_unit: str | None
