package snippets

// #region slice-view
func SumFirstTwo(values []int) (int, bool) {
	if len(values) < 2 {
		return 0, false
	}

	return values[0] + values[1], true
}

// #endregion slice-view
