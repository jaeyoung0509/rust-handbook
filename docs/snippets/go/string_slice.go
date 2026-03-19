package snippets

import "strings"

// #region string-slice-compare
func NormalizeUsername(raw string) string {
	return strings.ToLower(strings.TrimSpace(raw))
}
// #endregion string-slice-compare
