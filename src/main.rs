#[allow(dead_code)]
fn knapsack(n: i32, w: i32, profit: Vec<i32>, weight: Vec<i32>) -> i32 {
    fn go(n: usize, w: i32, profit: &Vec<i32>, weight: &Vec<i32>, dp: &mut Vec<Vec<i32>>) -> i32 {
        if n == 0 || w == 0 {
            return 0;
        } else if dp[n][w as usize] != -1 {
            dp[n][w as usize]
        } else if weight[n - 1] > w {
            dp[n][w as usize] = go(n - 1, w, profit, weight, dp);
            dp[n][w as usize]
        } else {
            let take = profit[n - 1] + go(n - 1, w - weight[n - 1], profit, weight, dp);
            let leave = go(n - 1, w, profit, weight, dp);
            dp[n][w as usize] = take.max(leave);
            dp[n][w as usize]
        }
    }

    let mut dp = vec![vec![-1; (w + 1) as usize]; (n + 1) as usize];
    go(n as usize, w, &profit, &weight, &mut dp)
}

mod tests {
    #[test]
    fn test_knapsack() {
        assert_eq!(
            super::knapsack(3, 50, vec![60, 100, 120], vec![10, 20, 30]),
            220
        );
        assert_eq!(super::knapsack(1, 1, vec![1], vec![1]), 1);
        assert_eq!(super::knapsack(3, 4, vec![1, 2, 3], vec![4, 5, 1]), 3);
    }
}

fn main() {}
