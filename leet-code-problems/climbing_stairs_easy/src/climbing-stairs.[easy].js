// You are climbing a staircase. It takes n steps to reach the top.

// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?


function climbStairs (n, memo = {}) {
  let solutionCount = 0;

  if (n in memo) return memo[n];
  if (n == 0) return 1;
  if (n < 0) return 0;

  memo[n-1] = climbStairs(n - 1, memo);
  memo[n-2] = climbStairs(n - 2, memo);

  solutionCount += memo[n-1];
  solutionCount += memo[n-2];
  
  return solutionCount;
}

console.log(climbStairs(2));
console.log(climbStairs(4));
console.log(climbStairs(100));
