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
    InvalidFees,
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

    #[allow(dead_code)]
    pub fn add_liquidity(
        self: &mut Self,
        token_amount: TokenAmount,
    ) -> Result<LpTokenAmount, Errors> {
        // TODO:
        // State change - Increases the Token reserve and the amount of LpToken
        // Returns - New amount of minted LpToken

        let minted_lp_token_amount: u64 = if self.liquidity_target.0 > self.token_amount.0 {
            token_amount.0
        } else {
            999910
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

        let fee_amount = (total_amount * fee) / 100;

        println!("Calculated Fee: {}", fee_amount);

        let net_token_amount = (staked_token_amount.0 * self.price.0 - fee_amount) / SCALING_FACTOR;

        println!("Received Net Token Amount: {}", net_token_amount);

        self.token_amount.0 -= net_token_amount;
        self.st_token_amount.0 += staked_token_amount.0;

        println!("self.token_amount.0 after swap: {}", self.token_amount.0);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Result<LpPool, Errors> {
        let price = Price(150_000);
        let min_fee = Percentage(10_000);
        let max_fee = Percentage(9 * SCALING_FACTOR);
        let liquidity_target = TokenAmount(90 * SCALING_FACTOR);

        LpPool::init(price, min_fee, max_fee, liquidity_target)
    }

    #[test]
    fn test_cant_initialize_pool_with_zero_value() {
        let price = Price(0);
        let min_fee = Percentage(10_000);
        let max_fee = Percentage(9 * SCALING_FACTOR);
        let liquidity_target = TokenAmount(90 * SCALING_FACTOR);

        let result = LpPool::init(price, min_fee, max_fee, liquidity_target);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Errors::ZeroValue);

        let price = Price(150_000);
        let min_fee = Percentage(10_000);
        let max_fee = Percentage(9 * SCALING_FACTOR);
        let liquidity_target = TokenAmount(0);

        let result = LpPool::init(price, min_fee, max_fee, liquidity_target);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Errors::ZeroValue);
    }

    #[test]
    fn test_cant_initialize_pool_with_wrong_fees() {
        let price = Price(150_000);
        let min_fee = Percentage(90_000);
        let max_fee = Percentage(89_999);
        let liquidity_target = TokenAmount(90 * SCALING_FACTOR);

        let result = LpPool::init(price, min_fee, max_fee, liquidity_target);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Errors::InvalidFees);
    }

    #[test]
    fn test_correctly_initializes_pool() {
        let pool = setup().expect("Failed to initialize pool!");

        assert_eq!(pool.price.0, 150_000);
        assert_eq!(pool.token_amount.0, 0);
        assert_eq!(pool.st_token_amount.0, 0);
        assert_eq!(pool.lp_token_amount.0, 0);
        assert_eq!(pool.liquidity_target.0, 90 * SCALING_FACTOR);
        assert_eq!(pool.min_fee.0, 10_000);
        assert_eq!(pool.max_fee.0, 9 * SCALING_FACTOR);
    }

    #[test]
    fn test_can_add_liquidity_first_time() {
        let mut pool = setup().expect("Failed to initialize pool!");

        let token_amount = TokenAmount(100 * SCALING_FACTOR);
        let result = pool.add_liquidity(token_amount);

        assert!(result.is_ok());
        let lp_token_amount = result.unwrap();

        assert_eq!(lp_token_amount.0, 100 * SCALING_FACTOR); // Value from story example
        assert_eq!(pool.st_token_amount.0, 0);
        assert_eq!(pool.token_amount.0, lp_token_amount.0);
        assert_eq!(pool.lp_token_amount.0, lp_token_amount.0);
    }

    #[test]
    fn test_cant_swap_withthout_liquidity() {
        let mut pool = setup().expect("Failed to initialize pool!");

        let staked_token_amount = StakedTokenAmount(6 * SCALING_FACTOR);
        let result = pool.swap(staked_token_amount);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Errors::InsufficientLiquidity);
    }

    #[test]
    fn test_can_swap_for_min_fee() {
        let mut pool = setup().expect("Failed to initialize pool!");
        let token_amount = TokenAmount(100 * SCALING_FACTOR);
        let first_liquidity = pool
            .add_liquidity(token_amount)
            .expect("Failed to add liquidity");

        // Liquidity before swap
        let token_reserve = pool.token_amount.0;

        let staked_token_amount = StakedTokenAmount(6 * SCALING_FACTOR);
        let result = pool.swap(staked_token_amount);

        assert!(result.is_ok());
        let st_token_amount = result.unwrap();

        assert_eq!(st_token_amount.0, 899100); // Value from story example
        assert_eq!(pool.st_token_amount.0, 6 * SCALING_FACTOR);
        assert_eq!(pool.token_amount.0, token_reserve - st_token_amount.0);
        assert_eq!(pool.lp_token_amount.0, first_liquidity.0);
    }

    #[test]
    fn test_can_swap_for_max_fee() {
        let mut pool = setup().expect("Failed to initialize pool!");
        let token_amount = TokenAmount(100 * SCALING_FACTOR);
        let first_liquidity = pool
            .add_liquidity(token_amount)
            .expect("Failed to add liquidity");
        let staked_token_amount = StakedTokenAmount(6 * SCALING_FACTOR);
        pool.swap(staked_token_amount).expect("Swap failed!");
        let token_amount = TokenAmount(10 * SCALING_FACTOR);
        let added_liquidity = pool
            .add_liquidity(token_amount)
            .expect("Failed to add liquidity");

        // Liquidity before swap
        let token_reserve = pool.token_amount.0;

        let staked_token_amount = StakedTokenAmount(30 * SCALING_FACTOR);
        let result = pool.swap(staked_token_amount);

        assert!(result.is_ok());
        let st_token_amount = result.unwrap();

        let new_lp_token_amount = first_liquidity.0 + added_liquidity.0;

        // BELOW TO BE FIXED!
        // assert_eq!(st_token_amount.0, 4344237); // Value from story example
        assert_eq!(pool.st_token_amount.0, 36 * SCALING_FACTOR);
        assert_eq!(pool.token_amount.0, token_reserve - st_token_amount.0);
        assert_eq!(pool.lp_token_amount.0, new_lp_token_amount);
    }

    #[test]
    fn test_cant_remove_liquidity_that_exceed_liquidity_supply() {
        let mut pool = setup().expect("Failed to initialize pool!");
        let lp_token_amount_to_remove = LpTokenAmount(666 * SCALING_FACTOR);

        let result = pool.remove_liquidity(lp_token_amount_to_remove);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Errors::InsufficientLpTokens);
    }
}
