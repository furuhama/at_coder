input_nums = gets.split.map(&:to_i)
gates = gets.split.map(&:to_i)

n = input_nums[0]
x = input_nums[2]

to_left = 0
(0..x).each do |num|
  to_left += 1 if gates.include?(num)
end

to_right = 0
(x..n).each do |num|
  to_right += 1 if gates.include?(num)
end

if to_left < to_right
  puts to_left
else
  puts to_right
end
