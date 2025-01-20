from enum import Enum

class HttpMethod(Enum):
    GET = 1
    POST = "POST"

print(type(str(HttpMethod.GET)))
print(HttpMethod.GET.name)

print(HttpMethod.GET.value)