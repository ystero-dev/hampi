
def count_bytes(num):
    count = 0
    for byte in num.to_bytes(16, byteorder='big', signed=True):
        if byte & 0xFF == 0xFF:
            continue
        if byte | 0x00 == 0:
            continue
        else:
            print(f"{byte:08b}")
            count += 1

    if count == 0:
        count += 1

    return count

print(count_bytes(-1))
print(count_bytes(0))
print(count_bytes(-128))
print(count_bytes(-129))
