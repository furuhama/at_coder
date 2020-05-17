k = gets.to_i
s = gets.chomp

if s.length <= k
  puts s
else
  puts "#{s[0..k-1]}..."
end
