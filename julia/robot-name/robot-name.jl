# 2nd method with prefabrication of possible names and then shuffling it. Shamelessly stolen from Eric Schierboum
import Random: shuffle

names = [l1 * l2 * lpad(n, 3, '0') for l1 in 'A':'Z' for l2 in 'A':'Z' for n in 0:999] |> shuffle

mutable struct Robot
    name::String
end

Robot() = Robot(pop!(names))
function reset!(instance::Robot)
    # if length(names) > 1
    #     instance.name = pop!(names)
    # else
    #     error()
    # end

    # @assert length(names) > 1 "All possible combinations of naming convention used. No unique names left."
    # instance.name = pop!(names)

    try instance.name = pop!(names)
    catch e
        if e isa ArgumentError
            error("All possible combinations of naming convention used. No unique names left.")
        else
            throw(e)
        end
    end

end
name(instance::Robot) = instance.name

# 1. method
# using Random
# const robolist = Set{String}()
# mutable struct Robot
#     name::String
#     function Robot()
#         return new(generate_name())
#     end
# end
# function generate_name()
#     rng = MersenneTwister() # random number generator
#     name = randstring(rng, 'A':'Z', 2) * randstring(rng, '1':'9', 3)
#     name in robolist ? generate_name : push!(robolist, name); return name
# end
# function reset!(instance::Robot) = instance.name = generate_name()
# name(instance::Robot) = instance.name
