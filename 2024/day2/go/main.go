package main

import (
	"fmt"
	"math"
	"os"
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

func parse_input(input string) [][]int {
	lines := strings.Split(input, "\n")
	lines = lines[:len(lines)-1] // Remove last empty line
	result := make([][]int, 0)
	for _, line := range lines {
		parts := strings.Split(line, " ")
		line_records := make([]int, 0)
		for _, part := range parts {
			conv, err := strconv.Atoi(part)
			if err != nil {
				panic(err)
			}
			line_records = append(line_records, conv)
		}

		result = append(result, line_records)
	}
	return result
}

func process_line(data []int) int {
	if data[0] == data[1] {
		return 0
	}

	is_increase := data[0] < data[1]

	for i := 1; i < len(data); i++ {
		if (is_increase && data[i] <= data[i-1]) || (!is_increase && data[i] >= data[i-1]) {
			return 0
		}

		if math.Abs(float64(data[i])-float64(data[i-1])) > 3 {
			return 0
		}
	}

	return 1

}

func process_line_tolerance(data []int) int {
	base_result := process_line(data)
	if base_result > 0 {
		return base_result
	}

	for i := range data {
		new_data := make([]int, 0)
		for j := range data {
			if i == j {
				continue
			}
			new_data = append(new_data, data[j])
		}
		res := process_line(new_data)
		if res > 0 {
			return res
		}
	}

	return 0
}

func part_one(input string) {
	data := parse_input(input)
	total := 0
	for _, d := range data {
		total += process_line(d)
	}
	fmt.Printf("Part one: %d\n", total)
}

func part_two(input string) {
	data := parse_input(input)
	total := 0
	for _, d := range data {
		total += process_line_tolerance(d)
	}
	fmt.Printf("Part two: %d\n", total)
}

func main() {
	input := get_input()
	fmt.Println("Go solution")
	part_one(input)
	part_two(input)
	fmt.Println("---------------------------------------------")
}
