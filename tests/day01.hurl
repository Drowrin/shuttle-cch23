GET http://localhost:8000/1/4/8
HTTP 200
[Asserts]
body == "1728"

GET http://localhost:8000/1/10
HTTP 200
[Asserts]
body == "1000"

GET http://localhost:8000/1/4/5/8/10
HTTP 200
[Asserts]
body == "27"

GET http://localhost:8000/1/1/2/3/4/5/6
HTTP 200
[Asserts]
body == "343"
