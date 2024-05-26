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
        // State change for all LpPool vars
        // Returns instance of LpPool

        if min_fee.0 > max_fee.0 {
            return Err(Errors::InvalidFee);
        } else {
            panic!("Aaaa");
        }
        // PROVIDE IMPLEMENTATION
    }

    pub fn add_liquidity(
        self: &mut Self,
        token_amount: TokenAmount,
    ) -> Result<LpTokenAmount, Errors> {
        // TODO:
        // State change for all LpPool vars
        // Returns instance of LpPool

        // PROVIDE IMPLEMENTATION
    }

    pub fn remove_liquidity(
        self: &mut Self,
        lp_token_amount: LpTokenAmount,
    ) -> Result<(TokenAmount, StakedTokenAmount), Errors> {
        // PROVIDE IMPLEMENTATION
    }

    pub fn swap(
        self: &mut Self,
        staked_token_amount: StakedTokenAmount,
    ) -> Result<TokenAmount, Errors> {
        // PROVIDE IMPLEMENTATION
    }
}

fn main() {
    println!("Hello, world!");
}

// Marinade -> liquid staking protocol on the Solana. It allows to divide your stake among many validators instead of just 1 of top 32.
// mSOL -> liquidity token that represents staked SOL (It is pegged to the value of SOL)
// EPOCH -> 2 days
