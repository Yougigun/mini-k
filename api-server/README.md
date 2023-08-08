# api-server

The api server will server all the request from internet

## spec

### base url

All URLs referenced in the API have the following base: `https://api.put_your_own_domain.com/`

## Authentication

header: X-Api-Key: xxx

### endpoints

#### POST /container

create a container

##### parameters

- request: body/json

```json
{
    "image": "nginx:latest",
    "name": "nginx",
    "command": "nginx -g 'daemon off;'",
    "env": {
        "key": "value"
    },
    "ports": 8080,
    "cpu": 1,
    "memory": 1,
}
```

- response: body/json

```json
{
    "id": "service id",
    "name": "service name",
    "code":0,
    "message": ""
}
```

#### GET /container

list all containers

- response body/json

```json
{
    "containers": [
        {
            "cid": "container id",
            "name": "container name",
            "status": "running",
            "image": "nginx:latest",
            "command": "nginx -g 'daemon off;'",
            "env": {
                "key": "value"
            },
            "port": 8080,
            "ip": "instance ip",
            "domain": "service.minik.com"
        }
    ],
    "code":0,
    "message": ""
}
```

#### GET /container/:cid

get a container

- request: body/json

```json
{
    "containers": 
        {
            "cid": "container id",
            "name": "container name",
            "status": "running",
            "image": "nginx:latest",
            "command": "nginx -g 'daemon off;'",
            "env": {
                "key": "value"
            },
            "port": 8080,
            "ip": "instance ip",
            "domain": "service.minik.com"
        },
    "code":0,
    "message": ""
}
```

#### DELETE /container/:cid

delete a container

```json
{
    "id": "service id",
    "name": "service name",
    "code":0,
    "message": ""
}
```
