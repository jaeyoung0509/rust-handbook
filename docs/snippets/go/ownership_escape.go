package snippets

// #region slice-view
func SumFirstTwo(values []int) (int, bool) {
	if len(values) < 2 {
		return 0, false
	}

	return values[0] + values[1], true
}

// #endregion slice-view

// #region borrow-splitting
func BumpFirstTwo(values []int, delta int) (int, int, bool) {
	if len(values) < 2 {
		return 0, 0, false
	}

	values[0] += delta
	values[1] += delta

	return values[0], values[1], true
}

// #endregion borrow-splitting
