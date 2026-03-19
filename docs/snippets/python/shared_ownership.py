#region shared-ownership-compare
def build_review_timeline() -> list[str]:
    draft = ["draft created"]
    reviewer = draft
    editor = draft

    editor.append("review requested")
    reviewer.append("comments addressed")
    return draft
#endregion shared-ownership-compare
