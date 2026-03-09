#region longer-value
def longer(left: str, right: str) -> str:
    return left if len(left) >= len(right) else right
#endregion longer-value
