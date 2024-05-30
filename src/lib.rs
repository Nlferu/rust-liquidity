pub const SCALING_FACTOR: u64 = 100_000;

#[derive(Debug)]
pub struct Price(pub u64);

#[derive(Debug)]
pub struct TokenAmount(pub u64);

#[derive(Debug)]
pub struct StakedTokenAmount(pub u64);

#[derive(Debug)]
pub struct LpTokenAmount(pub u64);

#[derive(Debug)]
pub struct Percentage(pub u64);

#[derive(Debug)]
pub struct LpPool {
    pub price: Price,
    pub token_amount: TokenAmount,
    pub st_token_amount: StakedTokenAmount,
    pub lp_token_amount: LpTokenAmount,
    pub liquidity_target: TokenAmount,
    pub min_fee: Percentage,
    pub max_fee: Percentage,
}

#[derive(Debug, PartialEq)]
pub enum Errors {
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
        println!("Current Lp Tokens: {}", self.lp_token_amount.0);

        Ok(TokenAmount(net_token_amount))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Result<LpPool, Errors> {
        let price = Price(1_50000);
        let min_fee = Percentage(0_10000);
        let max_fee = Percentage(9 * SCALING_FACTOR);
        let liquidity_target = TokenAmount(90 * SCALING_FACTOR);

        LpPool::init(price, min_fee, max_fee, liquidity_target)
    }

    #[test]
    fn test_cant_initialize_pool_with_zero_value() {
        let price = Price(0);
        let min_fee = Percentage(0_10000);
        let max_fee = Percentage(9 * SCALING_FACTOR);
        let liquidity_target = TokenAmount(90 * SCALING_FACTOR);

        let result = LpPool::init(price, min_fee, max_fee, liquidity_target);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Errors::ZeroValue);

        let price = Price(1_50000);
        let min_fee = Percentage(0_10000);
        let max_fee = Percentage(9 * SCALING_FACTOR);
        let liquidity_target = TokenAmount(0);

        let result = LpPool::init(price, min_fee, max_fee, liquidity_target);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Errors::ZeroValue);
    }

    #[test]
    fn test_cant_initialize_pool_with_wrong_fees() {
        let price = Price(1_50000);
        let min_fee = Percentage(9_0000);
        let max_fee = Percentage(8_9999);
        let liquidity_target = TokenAmount(90 * SCALING_FACTOR);

        let result = LpPool::init(price, min_fee, max_fee, liquidity_target);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Errors::InvalidFees);
    }

    #[test]
    fn test_correctly_initializes_pool() {
        let pool = setup().expect("Failed to initialize pool!");

        assert_eq!(pool.price.0, 1_50000);
        assert_eq!(pool.token_amount.0, 0);
        assert_eq!(pool.st_token_amount.0, 0);
        assert_eq!(pool.lp_token_amount.0, 0);
        assert_eq!(pool.liquidity_target.0, 90 * SCALING_FACTOR);
        assert_eq!(pool.min_fee.0, 0_10000);
        assert_eq!(pool.max_fee.0, 9 * SCALING_FACTOR);
    }

    #[test]
    fn test_cant_add_zero_liquidity() {
        let mut pool = setup().expect("Failed to initialize pool!");

        let result = pool.add_liquidity(TokenAmount(0));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Errors::ZeroValue);
    }

    #[test]
    fn test_can_add_liquidity_first_time() {
        let mut pool = setup().expect("Failed to initialize pool!");

        let result = pool.add_liquidity(TokenAmount(100 * SCALING_FACTOR));

        assert!(result.is_ok());
        let lp_token_amount = result.unwrap();

        assert_eq!(lp_token_amount.0, 100 * SCALING_FACTOR); // Value from story example
        assert_eq!(pool.st_token_amount.0, 0);
        assert_eq!(pool.token_amount.0, lp_token_amount.0);
        assert_eq!(pool.lp_token_amount.0, lp_token_amount.0);
    }

    #[test]
    fn test_can_add_liquidity_with_fees_accumulated() {
        let mut pool = setup().expect("Failed to initialize pool!");

        let first_addition = pool
            .add_liquidity(TokenAmount(100 * SCALING_FACTOR))
            .expect("Failed to add liquidity");

        let swapped_tokens = pool
            .swap(StakedTokenAmount(6 * SCALING_FACTOR))
            .expect("Swap failed!");

        let result = pool.add_liquidity(TokenAmount(10 * SCALING_FACTOR));

        assert!(result.is_ok());
        let lp_token_amount = result.unwrap();

        assert_eq!(lp_token_amount.0, 9_99910); // Value from story example
        assert_eq!(pool.st_token_amount.0, 6 * SCALING_FACTOR);
        assert_eq!(
            pool.token_amount.0,
            first_addition.0 - swapped_tokens.0 + 10 * SCALING_FACTOR
        );
        assert_eq!(pool.lp_token_amount.0, first_addition.0 + lp_token_amount.0);
    }

    #[test]
    fn test_cant_swap_zero_and_withthout_liquidity() {
        let mut pool = setup().expect("Failed to initialize pool!");

        let result = pool.swap(StakedTokenAmount(0));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Errors::ZeroValue);

        let result = pool.swap(StakedTokenAmount(6 * SCALING_FACTOR));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Errors::InsufficientLiquidity);
    }

    #[test]
    fn test_can_swap_for_min_fee() {
        let mut pool = setup().expect("Failed to initialize pool!");

        let first_liquidity = pool
            .add_liquidity(TokenAmount(100 * SCALING_FACTOR))
            .expect("Failed to add liquidity");

        // Liquidity before swap
        let token_reserve = pool.token_amount.0;

        let result = pool.swap(StakedTokenAmount(6 * SCALING_FACTOR));

        assert!(result.is_ok());
        let st_token_amount = result.unwrap();

        assert_eq!(st_token_amount.0, 8_99100); // Value from story example
        assert_eq!(pool.st_token_amount.0, 6 * SCALING_FACTOR);
        assert_eq!(pool.token_amount.0, token_reserve - st_token_amount.0);
        assert_eq!(pool.lp_token_amount.0, first_liquidity.0);
    }

    #[test]
    fn test_can_swap_for_calculated_fee() {
        let mut pool = setup().expect("Failed to initialize pool!");

        let first_liquidity = pool
            .add_liquidity(TokenAmount(100 * SCALING_FACTOR))
            .expect("Failed to add liquidity");

        pool.swap(StakedTokenAmount(6 * SCALING_FACTOR))
            .expect("Swap failed!");

        let added_liquidity = pool
            .add_liquidity(TokenAmount(10 * SCALING_FACTOR))
            .expect("Failed to add liquidity");

        // Liquidity before swap
        let token_reserve = pool.token_amount.0;

        let result = pool.swap(StakedTokenAmount(30 * SCALING_FACTOR));

        assert!(result.is_ok());
        let st_token_amount = result.unwrap();

        let new_lp_token_amount = first_liquidity.0 + added_liquidity.0;

        assert_eq!(st_token_amount.0, 4344239); // Value from story example 4344237 (inaccuracy/rounding/scaling)
        assert_eq!(pool.st_token_amount.0, 36 * SCALING_FACTOR);
        assert_eq!(pool.token_amount.0, token_reserve - st_token_amount.0);
        assert_eq!(pool.lp_token_amount.0, new_lp_token_amount);
    }

    #[test]
    fn test_cant_remove_zero_and_liquidity_that_exceed_liquidity_supply() {
        let mut pool = setup().expect("Failed to initialize pool!");

        let result = pool.remove_liquidity(LpTokenAmount(0));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Errors::ZeroValue);

        let result = pool.remove_liquidity(LpTokenAmount(666 * SCALING_FACTOR));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Errors::InsufficientLpTokens);
    }

    #[test]
    fn test_can_remove_liquidity() {
        let mut pool = setup().expect("Failed to initialize pool!");

        pool.add_liquidity(TokenAmount(100 * SCALING_FACTOR))
            .expect("Failed to add liquidity");

        pool.swap(StakedTokenAmount(6 * SCALING_FACTOR))
            .expect("Swap failed!");

        pool.add_liquidity(TokenAmount(10 * SCALING_FACTOR))
            .expect("Failed to add liquidity");

        pool.swap(StakedTokenAmount(30 * SCALING_FACTOR))
            .expect("Swap failed!");

        // Removing all lp_tokens
        let lp_token_amount_to_remove = LpTokenAmount(pool.lp_token_amount.0);
        assert_eq!(lp_token_amount_to_remove.0, 109_99910); // Value from story example

        let result = pool.remove_liquidity(lp_token_amount_to_remove);

        assert!(result.is_ok());
        let (token_amount, st_token_amount) = result.unwrap();

        assert_eq!(token_amount.0, 57_56661); // Value from story example 5756663 (inaccuracy/rounding/scaling)
        assert_eq!(st_token_amount.0, 36 * SCALING_FACTOR); // Value from story example
        assert_eq!(pool.st_token_amount.0, 0);
        assert_eq!(pool.token_amount.0, 0);
        assert_eq!(pool.lp_token_amount.0, 0);
    }
}
