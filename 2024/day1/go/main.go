package main

import (
	"fmt"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

func get_input() string {
	data, err := os.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}

	return string(data)
}

func parseInt(raw string) int {
	num, err := strconv.Atoi(raw)
	if err != nil {
		panic(err)
	}
	return num
}

func parse_input(input string) ([]int, []int) {
	lines := strings.Split(input, "\n")
	left_ids := make([]int, 0)
	right_ids := make([]int, 0)
	for _, line := range lines {
		if line == "" { // EOF
			break
		}
		parts := strings.Split(line, "   ")
		left_ids = append(left_ids, parseInt(parts[0]))
		right_ids = append(right_ids, parseInt(parts[1]))
	}
	return left_ids, right_ids
}
func part_one() {
	input := get_input()
	left_ids, right_ids := parse_input(input)
	sort.Ints(left_ids)
	sort.Ints(right_ids)
	total := 0
	for i := range left_ids {
		left_id := left_ids[i]
		right_id := right_ids[i]
		total += (int)(math.Abs(float64(left_id) - float64(right_id)))
	}

	fmt.Printf("Part one: %d\n", total)
}

func part_two() {
	input := get_input()
	left_ids, right_ids := parse_input(input)
	right_map := make(map[int]int)
	for _, id := range right_ids {
		if value, ok := right_map[id]; ok {
			right_map[id] = value + 1
		} else {
			right_map[id] = 1
		}
	}

	total := 0
	for _, id := range left_ids {
		if value, ok := right_map[id]; ok {
			total += value * id
		}
	}

	fmt.Printf("Part two: %d\n", total)
}

func main() {
	fmt.Println("Go solutions")
	part_one()
	part_two()
}
