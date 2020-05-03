haves = []
n = 0

line = gets
line.chomp!
n, k = line.split(' ')
k.to_i.times do
  d = gets
  d.chomp!
  as = gets
  as = as.chomp!.split(' ').map(&:to_i)

  haves = haves | as
end

puts n.to_i - haves.length
