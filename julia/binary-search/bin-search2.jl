tested_benus_tasks = (rev = true, by = true, lt = true, multiple_matches = true)

function binarysearch(haystack, needle, low = 1, high = length(haystack);
    by = identity,
    lt = <,
    rev = false)
    isempty(haystack) && return 1:0
    haystack = by.(haystack)
    lt = rev ? ifelse(lt == <, >, <) : lt
    needle = by(needle)

    lt(needle, haystack[low]) && return low:low - 1
    lt(haystack[high], needle) && return high + 1:high

    mid = (low + high) ÷ 2

    compare = haystack[mid]
    if lt(needle, compare)
        output = binarysearch(haystack, needle, low, mid - 1, lt = lt)
    elseif lt(compare, needle)
        output = binarysearch(haystack, needle, mid + 1, high, lt = lt)
    else
        output = mid:mid
    end
    output
end
