fn main() {
    let n = 101;
    println!("Sum of 1 to {} is {}", n, sum_to_n_a(n));
    println!("Sum of 1 to {c} is {}", n, sum_to_n_b(n));
    println!("Sum of 1 to {} is {}", n, sum_to_n_c(n));
    
}

fn sum_to_n_a(n:i32) -> i32 {
    // iterative solution
    // Time complexity: O(n), Space complexity: O(1)

    let mut sum = 0;
    for i in 1..n+1 {
        sum += i;
    }
    sum
	
}

fn sum_to_n_b(n:i32) -> i32 {
    // recursive solution
    // Time complexity: O(n), Space complexity: O(n)
    if n == 0 {
        return 0;
    }
    n + sum_to_n_b(n-1)
}

fn sum_to_n_c(n:i32) -> i32{
	// formula solution
    // Time complexity: O(1), Space complexity: O(1)
    // prevent n*(n+1) from overflowing
    if n % 2 == 0 {
        n/2 * (n+1)
    } else {
        (n+1)/2 * n
    }
}
