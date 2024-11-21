struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> (i32, usize, usize) {
        if prices.is_empty() {
            return (0, 0, 0);
        }

        let mut min_price = prices[0];
        let mut min_price_index = 0;
        let mut max_profit = 0;
        let mut buy_day = 0;
        let mut sell_day = 0;

        for (i, &price) in prices.iter().enumerate() {
            if price < min_price {
                min_price = price;
                min_price_index = i;
            } else {
                let profit = price - min_price;
                if profit > max_profit {
                    max_profit = profit;
                    buy_day = min_price_index;
                    sell_day = i;
                }
            }
        }

        (max_profit, buy_day, sell_day)
    }
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let (max_profit, buy_day, sell_day) = Solution::max_profit(prices);
    println!(
        "Max Profit: {}, Buy Day: {}, Sell Day: {}",
        max_profit, buy_day, sell_day
    );
}
