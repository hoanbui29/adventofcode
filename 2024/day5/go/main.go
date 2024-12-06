package main

import (
	"strings"
	"testing"
)

func main() {

}

func parseInput(input string) (map[int][]int, []int) {
	parts := strings.Split(input, "\n\n")
	ruleRaw := strings.Split(parts[0], "\n")
	messages := strings.Split(parts[1], "\n")
	rules := make(map[int][]int)
	updates := make([]int, 0)

	for _, line := range ruleRaw[:len(ruleRaw)-1] {
		strings.Split(line, "|")
	}
}

func TestGetInput(t *testing.T) {
}
