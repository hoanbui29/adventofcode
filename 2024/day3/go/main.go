package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func get_input() string {
	raw, err := os.ReadFile("input.txt")

	if err != nil {
		panic(err)
	}

	return string(raw)
}

func calc_mul(x string, y string) int {
	first, err := strconv.Atoi(x)
	if err != nil {
		panic(err)
	}
	second, err := strconv.Atoi(y)
	if err != nil {
		panic(err)
	}
	return first * second
}

func part_one(s string) {
	r := regexp.MustCompile(`mul\((\d+),(\d+)\)`)
	muls := r.FindAllStringSubmatch(s, -1)
	total := 0
	for _, mul := range muls {
		total += calc_mul(mul[1], mul[2])
	}
	fmt.Printf("Part one: %d\n", total)
}

func part_two(s string) {
	r := regexp.MustCompile(`mul\((\d+),(\d+)\)|(d)(o)(?:n't)?\(\)`)
	enabled := true
	total := 0
	for _, match := range r.FindAllStringSubmatch(s, -1) {
		switch match[0] {
		case "don't()":
			enabled = false
		case "do()":
			enabled = true
		default:
			if enabled {
				total += calc_mul(match[1], match[2])
			}
		}
	}
	fmt.Printf("Part two: %d\n", total)
}

func main() {
	input := get_input()
	fmt.Println("Go solution")
	part_one(input)
	part_two(input)
	fmt.Println("--------------------------------------------------")
}
