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

#[derive(Debug)]
enum Errors {
    InvalidFee,
    InsufficientLiquidity,
    InsufficientLpTokens,
    Other(String),
}

impl LpPool {
    pub fn init(
        price: Price,
        min_fee: Percentage,
        max_fee: Percentage,
        liquidity_target: TokenAmount,
    ) -> Result<Self, Errors> {
        // TODO:
        // State change - Updates all LpPool vars
        // Returns - Instance of LpPool

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
        // TODO:
        // State change - Increases the Token reserve and the amount of LpToken
        // Returns - New amount of minted LpToken

        self.token_amount = token_amount;

        Ok(LpTokenAmount(0))
    }

    pub fn remove_liquidity(
        self: &mut Self,
        lp_token_amount: LpTokenAmount,
    ) -> Result<(TokenAmount, StakedTokenAmount), Errors> {
        // TODO:
        // State change - Decreases Token reserve, decreases StakedToken reserve, and decreases the amount of LpToken
        // Returns - Specific amounts of Token and StakedToken. The amount of returned tokens is proportional to the LpToken passed,
        //           considering all LpTokens minted by the LpPool

        Ok((TokenAmount(0), StakedTokenAmount(0)))
    }

    pub fn swap(
        self: &mut Self,
        staked_token_amount: StakedTokenAmount,
    ) -> Result<TokenAmount, Errors> {
        // TODO:
        // State change - Decreases Token reserve and increases StakedToken reserve in the LpPool
        // Returns -  Amount of Token received as a result of the exchange.
        //            The received token amount depends on the StakedToken passed during invocation and the fee charged by the LpPool.

        Ok(TokenAmount(0))
    }
}

fn main() {
    println!("Liquidity protocol!");

    // Example usage
    let price = Price(15);
    let min_fee = Percentage(1);
    let max_fee = Percentage(9);
    let liquidity_target = TokenAmount(90);

    match LpPool::init(price, min_fee, max_fee, liquidity_target) {
        Ok(lpPool) => println!("Initialized: {:?}", lpPool),
        Err(e) => println!("Failed to initialize: {:?}", e),
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
