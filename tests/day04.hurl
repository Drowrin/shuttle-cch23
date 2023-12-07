POST http://localhost:8000/4/strength
Content-Type: application/json
[
  { "name": "Dasher", "strength": 5 },
  { "name": "Dancer", "strength": 6 },
  { "name": "Prancer", "strength": 4 },
  { "name": "Vixen", "strength": 7 }
]
HTTP 200
[Asserts]
body == "22"

POST http://localhost:8000/4/strength
Content-Type: application/json
[
  { "name": "Dasher", "strength": 3 },
  { "name": "Dancer", "strength": 2 },
  { "name": "Prancer", "strength": 9 },
  { "name": "Vixen", "strength": 1 }
]
HTTP 200
[Asserts]
body == "15"

POST http://localhost:8000/4/strength
Content-Type: application/json
[]
HTTP 200
[Asserts]
body == "0"

POST http://localhost:8000/4/contest
Content-Type: application/json
[
  {
    "name": "Dasher",
    "strength": 5,
    "speed": 50.4,
    "height": 80,
    "antler_width": 36,
    "snow_magic_power": 9001,
    "favorite_food": "hay",
    "cAnD13s_3ATeN-yesT3rdAy": 2
  },
  {
    "name": "Dancer",
    "strength": 6,
    "speed": 48.2,
    "height": 65,
    "antler_width": 37,
    "snow_magic_power": 4004,
    "favorite_food": "grass",
    "cAnD13s_3ATeN-yesT3rdAy": 5
  }
]
HTTP 200
[Asserts]
jsonpath "$.fastest" == "Speeding past the finish line with a strength of 5 is Dasher"
jsonpath "$.tallest" == "Dasher is standing tall with his 36 cm wide antlers"
jsonpath "$.magician" == "Dasher could blast you away with a snow magic power of 9001"
jsonpath "$.consumer" == "Dancer ate lots of candies, but also some grass"

POST http://localhost:8000/4/contest
Content-Type: application/json
[
  {
    "name": "Dasher",
    "strength": 5,
    "speed": 50.4,
    "height": 80,
    "antler_width": 36,
    "snow_magic_power": 9001,
    "favorite_food": "hay",
    "cAnD13s_3ATeN-yesT3rdAy": 2
  },
  {
    "name": "Dancer",
    "strength": 6,
    "speed": 48.2,
    "height": 65,
    "antler_width": 37,
    "snow_magic_power": 4004,
    "favorite_food": "grass",
    "cAnD13s_3ATeN-yesT3rdAy": 5
  },
  {
    "name": "Prancer",
    "strength": 9,
    "speed": 420.69,
    "height": 72,
    "antler_width": 35,
    "snow_magic_power": 69420,
    "favorite_food": "candies",
    "cAnD13s_3ATeN-yesT3rdAy": 3
  }
]
HTTP 200
[Asserts]
jsonpath "$.fastest" == "Speeding past the finish line with a strength of 9 is Prancer"
jsonpath "$.tallest" == "Dasher is standing tall with his 36 cm wide antlers"
jsonpath "$.magician" == "Prancer could blast you away with a snow magic power of 69420"
jsonpath "$.consumer" == "Dancer ate lots of candies, but also some grass"
