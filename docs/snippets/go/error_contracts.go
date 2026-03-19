package snippets

import (
	"fmt"
	"strings"
)

// #region option-result-flow
type Note struct {
	Title string
	Body  string
}

type NoteParseError struct {
	Reason string
}

func (e NoteParseError) Error() string {
	return e.Reason
}

type CatalogError struct {
	Reason string
	Err    error
}

func (e CatalogError) Error() string {
	return e.Reason
}

func (e CatalogError) Unwrap() error {
	return e.Err
}

func FindNote(notes []Note, title string) (Note, bool) {
	for _, note := range notes {
		if note.Title == title {
			return note, true
		}
	}

	return Note{}, false
}

func ParseNoteLine(line string) (Note, error) {
	if !strings.Contains(line, "|") {
		return Note{}, NoteParseError{Reason: "missing separator"}
	}

	parts := strings.SplitN(line, "|", 2)
	title := strings.TrimSpace(parts[0])
	body := strings.TrimSpace(parts[1])
	if title == "" {
		return Note{}, NoteParseError{Reason: "empty title"}
	}
	if body == "" {
		return Note{}, NoteParseError{Reason: "empty body"}
	}

	return Note{Title: title, Body: body}, nil
}

func PreviewNoteLine(notes []Note, line string) (string, error) {
	parsed, err := ParseNoteLine(line)
	if err != nil {
		return "", CatalogError{
			Reason: fmt.Sprintf("parse error in `%s`", line),
			Err:    err,
		}
	}

	note, ok := FindNote(notes, parsed.Title)
	if !ok {
		return "", CatalogError{
			Reason: fmt.Sprintf("note not found: %s", parsed.Title),
		}
	}

	return fmt.Sprintf("%+v", note), nil
}

// #endregion option-result-flow
