// Use 18 decimals later on for more precision
const SCALING_FACTOR: u64 = 100_000;

#[derive(Debug)]
struct Price(u64);

#[derive(Debug)]
struct TokenAmount(u64);

#[derive(Debug)]
struct StakedTokenAmount(u64);

#[derive(Debug)]
struct LpTokenAmount(u64);

#[derive(Debug)]
struct Percentage(u64);

#[allow(dead_code)]
#[derive(Debug)]
struct LpPool {
    price: Price,
    token_amount: TokenAmount,
    st_token_amount: StakedTokenAmount,
    lp_token_amount: LpTokenAmount,
    liquidity_target: TokenAmount,
    min_fee: Percentage,
    max_fee: Percentage,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Errors {
    InvalidFee,
    InsufficientLiquidity,
    InsufficientLpTokens,
    ZeroValue,
    Other(String),
}

impl LpPool {
    #[allow(dead_code)]
    pub fn init(
        price: Price,
        min_fee: Percentage,
        max_fee: Percentage,
        liquidity_target: TokenAmount,
    ) -> Result<Self, Errors> {
        // TODO:
        // State change - Updates all LpPool vars
        // Returns - Instance of LpPool

        if price.0 == 0 || liquidity_target.0 == 0 {
            return Err(Errors::ZeroValue);
        }

        Ok(Self {
            price,
            token_amount: TokenAmount(0),
            st_token_amount: StakedTokenAmount(0),
            lp_token_amount: LpTokenAmount(0),
            liquidity_target,
            min_fee,
            max_fee,
        })
    }

    #[allow(dead_code)]
    pub fn add_liquidity(
        self: &mut Self,
        token_amount: TokenAmount,
    ) -> Result<LpTokenAmount, Errors> {
        // TODO:
        // State change - Increases the Token reserve and the amount of LpToken
        // Returns - New amount of minted LpToken

        let minted_lp_token_amount: u64 = if self.lp_token_amount.0 == 0 {
            token_amount.0
        } else {
            let total_token_value =
                self.token_amount.0 + (self.st_token_amount.0 * self.price.0 / SCALING_FACTOR);

            let lp_price = total_token_value * SCALING_FACTOR / self.lp_token_amount.0;

            let new_lp_token_to_mint_amount = token_amount.0 * SCALING_FACTOR / lp_price;

            println!("Value: {}", new_lp_token_to_mint_amount);

            new_lp_token_to_mint_amount
        };

        self.token_amount.0 += token_amount.0;
        self.lp_token_amount.0 += minted_lp_token_amount;

        Ok(LpTokenAmount(minted_lp_token_amount))
    }

    #[allow(dead_code)]
    pub fn remove_liquidity(
        self: &mut Self,
        lp_token_amount: LpTokenAmount,
    ) -> Result<(TokenAmount, StakedTokenAmount), Errors> {
        // TODO:
        // State change - Decreases Token reserve, decreases StakedToken reserve, and decreases the amount of LpToken
        // Returns - Specific amounts of Token and StakedToken. The amount of returned tokens is proportional to the LpToken passed,
        //           considering all LpTokens minted by the LpPool

        if lp_token_amount.0 > self.lp_token_amount.0 {
            return Err(Errors::InsufficientLiquidity);
        }

        let token_amount_to_return =
            (lp_token_amount.0 * self.token_amount.0) / self.lp_token_amount.0;
        let staked_token_amount_to_return =
            (lp_token_amount.0 * self.st_token_amount.0) / self.lp_token_amount.0;

        self.token_amount.0 -= token_amount_to_return;
        self.st_token_amount.0 -= staked_token_amount_to_return;
        self.lp_token_amount.0 -= lp_token_amount.0;

        Ok((
            TokenAmount(token_amount_to_return),
            StakedTokenAmount(staked_token_amount_to_return),
        ))
    }

    #[allow(dead_code)]
    pub fn swap(
        self: &mut Self,
        staked_token_amount: StakedTokenAmount,
    ) -> Result<TokenAmount, Errors> {
        // TODO:
        // State change - Decreases Token reserve and increases StakedToken reserve in the LpPool
        // Returns -  Amount of Token received as a result of the exchange.
        //            The received token amount depends on the StakedToken passed during invocation and the fee charged by the LpPool.

        let total_amount = staked_token_amount.0 * self.price.0 / SCALING_FACTOR;

        if total_amount > self.token_amount.0 {
            return Err(Errors::InsufficientLiquidity);
        }

        let amount_after = self.token_amount.0 - total_amount;
        println!("Amount After: {}", amount_after);

        let mut fee = self.min_fee.0;

        if amount_after < self.liquidity_target.0 {
            let fee_difference = self.max_fee.0 - self.min_fee.0;

            fee = self.max_fee.0 - fee_difference * amount_after / self.liquidity_target.0;
            // fee = 346138
        }

        println!("Fee Used For Calculation: {}", fee);

        // This is our net amount
        let net_token_amount =
            (total_amount * (100 * SCALING_FACTOR - fee)) / (100 * SCALING_FACTOR);

        let fee_amount = total_amount - net_token_amount;
        println!("Charged Fee: {}", fee_amount);

        println!("Net Token Amount: {}", net_token_amount);

        self.token_amount.0 -= net_token_amount;
        self.st_token_amount.0 += staked_token_amount.0;

        println!("Reserve after swap: {}", self.token_amount.0);
        println!("Current LP Tokens: {}", self.lp_token_amount.0);

        Ok(TokenAmount(net_token_amount))
    }

    fn _calculate_fee(&self, amount: u64) -> u64 {
        let mut fee = self.min_fee.0;

        let amount_after = self.token_amount.0 - amount;
        println!("Amount After: {}", amount_after);

        if amount_after < self.liquidity_target.0 {
            let fee_difference = self.max_fee.0 - self.min_fee.0;

            fee = self.max_fee.0 - fee_difference * amount_after / self.liquidity_target.0;
            // fee = 346138
        }

        println!("Fee Used For Calculation: {}", fee);

        (amount * fee) / 100
    }
}

fn main() {
    println!("Liquidity protocol!");

    // Above will be needed for interface information

    let mut pools: Vec<LpPool> = Vec::new();

    // Example usage
    let price = Price(150_000);
    let min_fee = Percentage(10_000);
    let max_fee = Percentage(9 * SCALING_FACTOR);
    let liquidity_target = TokenAmount(90 * SCALING_FACTOR);

    let pool_0 = LpPool::init(price, min_fee, max_fee, liquidity_target);
    match pool_0 {
        Ok(pool) => {
            pools.push(pool);
        }
        Err(e) => println!("Failed to initialize pool 0: {:?}", e),
    }

    // Add liquidity to the first pool
    if let Some(pool) = pools.get_mut(0) {
        // Add Liquidity (100)
        match pool.add_liquidity(TokenAmount(100 * SCALING_FACTOR)) {
            Ok(lp_token) => println!("Added liquidity to pool 0: {:?}", lp_token),
            Err(e) => println!("Failed to add liquidity to pool 0: {:?}", e),
        }
    }

    // Swap to the first pool
    if let Some(pool) = pools.get_mut(0) {
        // Swap (6)
        match pool.swap(StakedTokenAmount(6 * SCALING_FACTOR)) {
            Ok(st_token) => println!("Swap performed: {:?}", st_token),
            Err(e) => println!("Failed to add liquidity to pool 0: {:?}", e),
        }
    }

    if let Some(pool) = pools.get_mut(0) {
        // Add Liquidity (10)
        match pool.add_liquidity(TokenAmount(10 * SCALING_FACTOR)) {
            Ok(lp_token) => println!("Added liquidity to pool 0: {:?}", lp_token),
            Err(e) => println!("Failed to add liquidity to pool 0: {:?}", e),
        }
    }

    if let Some(pool) = pools.get_mut(0) {
        // Swap (30)
        match pool.swap(StakedTokenAmount(3000_000)) {
            Ok(st_token) => println!("Swap performed: {:?}", st_token),
            Err(e) => println!("Failed to add liquidity to pool 0: {:?}", e),
        }
    }

    if let Some(pool) = pools.get_mut(0) {
        // Remove (109.9991)
        match pool.remove_liquidity(LpTokenAmount(10999910)) {
            Ok(tokens) => println!("Removed Liquidity: {:?}", tokens),
            Err(e) => println!("Failed to add liquidity to pool 0: {:?}", e),
        }
    }

    // Initialize the second pool
    // let price2 = Price(200_000);
    // let min_fee2 = Percentage(20_000);
    // let max_fee2 = Percentage(950_000);
    // let liquidity_target2 = TokenAmount(10_000_000);

    // let pool_1 = LpPool::init(price2, min_fee2, max_fee2, liquidity_target2);
    // match pool_1 {
    //     Ok(pool) => {
    //         pools.push(pool);
    //     }
    //     Err(e) => println!("Failed to initialize pool 0: {:?}", e),
    // }

    // Access each pool's data
    for (i, pool) in pools.iter().enumerate() {
        println!("Pool {}: {:?}", i, pool);
    }
}

// Marinade -> liquid staking protocol on the Solana. It allows to divide your stake among many validators instead of just 1 of top 32.
// mSOL -> liquidity token that represents staked SOL (It is pegged to the value of SOL)
// EPOCH -> 2 days

// Requirements:
// ● Use fixed-point decimals based on the u64 type for all of these parameters, instead of floating points.
// ● Assume that the price is constant for simplicity.
// ● Implement a math model in pure Rust; integration with blockchain or UI is not necessary.
// ● Include unit tests for at least the most important functions.
// ● Choose any implementation paradigm (such as OOP, functional programming, etc.) based on your preferences.
