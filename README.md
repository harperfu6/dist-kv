in-memory kv store

# start server
```
$ cargo r 
```

# request by client
## POST data
```
$ curl -X POST -H "Content-Type: application/json" -d '{"key": "value"}' localhost:8080
```
## GET data
```
$ curl localhost:8080/{key}
```

## example
```
# POST DATA
$ curl -X POST -H "Content-Type: application/json" -d '{"name":"daiki", "age":"30"}' localhost:8080
The specified key was successfully created.

# GET by key
$ curl localhost:8080/name
daiki
```
