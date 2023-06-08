use rand::distributions::{Distribution, Normal};
use rand::Rng;

// Struct to hold the simulation result
#[derive(Debug)]
pub struct SimulationResult {
    pub average_return: f64,
    pub std_deviation: f64,
}

// Monte Carlo Trading function
pub fn monte_carlo_trading(initial_investment: f64, num_simulations: u32) -> SimulationResult {
    let mean_return = 0.08; // Mean return of the trading strategy
    let volatility = 0.2; // Volatility of the trading strategy

    let mut rng = rand::thread_rng();

    let normal_distribution = Normal::new(mean_return, volatility);

    let mut returns = Vec::new();

    for _ in 0..num_simulations {
        let random_number = rng.sample(&normal_distribution);
        let end_return = initial_investment * (1.0 + random_number);

        returns.push(end_return);
    }

    let sum: f64 = returns.iter().sum();
    let average_return = sum / returns.len() as f64;

    let squared_differences: f64 = returns
        .iter()
        .map(|&x| (x - average_return).powi(2))
        .sum();
    let variance = squared_differences / returns.len() as f64;
    let std_deviation = variance.sqrt();

    SimulationResult {
        average_return,
        std_deviation,
    }
}
