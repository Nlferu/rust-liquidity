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

#[derive(Debug, PartialEq)]
enum Errors {
    InvalidFees,
    InsufficientLiquidity,
    InsufficientLpTokens,
    ZeroValue,
}

impl LpPool {
    pub fn init(
        price: Price,
        min_fee: Percentage,
        max_fee: Percentage,
        liquidity_target: TokenAmount,
    ) -> Result<Self, Errors> {
        // State change - Updates all LpPool vars
        // Returns - Instance of LpPool

        if price.0 == 0 || liquidity_target.0 == 0 {
            return Err(Errors::ZeroValue);
        }

        if max_fee.0 < min_fee.0 {
            return Err(Errors::InvalidFees);
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

    pub fn add_liquidity(
        self: &mut Self,
        token_amount: TokenAmount,
    ) -> Result<LpTokenAmount, Errors> {
        // State change - Increases the Token reserve and the amount of LpToken
        // Returns - New amount of minted LpToken

        if token_amount.0 == 0 {
            return Err(Errors::ZeroValue);
        }

        let minted_lp_token_amount: u64 = if self.lp_token_amount.0 == 0 {
            token_amount.0
        } else {
            let total_token_value =
                self.token_amount.0 + (self.st_token_amount.0 * self.price.0 / SCALING_FACTOR);

            let lp_price = total_token_value * SCALING_FACTOR / self.lp_token_amount.0;

            let new_lp_token_to_mint_amount = token_amount.0 * SCALING_FACTOR / lp_price;

            println!("Minted Lp Token Amount: {}", new_lp_token_to_mint_amount);

            new_lp_token_to_mint_amount
        };

        self.token_amount.0 += token_amount.0;
        self.lp_token_amount.0 += minted_lp_token_amount;

        Ok(LpTokenAmount(minted_lp_token_amount))
    }

    pub fn remove_liquidity(
        self: &mut Self,
        lp_token_amount: LpTokenAmount,
    ) -> Result<(TokenAmount, StakedTokenAmount), Errors> {
        // State change - Decreases Token reserve, decreases StakedToken reserve, and decreases the amount of LpToken
        // Returns - Specific amounts of Token and StakedToken. The amount of returned tokens is proportional to the LpToken passed,
        //           considering all LpTokens minted by the LpPool

        if lp_token_amount.0 == 0 {
            return Err(Errors::ZeroValue);
        }

        if lp_token_amount.0 > self.lp_token_amount.0 {
            return Err(Errors::InsufficientLpTokens);
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

    pub fn swap(
        self: &mut Self,
        staked_token_amount: StakedTokenAmount,
    ) -> Result<TokenAmount, Errors> {
        // State change - Decreases Token reserve and increases StakedToken reserve in the LpPool
        // Returns -  Amount of Token received as a result of the exchange.
        //            The received token amount depends on the StakedToken passed during invocation and the fee charged by the LpPool.

        if staked_token_amount.0 == 0 {
            return Err(Errors::ZeroValue);
        }

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
        }

        println!("Fee Used For Calculation: {}", fee);

        let net_token_amount =
            (total_amount * (100 * SCALING_FACTOR - fee)) / (100 * SCALING_FACTOR);

        self.token_amount.0 -= net_token_amount;
        self.st_token_amount.0 += staked_token_amount.0;

        println!("Current Tokens: {}", self.token_amount.0);
        println!("Current LP Tokens: {}", self.lp_token_amount.0);

        Ok(TokenAmount(net_token_amount))
    }
}

fn main() {
    println!("Liquidity protocol!");

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
