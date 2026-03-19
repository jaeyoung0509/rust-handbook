#region hidden-copy
def decorate_title(title: str, tag: str) -> str:
    return f"{title}::{tag}"
#endregion hidden-copy

#region borrow-splitting
def bump_first_two(values: list[int], delta: int) -> tuple[int, int] | None:
    if len(values) < 2:
        return None

    values[0] += delta
    values[1] += delta
    return values[0], values[1]
#endregion borrow-splitting
