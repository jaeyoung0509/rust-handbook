package snippets

import (
	"slices"
	"strings"
	"sync"
)

// #region worker-pool
func CollectJobs(inputs []string) []string {
	results := make([]string, 0, len(inputs))
	ch := make(chan string, len(inputs))
	var wg sync.WaitGroup

	for _, input := range inputs {
		wg.Add(1)
		go func(item string) {
			defer wg.Done()
			ch <- strings.ToUpper(item)
		}(input)
	}

	wg.Wait()
	close(ch)

	for result := range ch {
		results = append(results, result)
	}

	slices.Sort(results)
	return results
}

// #endregion worker-pool
