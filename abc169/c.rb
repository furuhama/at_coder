ab = gets.chomp
a, b = ab.split(' ')
a = a.to_i
b_i, b_f = b.split('.')

b = b_i.to_i * 100 + b_f.to_i

r = a * b / 100
puts r
