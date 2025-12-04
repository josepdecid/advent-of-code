def draw_gift(size, symbol):
    if size < 2:
        return ""

    border_line = symbol * size
    center_lines = [symbol + " " * (size - 2) + symbol] * (size - 2)

    return "\n".join([border_line, *center_lines, border_line])
