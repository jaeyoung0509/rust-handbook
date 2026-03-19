#region option-result-flow
def find_note(notes: list[dict[str, str]], title: str) -> dict[str, str] | None:
    return next((note for note in notes if note["title"] == title), None)


def parse_note_line(line: str) -> dict[str, str]:
    if "|" not in line:
        raise ValueError("missing separator")

    title, body = [part.strip() for part in line.split("|", 1)]
    if not title:
        raise ValueError("empty title")
    if not body:
        raise ValueError("empty body")

    return {"title": title, "body": body}
#endregion option-result-flow
