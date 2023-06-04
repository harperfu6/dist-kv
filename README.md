![dist-kv](https://github.com/harperfu6/dist-kv/actions/workflows/rust.yml/badge.svg)

in-memory kv store

# TODO
- [x] auth
- [ ] CLI対応
- [ ] WebUI
- [ ] KVSの残りの機能（削除とか）

# start server
```
$ cargo r 
```

# request by client
## POST data
```
$ curl -X POST -H "Content-Type: application/json" -d '{"key": "value"}' localhost:8080/api/kv
```
## GET data
```
$ curl localhost:8080/api/kv/{key}
```

## health check
```
$ curl localhost:8080/health
OK
```

## example
```
# POST DATA
$ curl -X POST -H "Content-Type: application/json" -d '{"name":"daiki", "age":"30"}' localhost:8080/api/kv
The specified key was successfully created.

# GET by key
$ curl localhost:8080/api/kv/name
daiki
```
