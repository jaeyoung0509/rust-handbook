#region enum-match-compare
from dataclasses import dataclass


@dataclass(frozen=True)
class Publication:
    status: str
    reviewers: int
    published: bool

#endregion enum-match-compare
