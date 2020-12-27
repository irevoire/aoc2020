local iterator = {}  -- create a table to represent the module

function iterator.get_lines_from_args(n)
	if #arg < n then
		return nil
	end
	io.input(arg[n])

	return function()
		return io.read()
	end
end

function iterator.enumerate(iter)
	local i = 0

	return function()
		i = i + 1
		local tmp = {iter()}
		return tmp[1] and i, unpack(tmp)
	end
end

function iterator.map(iter, fn)
	return function ()
		local tmp = {iter()}
		if tmp[1] then return fn(unpack(tmp)) else return nil end
	end
end

function iterator.cartesian_product(a, b)
	-- I can't clone an iterator / a function?
	local b = iterator.icollect(b)

	return
	iterator.map(
	a,
	function (a)
		local b = iterator.iiter(b)
		return iterator.map(
		b,
		function (b)
			return a, b
		end)
	end)
end

function iterator.flatten(iter)
	local current = iter()

	return function()
		-- the main iterator
		current = current or iter()
		if current == nil then
			return nil
		end
		-- the sub iterator
		local res = {current()}
		if res[1] == nil then
			current = iter()
			if current == nil then
				return nil
			else
				return current()
			end
		else
			return unpack(res)
		end
	end
end

function iterator.find(iter, pred)
	local iter = iterator.map(iter, function(...) return {...} end)
	for arg in iter do
		if pred(unpack(arg)) then return unpack(arg) end
	end
	return nil
end

function iterator.icollect(iter)
	local res = {}

	for i, el in iterator.enumerate(iter) do
		res[i] = el
	end

	return res
end

function iterator.iiter(table)
	local i = 0
	return function()
		i = i + 1
		return table[i]
	end
end

-- load everything in the global scope
function open()
	for n,v in pairs(iterator) do _G[n] = v end
end

return open
