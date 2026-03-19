package snippets

// #region shared-ownership-compare
func BuildReviewTimeline() []string {
	draft := []string{"draft created"}
	reviewer := &draft
	editor := &draft

	*editor = append(*editor, "review requested")
	*reviewer = append(*reviewer, "comments addressed")
	return draft
}
// #endregion shared-ownership-compare
