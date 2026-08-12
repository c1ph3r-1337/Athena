# REST API Project

A Node.js + TypeScript + Express REST API.

## Features
- Task management (CRUD operations)
- PostgreSQL database using Prisma ORM
- Jest & Supertest for testing
- Swagger API documentation
- Docker & Docker Compose support

## Setup

1. **Install dependencies:**
   ```bash
   npm install
   ```

2. **Environment Variables:**
   Copy `.env.example` to `.env` and adjust variables.

3. **Database Migrations:**
   ```bash
   npx prisma migrate dev
   ```

4. **Run the API:**
   ```bash
   npm run dev
   ```

## Docker

To run the application and PostgreSQL database via Docker:

```bash
docker-compose up --build
```

## Tests

```bash
npm test
```
