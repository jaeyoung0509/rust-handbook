#region enum-match-compare
from dataclasses import dataclass


@dataclass(frozen=True)
class InReview:
    reviewers: int

#endregion enum-match-compare
