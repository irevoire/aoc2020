require"lib"()

local input = icollect(get_lines_from_args(1))
local a, b = find(flatten(cartesian_product(iiter(input), iiter(input))), function(a, b) return a + b == 2020 end)

print(a * b)
