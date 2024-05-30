use rust_liquidity::{
    Errors, LpPool, LpTokenAmount, Percentage, Price, StakedTokenAmount, TokenAmount,
    SCALING_FACTOR,
};

fn main() {
    println!("Liquidity Pool Model Example");

    let mut pools: Vec<LpPool> = Vec::new();

    // Example usage
    let price = Price(1_50000);
    let min_fee = Percentage(0_10000);
    let max_fee = Percentage(9 * SCALING_FACTOR);
    let liquidity_target = TokenAmount(90 * SCALING_FACTOR);

    let pool_0 = LpPool::init(price, min_fee, max_fee, liquidity_target);
    match pool_0 {
        Ok(pool) => {
            pools.push(pool);
        }
        Err(Errors::ZeroValue) => println!("Failed to initialize pool: Zero value provided."),
        Err(Errors::InvalidFees) => println!("Failed to initialize pool: Invalid fee structure."),
        _ => println!("Failed to initialize pool: Unknown error."),
    }

    // Add liquidity
    if let Some(pool) = pools.get_mut(0) {
        // Add Liquidity (100)
        match pool.add_liquidity(TokenAmount(100 * SCALING_FACTOR)) {
            Ok(lp_token) => println!("Added liquidity to pool: {:?}", lp_token),
            Err(Errors::ZeroValue) => {
                println!("Failed to add liquidity to the pool: Zero value provided.")
            }
            _ => println!("Failed to add liquidity to the pool: Unknown error."),
        }
    }

    // Swap
    if let Some(pool) = pools.get_mut(0) {
        // Swap (6)
        match pool.swap(StakedTokenAmount(6 * SCALING_FACTOR)) {
            Ok(st_token) => println!("Swap performed: {:?}", st_token),
            Err(Errors::ZeroValue) => println!("Failed to swap: Zero value provided."),
            Err(Errors::InsufficientLiquidity) => {
                println!("Failed to swap: Insufficient liquidity.")
            }
            _ => println!("Failed to swap: Unknown error."),
        }
    }

    // Add liquidity
    if let Some(pool) = pools.get_mut(0) {
        // Add Liquidity (10)
        match pool.add_liquidity(TokenAmount(10 * SCALING_FACTOR)) {
            Ok(lp_token) => println!("Added liquidity to pool: {:?}", lp_token),
            Err(Errors::ZeroValue) => {
                println!("Failed to add liquidity to the pool: Zero value provided.")
            }
            _ => println!("Failed to add liquidity to the pool: Unknown error."),
        }
    }

    // Swap
    if let Some(pool) = pools.get_mut(0) {
        // Swap (30)
        match pool.swap(StakedTokenAmount(30_00000)) {
            Ok(st_token) => println!("Swap performed: {:?}", st_token),
            Err(Errors::ZeroValue) => println!("Failed to swap: Zero value provided."),
            Err(Errors::InsufficientLiquidity) => {
                println!("Failed to swap: Insufficient liquidity.")
            }
            _ => println!("Failed to swap: Unknown error."),
        }
    }

    // Remove liquidity
    if let Some(pool) = pools.get_mut(0) {
        // Remove liquidity (109.9991)
        match pool.remove_liquidity(LpTokenAmount(109_99910)) {
            Ok(tokens) => println!("Removed Liquidity: {:?}", tokens),
            Err(Errors::ZeroValue) => println!("Failed to remove liquidity: Zero value provided."),
            Err(Errors::InsufficientLpTokens) => {
                println!("Failed to remove liquidity: Insufficient LP tokens.")
            }
            _ => println!("Failed to remove liquidity: Unknown error."),
        }
    }

    // Read each pool's data
    for (i, pool) in pools.iter().enumerate() {
        println!("Pool {}: {:?}", i, pool);
    }
}
