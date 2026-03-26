# Notes REST API 🦀

A production-grade REST API built in Rust with async Rust, axum, sqlx, and PostgreSQL. Full CRUD operations, database migrations, tests, and CI/CD pipeline.

## Features

- Full CRUD REST API with 5 endpoints
- Async throughout with tokio runtime
- PostgreSQL database with sqlx migrations
- Compile-time SQL query validation
- Proper error handling with Result
- Clean modular architecture
- Integration tests
- GitHub Actions CI/CD pipeline
- Docker for local development

## Tech Stack

- [Rust](https://www.rust-lang.org/) — systems programming language
- [axum](https://crates.io/crates/axum) — async web framework
- [sqlx](https://crates.io/crates/sqlx) — async database access with compile-time query validation
- [tokio](https://crates.io/crates/tokio) — async runtime
- [PostgreSQL](https://www.postgresql.org/) — relational database
- [Docker](https://www.docker.com/) — local development environment

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/notes` | Get all notes |
| POST | `/notes` | Create a new note |
| GET | `/notes/:id` | Get a note by id |
| PUT | `/notes/:id` | Update a note |
| DELETE | `/notes/:id` | Delete a note |

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/)
- [Docker Desktop](https://www.docker.com/products/docker-desktop)
- [sqlx-cli](https://crates.io/crates/sqlx-cli)

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

### Setup

**1 — Clone the repository**

```bash
git clone https://github.com/YOUR_USERNAME/rest_api
cd rest_api
```

**2 — Create a `.env` file**

```
DATABASE_URL=postgres://postgres:password@localhost:5432/rest_api
```

**3 — Start the database**

```bash
docker-compose up -d
```

**4 — Run migrations**

```bash
sqlx migrate run
```

**5 — Start the server**

```bash
cargo run
```

Server runs at `http://localhost:3000`

## Usage Examples

**Get all notes**
```bash
curl http://localhost:3000/notes
```

**Create a note**
```bash
curl -X POST http://localhost:3000/notes \
  -H "Content-Type: application/json" \
  -d '{"title": "My note", "content": "Hello from Rust!"}'
```

**Get a note by id**
```bash
curl http://localhost:3000/notes/<id>
```

**Update a note**
```bash
curl -X PUT http://localhost:3000/notes/<id> \
  -H "Content-Type: application/json" \
  -d '{"title": "Updated title", "content": "Updated content"}'
```

**Delete a note**
```bash
curl -X DELETE http://localhost:3000/notes/<id>
```

## Example Response

```json
{
  "id": "2360b914-6cd5-4e36-82dd-c01ee0d2ddee",
  "title": "My note",
  "content": "Hello from Rust!",
  "created_at": "2026-03-26T19:57:03.521839Z"
}
```

## Project Structure

```
src/
├── main.rs        → server setup and router
├── models.rs      → Note and CreateNote structs
└── handlers.rs    → route handlers and tests
migrations/
└── xxxxxxxx_create_notes_table.sql
```

## Running Tests

```bash
cargo test
```

## CI/CD

GitHub Actions pipeline runs on every push:

- Spins up a fresh PostgreSQL container
- Runs database migrations
- Checks code formatting with `cargo fmt`
- Runs `cargo clippy` linter
- Runs all tests

## Roadmap

- [ ] Authentication with JWT
- [ ] Search notes by title
- [ ] Pagination for GET /notes
- [ ] Rate limiting
- [ ] Redis caching

## License

MIT