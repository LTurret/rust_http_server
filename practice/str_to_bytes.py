string_data: str = "hello world"
byte_data: bytes = string_data.encode("utf-8")

print(type(string_data), type(byte_data))