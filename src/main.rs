struct TokenAmount(u64);
struct StakedTokenAmount(u64);
struct LpTokenAmount(u64);
struct Price(u64);
struct Percentage(u64);

struct LpPool {
    price: Price,
    token_amount: TokenAmount,
    st_token_amount: StakedTokenAmount,
    lp_token_amount: LpTokenAmount,
    liquidity_target: TokenAmount,
    min_fee: Percentage,
    max_fee: Percentage,
}

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
        // State change - All LpPool vars
        // Returns - Instance of LpPool
    }

    pub fn add_liquidity(
        self: &mut Self,
        token_amount: TokenAmount,
    ) -> Result<LpTokenAmount, Errors> {
        // TODO:
        // State change - Increases the Token reserve and the amount of LpToken
        // Returns - New amount of minted LpToken
    }

    pub fn remove_liquidity(
        self: &mut Self,
        lp_token_amount: LpTokenAmount,
    ) -> Result<(TokenAmount, StakedTokenAmount), Errors> {
        // TODO:
        // State change - Decreases Token reserve, decreases StakedToken reserve, and decreases the amount of LpToken
        // Returns - Specific amounts of Token and StakedToken. The amount of returned tokens is proportional to the LpToken passed,
        //           considering all LpTokens minted by the LpPool
    }

    pub fn swap(
        self: &mut Self,
        staked_token_amount: StakedTokenAmount,
    ) -> Result<TokenAmount, Errors> {
        // TODO:
        // State change - Decreases Token reserve and increases StakedToken reserve in the LpPool
        // Returns -  Amount of Token received as a result of the exchange.
        //            The received token amount depends on the StakedToken passed during invocation and the fee charged by the LpPool.
    }
}

fn main() {
    println!("Liquidity protocol!");
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
