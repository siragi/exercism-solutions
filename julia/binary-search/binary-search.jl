# Change any of the following NamedTuple values to true to 
# enable testing different bonus tasks
tested_benus_tasks = (rev = true, by = true, lt = true, multiple_matches = true)


function binarysearch(ordered_list, search_element; rev = false, lt = <, by=identity)
    bt = (x, y) -> !lt(x,y) && x != y
    # keyword arguments (those args with default values after semicolon)
    cmp = rev ? bt : lt # compare function by default is lt = < (becausce rev(reverse) is by default false) 
                         # not lowerthan is '>=', thats why i have to write my own biggerthan.
    search_element = by(search_element)  # apply function called 'by'
    ordered_list = map(by, ordered_list) # apply function called 'by' upon every element
    
    len = length(ordered_list)
    len == 0 && return 1:0  # emptylist means: go ahead and insert at pos 1

    function helper(a,b) 
        # element below lower boundery ?
        cmp(search_element, ordered_list[a])  && return a:a-1
            # element not found => insert position:-1 indication (element not found) 
        # element above upper bounderiy?         
        cmp(ordered_list[b], search_element)  && return b+1:b 
            # element not found => insert position :-1 indication (element not found) 

        m = div(a+b,2)
        if  search_element == ordered_list[m]
            # to catch all elements before and after, that are equal to search element
            duplicates_before = 0
            while search_element == try ordered_list[m-1-duplicates_before] catch  end
                duplicates_before += 1
            end
            
            duplicates_after = 0
            while search_element == try ordered_list[m+1+duplicates_after] catch  end
                duplicates_after += 1
            end
            
            return m-duplicates_before: m+duplicates_after
        elseif cmp(search_element, ordered_list[m])
            helper(a,m-1)
        else
            helper(m+1,b)
        end
    end

    helper(1,len)
end
