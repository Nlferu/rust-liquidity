# Simplified Liquidity Pool Model

**This pool model is based on Marinade Protocol**

The code enables basic operations such as initializing a pool, adding liquidity, removing liquidity, and swapping tokens within the pool.

## Features

- **Initialization:** Create a new liquidity pool with specific parameters.

- **Add Liquidity:** Add tokens to the pool and mint LP tokens in return.

- **Remove Liquidity:** Withdraw tokens from the pool by burning LP tokens.

- **Swap Tokens:** Swap staked tokens for regular tokens within the pool.

## Guidelines

- **Fixed-Point Decimals:** Uses of fixed-point decimals based on the u64 type for all parameters, avoiding floating points.

- **Constant Price:** Assumes a constant price for simplicity.

- **Unit Tests:** Includes unit tests for all functions.

## Structure

- **Structs:**

  - **Price:** Represents the price of the token.
  - **TokenAmount:** Represents the amount of tokens.
  - **StakedTokenAmount:** Represents the amount of staked tokens.
  - **LpTokenAmount:** Represents the amount of LP tokens.
  - **Percentage:** Represents a percentage value.

- **Errors:**

  - InvalidFee
  - InsufficientLiquidity
  - InsufficientLpTokens
  - ZeroValue
  - Other(String)

- **LpPool Struct**
  - **price:** Price of the token.
  - **token_amount:** Amount of tokens in the pool.
  - **st_token_amount:** Amount of staked tokens in the pool.
  - **lp_token_amount:** Amount of LP tokens in the pool.
  - **liquidity_target:** Target amount of liquidity for the pool.
  - **min_fee:** Minimum fee percentage.
  - **max_fee:** Maximum fee percentage.

## Methods

**`init`** - Initializes a new liquidity pool.

**`add_liquidity`** - Adds liquidity to the pool and mints LP tokens.

**`remove_liquidity`** - Removes liquidity from the pool and burns LP tokens.

**`swap`** - Swaps staked tokens for regular tokens within the pool.
