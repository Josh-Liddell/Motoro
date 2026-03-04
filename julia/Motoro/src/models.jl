struct Binomial
    steps::Int
end

# function price(option::EuropeanOption, model::Binomial, data::MarketData)
#     u = exp((data.rate - data.div)*option.expiry) + data.vol * sqrt(option.expiry)
#     d = exp((data.rate - data.div)*option.expiry) - data.vol * sqrt(option.expiry)
#     Cu = payoff(option, data.spot*u)
#     Cd = payoff(option, data.spot*d)
#     D = exp(-data.div * option.expiry) * ((Cu - Cd) / (u*data.spot - d*data.spot))
#     B = exp(-data.rate * option.expiry) * ((u*Cd - d*Cu) / (u - d))

#     return D*data.spot + B
#     # println(D*data.spot + B)
# end


function price(option::EuropeanOption, engine::Binomial, data::MarketData)
    (; strike, expiry) = option
    (; spot, rate, vol, div) = data
    steps = engine.steps

    dt = expiry / steps
    u = exp((rate - div) * dt + vol * sqrt(dt))
    d = exp((rate - div) * dt - vol * sqrt(dt))
    pu = (exp((rate - div) * dt) - d) / (u - d)
    pd = 1 - pu
    disc = exp(-rate * dt)

    s = zeros(steps + 1)
    x = zeros(steps + 1)

    @inbounds for i in 1:steps+1
        s[i] = spot * u^(steps + 1 - i) * d^(i - 1)
        x[i] = payoff(option, s[i])
    end

    for j in steps:-1:1
        @inbounds for i in 1:j
            x[i] = disc * (pu * x[i] + pd * x[i + 1])
        end
    end

    return x[1]
end
