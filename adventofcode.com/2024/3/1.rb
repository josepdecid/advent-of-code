result = 0

File.open(File.dirname(__FILE__) + "/input.txt", "r") do |f|
  f.each_line do |line|
    line.scan(/mul\(\d+,\d+\)/).each do |operation|
      values = operation[4..-2].split(',')
      result += Integer(values[0]) * Integer(values[1])
    end
  end
end

puts(result)

