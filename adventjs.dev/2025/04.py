def decode_santa_pin(code: str) -> str | None:
    import re

    blocks = re.findall(r"\[([^\]]+)\]", code)
    if len(blocks) != 4:
        return None

    values = []
    for i, block in enumerate(blocks):
        raw_number = block[0]
        if raw_number == "<":
            current_number = values[i - 1]
        else:
            current_number = int(raw_number)

        for operation in block[1:]:
            offset = 1 if operation == "+" else -1
            current_number += offset

        values.append(current_number)

    return "".join([v % 10 for v in values])
