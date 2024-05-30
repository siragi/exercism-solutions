using Dates: Minute, value

struct Clock 
    m::Integer

    Clock(m) = new(mod(m, 60*24))     
end

function Clock(hh, mm)
    return Clock(hh*60+mm)    
end

function Base.:+(clock::Clock, minute::Minute)
    a_few_minutes_later = clock.m + value(minute)
    return Clock(a_few_minutes_later)
end 

Base.:-(clock::Clock, minute::Minute) = clock + (- minute)


function Base.show(io::IO, clock::Clock) 
    hh, mm = divrem(clock.m, 60)
    # "\"08:00\""
    print(io, '"',lpad(hh, 2, '0'),':', lpad(mm, 2, '0'), '"')
end
