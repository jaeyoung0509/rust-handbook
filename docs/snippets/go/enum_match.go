package snippets

// #region enum-match-compare
type PublishingState int

const (
	Draft PublishingState = iota
	InReview
	Published
)
// #endregion enum-match-compare
