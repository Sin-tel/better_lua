-- defines a factorial function
fact = function(n)
  if n == 0 then
    return 1
  else
    return n * fact(n - 1)
  end
end

return fact(6)
