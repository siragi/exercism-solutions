import Base: isempty, length, iterate, push!, intersect!, intersect, union!, union, +, delete!, copy
# sorted Set for fun
mutable struct CustomSet{T} <: AbstractSet{T} 
    data::Vector{T}

    function CustomSet(list::Vector{T}) where T # same as where T<:Any
        new{T}(sort(unique(list))) # constructor using list requires iterate(), which is handy for sort and unique too.
    end
end

isempty(cs::CustomSet) = isempty(cs.data) # required for constructor with empty list

length(cs::CustomSet) = length(cs.data) # required for iterate (which would mean, that itr are not lazy i guess?)

# iterate(cs::CustomSet) = iterate(cs.data) # obsolete when index=1 (defaulted)
iterate(cs::CustomSet, index=1) = iterate(cs.data, index) # enables constructor with list, "in", "issubset" and more...

# gives back position of element (at, true) or where to insert (insert_at, false) 
function binsearch(cs::CustomSet, element)
    a = 1
    z = length(cs.data)
    m = (a+z) ÷ 2
    while a <= z
        element < cs.data[m] ? z = m - 1 :
        element > cs.data[m] ? a = m + 1 :
        return (m, true)  #  element == cs.data[m]
        m = (a+z) ÷ 2
    end
    return (m+1, false)
end

# push!(cs::CustomSet, element) = element in cs.data ? cs.data : push!(cs.data, element)
function push!(cs::CustomSet, element)
    (pos, exists) = binsearch(cs, element)
    exists || insert!(cs.data, pos, element)
    return cs.data
end

# not necessary for this exercise, but 'common'!
function delete!(cs::CustomSet, element)
    (m, exists) = binsearch(cs, element)
    exists && deleteat!(cs.data, m)
    return cs
end


intersect!(cs1::CustomSet, cs2::CustomSet) = (intersect!(cs1.data, cs2.data) ; return cs1)
intersect(cs1::CustomSet, cs2::CustomSet) = CustomSet(intersect(cs1.data, cs2.data))

complement!(cs1::CustomSet, cs2::CustomSet) = (setdiff!(cs1.data, cs2.data) ; return cs1)
complement(cs1::CustomSet, cs2::CustomSet) = CustomSet(setdiff(cs1.data, cs2.data))

# union!(cs1::CustomSet, cs2::CustomSet) = (union!(cs1.data, cs2.data) ; return cs1)
# union(cs1::CustomSet, cs2::CustomSet) = CustomSet(union(cs1.data, cs2.data))

disjoint(cs1::CustomSet, cs2::CustomSet) = isdisjoint(cs1.data, cs2.data)

copy(cs::CustomSet) = CustomSet(cs.data)