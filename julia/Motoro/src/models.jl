struct Binomial
    steps::Int
end

function price(option::EuropeanOption, model::Binomial, data::MarketData)
    u = exp((data.rate - data.div)*option.expiry) + data.vol * sqrt(option.expiry)
    d = exp((data.rate - data.div)*option.expiry) - data.vol * sqrt(option.expiry)
    Cu = payoff(option, data.spot*u)
    Cd = payoff(option, data.spot*d)
    D = exp(-data.div * option.expiry) * ((Cu - Cd) / (u*data.spot - d*data.spot))
    B = exp(-data.rate * option.expiry) * ((u*Cd - d*Cu) / (u - d))

    return D*data.spot + B
    # println(D*data.spot + B)
end
