"Square the sum of the first `n` positive integers"
function square_of_sum(n)
    # imperative
    #sum = 0
	#for i in range(1,stop=n)
    #	sum += i
    #end
 	#sum*sum

    # mathy using the available sum over range function
	#sum(1:n)^2

    # Real Math
	#gauss_sum = (n^2 + n)/2 # Gauss sum for 1 + 2 + ... + n 
	# TypeError: in typeassert, expected Integer, got a value of type Float64
	# division operator / uses float. 
	gauss_sum = n*(n + 1) ÷ 2
	gauss_sum^2
end

"Sum the squares of the first `n` positive integers"
function sum_of_squares(n)
#sum = 0
	#for i in 1:n
    #	sum += i^2
    #end
	#sum

    # sum((1:n) .^ 2)
	#sum(x -> x^2, 0:n)


	pyramidal_number = n*(n+1)*(2n+1)÷6 # n² + (n-1)² + ... (n-n)² 
end

"Subtract the sum of squares from square of the sum of the first `n` positive ints"
function difference(n)
    #square_of_sum(n) - sum_of_squares(n)
	#n*(n+1) * n*(n+1) ÷ 4 - n*(n+1) * (2n+1) ÷6 
	x = n*(n+1)
	y = (2n+1)
	
	#x*(x÷4 - y÷6) # doesn't work :-(
	#(x*x÷2 - x*y÷3)÷2  #works, unless x is outside. 
	
	#use 6/6 to simplyfy equation
	x*(3x - 2y)÷12
end
