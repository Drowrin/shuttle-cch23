GET http://localhost:8000/7/decode
[Cookies]
recipe: eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==
HTTP 200
[Asserts]
body == "{\"flour\":100,\"chocolate chips\":20}"

GET http://localhost:8000/7/bake
[Cookies]
recipe: eyJyZWNpcGUiOnsiZmxvdXIiOjk1LCJzdWdhciI6NTAsImJ1dHRlciI6MzAsImJha2luZyBwb3dkZXIiOjEwLCJjaG9jb2xhdGUgY2hpcHMiOjUwfSwicGFudHJ5Ijp7ImZsb3VyIjozODUsInN1Z2FyIjo1MDcsImJ1dHRlciI6MjEyMiwiYmFraW5nIHBvd2RlciI6ODY1LCJjaG9jb2xhdGUgY2hpcHMiOjQ1N319
HTTP 200
[Asserts]
jsonpath "$.cookies" == 4
jsonpath "$.pantry.flour" == 5
jsonpath "$.pantry.sugar" == 307
jsonpath "$.pantry.butter" == 2002
jsonpath "$.pantry.['baking powder']" == 825
jsonpath "$.pantry.['chocolate chips']" == 257

GET http://localhost:8000/7/bake
[Cookies]
recipe: eyJyZWNpcGUiOnsic2xpbWUiOjl9LCJwYW50cnkiOnsiY29iYmxlc3RvbmUiOjY0LCJzdGljayI6IDR9fQ==
HTTP 200
[Asserts]
jsonpath "$.cookies" == 0
jsonpath "$.pantry.cobblestone" == 64
jsonpath "$.pantry.stick" == 4
