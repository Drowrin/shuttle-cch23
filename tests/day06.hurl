POST http://localhost:8000/6
```
The mischievous elf peeked out from behind the toy workshop,
and another elf joined in the festive dance.
Look, there is also an elf on that shelf!
```
HTTP 200
[Asserts]
jsonpath "$.elf" == 4

POST http://localhost:8000/6 
```
there is an elf on a shelf on an elf.
there is also another shelf in Belfast.
```
HTTP 200
[Asserts]
jsonpath "$.elf" == 5
jsonpath "$.['elf on a shelf']" == 1
jsonpath "$.['shelf with no elf on it']" == 1
