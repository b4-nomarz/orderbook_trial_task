/// Core functions of the domain

/*
  Calculates the average order book price according to the spec given
  in the task prompt

  Average price of order book = (Sum of Asks + Sum of Bids ) / Number of Asks and Bids
*/
pub fn average_price_of_order_book(asks: Vec<f32>, bids: Vec<f32>) -> f32 {
    let sum_of_ask_prices = asks.into_iter().fold::<f32, _>(0f32, |acc, el| acc + el);

    let sum_of_bid_prices = bids.into_iter().fold::<f32, _>(0f32, |acc, el| acc + el);

    let length_of_asks_and_bids = {
        let a: u64 = asks.len().into();
        let b = bids.len();

        a + b
    };

    // TODO fix division error caused by type mismatch
    let avg = (sum_of_bid_prices, sum_of_ask_prices) / length_of_asks_and_bids;

    avg
}
