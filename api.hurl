GET http://127.0.0.1:8080/hello/jamon
HTTP 200
[Asserts]
header "Content-Type" == "text/plain; charset=utf-8"
body contains "Hello, jamon!"
duration < 50