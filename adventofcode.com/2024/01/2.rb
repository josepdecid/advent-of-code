list_1 = []
list_2 = []

File.open(File.dirname(__FILE__) + "/input.txt", "r") do |f|
  f.each_line do |line|
    values = line.strip.split('   ')
    list_1.push(Integer(values[0]))
    list_2.push(Integer(values[1]))
  end
end

list_2_counts = Hash.new(0)
list_2.each do |number|
  list_2_counts[number] += 1
end

score = 0
list_1.each do |number|
  score += number * list_2_counts[number]
end

puts(score)  # 23046913

