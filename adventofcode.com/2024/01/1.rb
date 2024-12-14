list_1 = []
list_2 = []

File.open(File.dirname(__FILE__) + "/input.txt", "r") do |f|
  f.each_line do |line|
    values = line.strip.split('   ')
    list_1.push(Integer(values[0]))
    list_2.push(Integer(values[1]))
  end
end

list_1 = list_1.sort
list_2 = list_2.sort

distance = 0
(0..list_1.length - 1).each do |i|
  distance += (list_1[i] - list_2[i]).abs
end

puts(distance)  # 1580061

