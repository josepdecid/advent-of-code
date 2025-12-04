def manufacture_gifts(gifts_to_produce):
    gifts = []
    for gift in gifts_to_produce:
        quantity, toy = gift["quantity"], gift["toy"]
        if isinstance(quantity, int) and quantity > 0:
            gifts.extend([toy] * quantity)

    return gifts
