n = gets.to_i
as = gets.chomp
arr = as.split(' ').map(&:to_i)

mul = 1

if arr.any? { |e| e == 0 }
  puts 0
  return
end

arr.each do |i|
  if mul > 1_000_000_000_000_000_000
    puts -1
    return
  end

  mul *= i
end

if mul > 1_000_000_000_000_000_000
  puts -1
else
  puts mul
end
