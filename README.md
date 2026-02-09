# Metabrowser API & Search Aggregator

A high-performance search engine aggregator built with **Rust**. This project concurrently fetches results from multiple providers and provides a unified API for the frontend.

---

## Tech Stack

* **Language**: Rust
* **Web Framework**: Actix-web
* **ORM**: Diesel (PostgreSQL)
* **Security**: Argon2 (Password hashing), Actix-session (Cookies)
* **API Docs**: Utoipa & Swagger UI

---

## Getting Started

### 1. Prerequisites
Ensure you have the following installed:
* Rust (latest stable)
* PostgreSQL
* `diesel_cli` (`cargo install diesel_cli --no-default-features --features postgres`)

### 2. Environment Setup
Create a `.env` file in the root directory:
```env
DATABASE_URL=postgres://user:password@localhost/metabrowser
BRAVE_API_KEY=your_api_key_here
