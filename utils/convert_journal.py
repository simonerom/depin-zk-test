# Example array with decimal values representing UTF-8 characters

example_array = [20,0,0,0,123,34,51,34,58,49,54,44,34,48,34,58,49,44,34,49,34,58,56,125]
# Convert the decimal values to characters and join them into a string
utf8_string = ''.join(chr(number) for number in example_array)

print(utf8_string)

