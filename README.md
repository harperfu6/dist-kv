## start server
```
$ cargo r 
```

## request by client
```
$ curl localhost:8080/hello/daiki
Hello, daiki!

$ curl -X POST -H "Content-Type: application/json" -d '{"name":"daiki", "age":"30"}' localhost:8080/data
{"age":"30","name":"daiki"}
```
