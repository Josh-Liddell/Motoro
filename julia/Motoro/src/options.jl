"""
    VanillaOption

Abstract base type for all vanilla option contracts.

See also: [`EuropeanOption`](@ref), [`AmericanOption`](@ref)
"""
abstract type VanillaOption end


"""
    EuropeanOption <: VanillaOption

Abstract type for European-style options that can only be exercised at expiration.

See also: [`EuropeanCall`](@ref), [`EuropeanPut`](@ref)
"""
abstract type EuropeanOption <: VanillaOption end


"""
    AmericanOption <: VanillaOption

Abstract type for American-style options that can be exercised at any time up to
and including expiration.

See also: [`AmericanCall`](@ref), [`AmericanPut`](@ref)
"""
abstract type AmericanOption <: VanillaOption end

## European Options

"""
    EuropeanCall(strike, expiry)

A European call option that gives the holder the right (but not the obligation)
to buy the underlying asset at the strike price at expiration.

# Fields
- `strike::AbstractFloat`: Strike price (exercise price)
- `expiry::AbstractFloat`: Time to expiration in years

# Payoff
The payoff at expiration is `max(0, S - K)` where `S` is the spot price and `K`
is the strike.

# Examples
```julia
# At-the-money call with 1 year to expiration
call = EuropeanCall(100.0, 1.0)

# Out-of-the-money call with 6 months to expiration
call = EuropeanCall(110.0, 0.5)
```

See also: [`EuropeanPut`](@ref), [`payoff`](@ref)
"""
struct EuropeanCall <: EuropeanOption
    strike::AbstractFloat
    expiry::AbstractFloat
end

Base.broadcastable(x::EuropeanCall) = Ref(x)

function payoff(option::EuropeanCall, spot::AbstractFloat)
    return max(0.0, spot - option.strike)
end


"""
    EuropeanPut(strike, expiry)

A European put option that gives the holder the right (but not the obligation)
to sell the underlying asset at the strike price at expiration.

# Fields
- `strike::AbstractFloat`: Strike price (exercise price)
- `expiry::AbstractFloat`: Time to expiration in years

# Payoff
The payoff at expiration is `max(0, K - S)` where `S` is the spot price and `K`
is the strike.

# Examples
```julia
# At-the-money put with 1 year to expiration
put = EuropeanPut(100.0, 1.0)

# In-the-money put with 6 months to expiration
put = EuropeanPut(110.0, 0.5)
```

See also: [`EuropeanCall`](@ref), [`payoff`](@ref)
"""
struct EuropeanPut <: EuropeanOption
    strike::AbstractFloat
    expiry::AbstractFloat
end

Base.broadcastable(x::EuropeanPut) = Ref(x)

"""
    payoff(option::VanillaOption, spot)

Calculate the intrinsic value (payoff) of an option at a given spot price.

# Arguments
- `option::VanillaOption`: The option contract
- `spot`: Spot price(s) of the underlying asset (can be scalar or vector)

# Returns
The intrinsic value of the option. For calls: `max(0, S - K)`, for puts: `max(0, K - S)`.

# Examples
```julia
call = EuropeanCall(100.0, 1.0)
payoff(call, 110.0)  # Returns 10.0

put = EuropeanPut(100.0, 1.0)
payoff(put, 90.0)    # Returns 10.0
payoff(put, 110.0)   # Returns 0.0
```
"""
function payoff(option::EuropeanPut, spot::AbstractFloat)
    return max.(0.0, option.strike - spot)
end
