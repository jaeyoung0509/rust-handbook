package snippets

// #region longer-value
func Longer(left string, right string) string {
	if len(left) >= len(right) {
		return left
	}

	return right
}

// #endregion longer-value
