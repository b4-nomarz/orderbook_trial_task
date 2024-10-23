/// Core functions of the domain
use std::ops::{Add, Div};

/*
  Calculates the average order book price according to the spec given
  in the task prompt

  Average price of order book = (Sum of Asks + Sum of Bids ) / Number of Asks and Bids
*/
pub fn average_price_of_order_book(asks: Vec<f32>, bids: Vec<f32>) -> f32 {
    let sum = |values: &Vec<f32>| values.into_iter().fold(0f32, |acc, v| acc.add(v));

    let asks_sum = sum(&asks);
    let bids_sum = sum(&bids);

    let asks_len = asks.len();
    let bids_len = asks.len();

    let sum_prices = asks_sum.add(bids_sum);

    let sum_len = {
        let total = asks_len.add(bids_len);

        // coerce type from usize to f32
        total as f32
    };

    sum_prices.div(sum_len)
}

#[cfg(test)]
mod tests {
    use std::ops::{Add, Div};

    use super::average_price_of_order_book;

    #[test]
    fn test_average_price() {
        // setup
        let asks: Vec<f32> = vec![1f32, 2f32, 3f32, 4f32];
        let bids: Vec<f32> = vec![1f32, 2f32, 3f32, 4f32, 5f32];

        let sum = |values: &Vec<f32>| values.into_iter().fold(0f32, |acc, v| acc.add(v));

        let setup_avg_price = {
            let asks_sum = sum(&asks);
            let bids_sum = sum(&bids);

            let asks_len = asks.len();
            let bids_len = asks.len();

            let sum_prices = asks_sum.add(bids_sum);

            let sum_len = {
                let total = asks_len.add(bids_len);

                // coerce type from usize to f32
                total as f32
            };

            sum_prices.div(sum_len)
        };

        let _test_fn = {
            let price = average_price_of_order_book(asks, bids);

            assert_eq!(price, setup_avg_price)
        };
    }
}
