# Mudro: A microservice featuring quotes from renowned figures.

[MIT License](LICENSE.txt)

This Rust microservice was developed as a straightforward solution for managing data related to authors and their quotas. It is implemented in Rust, utilizing sqlx, actix-web, and postgresql.

The primary motivation behind the development of this application was to gain proficiency in the Rust programming language.

## How to run (in docker)

Build containers
```shell
docker-compose up -d --build
```

will be run on http://localhost:1339/

Stop conainers
```shell
docker-compose stop
```

Run existing containers
```shell
docker-compose stop
```

Remove containers and volumes
```shell
docker-compose down
```

To see running containers
```shell
docker ps
```

TODO: try to integrate OpenAPI schema

## Example querys:

- Get ranrom quote:

GET http://localhost:1339/api/quotes/random

```json
{
    "data": {
        "quote": {
            "author_id": 1,
            "author_name": "Steve Jobs",
            "id": 1,
            "text": "The only way to do great work is to love what you do."
        }
    },
    "status": "success"
}
```

- Get an Author by id:

GET http://localhost:1339/api/authors/1

```json
{
    "data": {
        "author": {
            "author_name": "Steve Jobs",
            "id": 1
        }
    },
    "status": "success"
}
```
- Get list of Authors:

GET http://localhost:1339/api/authors?limit=3&offset=10

```json
{
    "data": [
        {
            "author_name": "Franklin D. Roosevelt",
            "id": 4
        },
        {
            "author_name": "Henry David Thoreau",
            "id": 33
        },
        {
            "author_name": "J.K. Rowling",
            "id": 13
        }
    ],
    "status": "success"
}
```

- Create new Author record:

POST http://localhost:1339/api/authors

payload:

```json
{"author_name": "J. R. R. Tolkien"}
```

response:

```json
{
    "data": {
        "author": {
            "author_name": "J. R. R. Tolkien",
            "id": 34
        }
    },
    "status": "success"
}
```

- Update an Author record:

PATCH http://localhost:1339/api/authors/34

payload:

```json
{"author_name": "John Ronald Reuel Tolkien"}
```

response:

```json
{
    "data": {
        "author": {
            "author_name": "John Ronald Reuel Tolkien",
            "id": 34
        }
    },
    "status": "success"
}
```
- Delete an existing Author record:
  
DELETE http://localhost:1339/api/authors/34

response like in update command.

- Create new quote record:

POST http://localhost:1339/api/quotes

payload:

```json
{"author_id": 34, "text": "Home Is Behind, The World Ahead"}
```

response:

```json
{
    "data": {
        "quote": {
            "author_id": 34,
            "id": 41,
            "text": "Home Is Behind, The World Ahead"
        }
    },
    "status": "success"
}
```

- Get Quote record by id:

GET http://localhost:1339/api/quotes/41

response like in create record item.

- Update record:

PATCH http://localhost:1339/api/quotes/41

payload:

```json
{"author_id": 34, "text": "Home Is Behind, The World Ahead."}
```

response:

```json
{
    "data": {
        "quote": {
            "author_id": 34,
            "id": 41,
            "text": "Home Is Behind, The World Ahead."
        }
    },
    "status": "success"
}
```

Delete quote by id:

DELETE http://localhost:1339/api/quotes/41

response like in previous item.


## How to run (in kubernetis)

> **_NOTE:_** Draft (without nginx), tested in minkube only with port forwarding 8080:8080.

```shell
cd kub_conf/conf1/
kubectl apply -f .
```
Set port forwarding 8080:8080 fnd enjoy: http://localhost:8080/api/quotes/random
