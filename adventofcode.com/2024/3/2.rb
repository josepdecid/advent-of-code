result = 0
enabled = true

File.open(File.dirname(__FILE__) + "/input.txt", "r") do |f|
  f.each_line do |line|
    line.scan(/mul\(\d+,\d+\)|do\(\)|don't\(\)/).each do |operation|
      if operation == "do()"
        enabled = true
      elsif operation == "don't()"
        enabled = false
      else
        if enabled
          values = operation[4..-2].split(',')
          result += Integer(values[0]) * Integer(values[1])
        end
      end
    end
  end
end

puts(result)

