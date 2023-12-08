GET http://localhost:8000/8/weight/25
HTTP 200
[Asserts]
body == "6"

GET http://localhost:8000/8/drop/25
HTTP 200
[Asserts]
body == "84.10707461325713"
