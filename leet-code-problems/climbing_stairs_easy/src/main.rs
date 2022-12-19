// You are climbing a staircase. It takes n steps to reach the top.

// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

// Input: n (u32)
// Output: m (u32)

// Modeling:
// 0: 0
// 1: 1
// 2: 2
// 3: 1,1,1 | 1,2 | 2,1 = 3
// 4: 1,1,1,1 | 1,2,1 | 2,1,1 | 1,1,2 | 2,2 = 5 
// .. Indicates a binary tree, recursion problem!

// Base cases:
// 0: return 1 successful solution.

fn main() {
    println!("Step combinations for 1-step stairs: {}", climb_stairs(1));
    println!("Step combinations for 2-step stairs: {}", climb_stairs(2));
    println!("Step combinations for 3-step stairs: {}", climb_stairs(3));
    println!("Step combinations for 4-step stairs: {}", climb_stairs(4));
    println!("Step combinations for 5-step stairs: {}", climb_stairs(5));
    println!("Step combinations for 6-step stairs: {}", climb_stairs(6));
    println!("Step combinations for 7-step stairs: {}", climb_stairs(7));
}

fn climb_stairs(n: i32) -> u32 {
    let mut solutions = 0;

    if n == 0 {
        return 1;
    }

    if n < 0 {
        return 0;
    }

    solutions += climb_stairs(n-1);
    solutions += climb_stairs(n-2);

    solutions
}
