package main

import "fmt"

func main() {
	n := 101
	fmt.Println(sum_to_n_a(n))
	fmt.Println(sum_to_n_b(n))
	fmt.Println(sum_to_n_c(n))
}

func sum_to_n_a(n int) int {
	// Iterative solution
	// Time complexity: O(n), Space complexity: O(1)
	sum := 0
	for i := 1; i <= n; i++ {
		sum += i
	}
	return sum
}

func sum_to_n_b(n int) int {
	// Recursive solution
	// Time complexity: O(n), Space complexity: O(n)
	if n == 1 {
		return 1
	}
	return n + sum_to_n_b(n-1)
}

func sum_to_n_c(n int) int {
	// Formula solution
	// Time complexity: O(1), Space complexity: O(1)
	// Prevent overflow
	if n%2 == 0 {
		return (1 + n) * (n / 2)
	} else {
		return ((1 + n) / 2) * n
	}
}
