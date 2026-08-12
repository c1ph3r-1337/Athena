# Orchestration Plan

## Tasks: 7

### T001 - Project Initialization and Architecture
- **Agent**: claude
- **Dependencies**: None
- **Description**: Initialize a Node.js + TypeScript + Express project. Set up the core directory structure (src/routes, src/controllers, src/services, src/models, etc.), configure TypeScript, set up ESLint/Prettier, and create the basic Express app entry point. Provide a docker-compose.yml for a local PostgreSQL database setup.

### T002 - Error Handling, Logging, and Validation
- **Agent**: claude
- **Dependencies**: T001
- **Description**: Implement centralized error handling middleware. Set up structured logging (e.g., Pino or Winston). Create generic request validation middleware using a schema validator (like Zod) and establish standardized JSON response and error formats.

### T003 - Database Design and Migrations
- **Agent**: agy
- **Dependencies**: T001
- **Description**: Configure the database connection for PostgreSQL using an ORM/Query Builder (e.g., Prisma or Drizzle). Design the initial schema for Users and a primary business resource. Generate initial migration files and create database seed scripts.

### T004 - Authentication and Security Layer
- **Agent**: codex
- **Dependencies**: T003, T002
- **Description**: Implement secure authentication endpoints (register, login, refresh). Use bcrypt/argon2 for password hashing and JWT for stateless sessions. Create role-based authorization middleware. Add security enhancements including Helmet, CORS, and rate limiting.

### T005 - Core Business Logic and Resource Endpoints
- **Agent**: agy
- **Dependencies**: T003, T002
- **Description**: Develop full CRUD operations for the primary business entities. Implement robust service layers handling business logic and controllers handling HTTP contexts. Ensure endpoints support standardized pagination, filtering, and sorting.

### T006 - Automated Testing Implementation
- **Agent**: codex
- **Dependencies**: T004, T005
- **Description**: Set up a testing framework (Jest + Supertest). Write comprehensive integration tests for authentication flows and resource CRUD endpoints. Write unit tests for critical business logic and error handlers. Ensure test database setup/teardown works seamlessly.

### T007 - API Documentation and Polish
- **Agent**: claude
- **Dependencies**: T006
- **Description**: Implement Swagger/OpenAPI documentation for all REST endpoints, defining request/response bodies, parameters, and auth requirements. Write a comprehensive README.md detailing project setup, execution commands, and architectural decisions.

## Available Agents

- **agy** (/home/c1ph3r/.local/bin/agy): coding, architecture, debugging, testing, documentation
- **codex** (/usr/bin/codex): coding, refactoring, review, security
- **claude** (/home/c1ph3r/.local/bin/claude): coding, architecture, analysis, documentation, review
