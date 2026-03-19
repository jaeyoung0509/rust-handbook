#region option-result-flow
class NoteParseError(Exception):
    pass


class CatalogError(Exception):
    pass


def find_note(notes: list[dict[str, str]], title: str) -> dict[str, str] | None:
    return next((note for note in notes if note["title"] == title), None)


def parse_note_line(line: str) -> dict[str, str]:
    if "|" not in line:
        raise NoteParseError("missing separator")

    title, body = [part.strip() for part in line.split("|", 1)]
    if not title:
        raise NoteParseError("empty title")
    if not body:
        raise NoteParseError("empty body")

    return {"title": title, "body": body}


def preview_note_line(notes: list[dict[str, str]], line: str) -> str:
    try:
        parsed = parse_note_line(line)
    except NoteParseError as exc:
        raise CatalogError(f"parse error in `{line}`") from exc

    note = find_note(notes, parsed["title"])
    if note is None:
        raise CatalogError(f"note not found: {parsed['title']}")

    return f"{note!r}"
#endregion option-result-flow
