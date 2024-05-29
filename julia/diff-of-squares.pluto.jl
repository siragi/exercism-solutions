### A Pluto.jl notebook ###
# v0.19.42

using Markdown
using InteractiveUtils

# ╔═╡ 20d45d9f-49ed-4dbd-a390-1d7ad8f8049b
md"""
# Difference of Squares

Welcome to Difference of Squares on Exercism's Julia Track.
If you need help running the tests or submitting your code, check out `HELP.md`.

## Instructions

Find the difference between the square of the sum and the sum of the squares of the first N natural numbers.

The square of the sum of the first ten natural numbers is
(1 + 2 + ... + 10)² = 55² = 3025.

The sum of the squares of the first ten natural numbers is
1² + 2² + ... + 10² = 385.

Hence the difference between the square of the sum of the first ten natural numbers and the sum of the squares of the first ten natural numbers is 3025 - 385 = 2640.

You are not expected to discover an efficient solution to this yourself from first principles; research is allowed, indeed, encouraged.
Finding the best algorithm for the problem is a key skill in software engineering.

## Source

### Created by

- @SaschaMann

### Contributed to by

- @bovine3dom
- @cmcaine

### Based on

Problem 6 at Project Euler - https://projecteuler.net/problem=6
"""

# ╔═╡ e1a5a259-6c8d-4dfc-9a7d-6b53ffae2f8b
"Square the sum of the first `n` positive integers"
function square_of_sum(n)
	
	#sum = 0
	#for i in range(1,stop=n)
    #	sum += i
    #end
 	#sum*sum

	#sum(1:n)^2

	#gauss_sum = (n^2 + n)/2 # Gauss sum for 1 + 2 + ... + n 
	# TypeError: in typeassert, expected Integer, got a value of type Float64
	# division operator / uses float. 
	#gauss_sum = (n^2 + n) ÷ 2
	#gauss_sum^2
	(n^2 + n)^2 ÷ 4
end

# ╔═╡ b0d20b14-3eb2-4ad9-a38b-f8bee8c5090e
"Sum the squares of the first `n` positive integers"
function sum_of_squares(n)
	#sum = 0
	#for i in 1:n
    #	sum += i^2
    #end
	#sum

	#sum(x -> x^2, 0:n)

	sum((1:n) .^ 2)
	#pyramidal_number = n*(n+1)*(2n+1)÷6 # n² + (n-1)² + ... (n-n)² 
	
end

# ╔═╡ d0a98c6f-5e64-4479-b784-0f320e515782
"Subtract the sum of squares from square of the sum of the first `n` positive ints"
function difference(n)
	#square_of_sum(n) - sum_of_squares(n)
	#n*(n + 1)*(n^2 + n)÷4 - n*(n + 1)*(2n+1)÷6 
	x = n*(n+1)
	y = (2n+1)
	
	#x*(x÷4 - y÷6) # doesn't work :-(
	#(x*x÷2 - x*y÷3)÷2  #works, unless x is not outside. 
	
	#use 6/6 to simplyfy equation
	x*(3x - 2y)÷12
	
end

# ╔═╡ 8651c048-9178-4b9c-936f-6c3bdf7a08cb
begin
using Test

#include("difference-of-squares.jl")

@testset verbose = true "tests" begin
    @testset "Square the sum of the numbers up to the given number" begin
        @test square_of_sum(1)::Integer == 1
        @test square_of_sum(5)::Integer == 225
        @test square_of_sum(10)::Integer == 3025
        @test square_of_sum(100)::Integer == 25502500
    end

    @testset "Sum the squares of the numbers up to the given number" begin
        @test sum_of_squares(1)::Integer == 1
        @test sum_of_squares(5)::Integer == 55
        @test sum_of_squares(10)::Integer == 385
        @test sum_of_squares(100)::Integer == 338350
    end

    @testset "Subtract sum of squares from square of sums" begin
        @test difference(0)::Integer == 0
        @test difference(1)::Integer == 0
        @test difference(5)::Integer == 170
        @test difference(10)::Integer == 2640
        @test difference(100)::Integer == 25164150
    end
end
end

# ╔═╡ 00000000-0000-0000-0000-000000000001
PLUTO_PROJECT_TOML_CONTENTS = """
[deps]
Test = "8dfed614-e22c-5e08-85e1-65c5234f0b40"
"""

# ╔═╡ 00000000-0000-0000-0000-000000000002
PLUTO_MANIFEST_TOML_CONTENTS = """
# This file is machine-generated - editing it directly is not advised

julia_version = "1.9.2"
manifest_format = "2.0"
project_hash = "71d91126b5a1fb1020e1098d9d492de2a4438fd2"

[[deps.Base64]]
uuid = "2a0f44e3-6c83-55bd-87e4-b1978d98bd5f"

[[deps.InteractiveUtils]]
deps = ["Markdown"]
uuid = "b77e0a4c-d291-57a0-90e8-8db25a27a240"

[[deps.Logging]]
uuid = "56ddb016-857b-54e1-b83d-db4d58db5568"

[[deps.Markdown]]
deps = ["Base64"]
uuid = "d6f4376e-aef5-505a-96c1-9c027394607a"

[[deps.Random]]
deps = ["SHA", "Serialization"]
uuid = "9a3f8284-a2c9-5f02-9a11-845980a1fd5c"

[[deps.SHA]]
uuid = "ea8e919c-243c-51af-8825-aaa63cd721ce"
version = "0.7.0"

[[deps.Serialization]]
uuid = "9e88b42a-f829-5b0c-bbe9-9e923198166b"

[[deps.Test]]
deps = ["InteractiveUtils", "Logging", "Random", "Serialization"]
uuid = "8dfed614-e22c-5e08-85e1-65c5234f0b40"
"""

# ╔═╡ Cell order:
# ╟─20d45d9f-49ed-4dbd-a390-1d7ad8f8049b
# ╠═e1a5a259-6c8d-4dfc-9a7d-6b53ffae2f8b
# ╠═b0d20b14-3eb2-4ad9-a38b-f8bee8c5090e
# ╠═d0a98c6f-5e64-4479-b784-0f320e515782
# ╟─8651c048-9178-4b9c-936f-6c3bdf7a08cb
# ╟─00000000-0000-0000-0000-000000000001
# ╟─00000000-0000-0000-0000-000000000002
