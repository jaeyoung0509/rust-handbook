package snippets

import "strings"

// #region hidden-runtime-cost
func EnrichUsernames(users []string) []string {
	result := make([]string, 0, len(users))
	for _, user := range users {
		result = append(result, strings.ToLower(strings.TrimSpace(user)))
	}

	return result
}

// #endregion hidden-runtime-cost
