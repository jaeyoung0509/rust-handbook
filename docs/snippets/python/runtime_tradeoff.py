#region hidden-runtime-cost
def enrich_usernames(users: list[str]) -> list[str]:
    return [user.strip().lower() for user in users]
#endregion hidden-runtime-cost
