package snippets

import (
	"errors"
	"strings"
)

// #region option-result-flow
type Note struct {
	Title string
	Body  string
}

var (
	ErrMissingSeparator = errors.New("missing separator")
	ErrEmptyTitle       = errors.New("empty title")
	ErrEmptyBody        = errors.New("empty body")
)

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
		return Note{}, ErrMissingSeparator
	}

	parts := strings.SplitN(line, "|", 2)
	title := strings.TrimSpace(parts[0])
	body := strings.TrimSpace(parts[1])
	if title == "" {
		return Note{}, ErrEmptyTitle
	}
	if body == "" {
		return Note{}, ErrEmptyBody
	}

	return Note{Title: title, Body: body}, nil
}

// #endregion option-result-flow
