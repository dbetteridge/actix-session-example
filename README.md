# Setup

- create a .env file in the root using the .env.example
- `cargo run` to start the API

POST to /register to create a user
```json
{
    "username": string,
    "password": string,
    "email" : string,
    "name" : string
}
```

POST to /login to create a session
```json
{
    "username": string,
    "password": string,
    "email" : string,
    "name" : string
}
```