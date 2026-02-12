# Rusty Project

## Overview
Simple rust application

## Prerequisites
- [install](https://doc.rust-lang.org/cargo/getting-started/installation.html) cargo & rust

## Commands
- Start API Service
  ```sh
  make up
  ```
- Shut Down API Service
  ```sh
  make down
  ```
- Build API Service docker image
  ```sh
  make docker-build
  ```
- Build all docker images
  ```sh
  make build-all
  ```
- Build API Service local image
  ```sh
  make local-build
  ```
- Setup DB (note: password is `password`)
  ```sh
  make setup-db
  ```
- Run clippy lints
  ```sh
  make lint
  ```
- Auto-fix clippy warnings
  ```sh
  make lint-fix
  ```
- Format code
  ```sh
  make fmt
  ```
- Check code formatting
  ```sh
  make fmt-check
  ```
- Run all checks (format, lint, tests)
  ```sh
  make check
  ```
- Run tests
  ```sh
  make test
  ```
- Run tests with verbose output
  ```sh
  make test-verbose
  ```
- View all logs
  ```sh
  make logs
  ```
- View nginx logs
  ```sh
  make logs-nginx
  ```
- View API logs
  ```sh
  make logs-api
  ```
- View service status
  ```sh
  make status
  ```

## API Endpoints

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| GET | `/health/live` | Liveness check | No |
| GET | `/health/ready` | Readiness check | No |
| POST | `/users` | Create user | Yes |
| GET | `/users/{id}` | Get user | Yes |
| PUT | `/users/{id}` | Update user | Yes |
| DELETE | `/users/{id}` | Delete user | Yes |

**Authentication:** Include `X-API-Key` header with your API key.

**Example:**
```sh
curl -X POST http://localhost/users \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-api-key" \
  -d '{"name": "John Doe", "email": "john@example.com"}'
```
