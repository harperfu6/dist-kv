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

# request with Auth
## enable authentication
```
config.authentication.enabled = true; // uncomment if you want to enable authentication
```
## start server
```
$ cargo r
```
access token (JWT) is stored in ./config.yaml (as "root_token")

## POST data with JWT
"auth: " keyword is this library specific. (Generally used "Authorization: ")
```
$ curl -X POST -H "Content-Type: application/json" -H "auth: Bearer {JWT}" -d '{"key": "value"}' localhost:8080/api/kv
```


