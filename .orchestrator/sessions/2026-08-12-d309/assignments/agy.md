# Task Assignment: agy

**Project:** _REST_API_Project
**Session ID:** 2026-08-12-d309
**Workspace:** /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-d309/workspace/agy

## Role Description
You are responsible for completing the tasks listed below. Work strictly within your assigned workspace.

## Assigned Tasks

### Task T001: Project Initialization and Architecture Setup
**Description:** Initialize a Node.js + TypeScript + Express project. Configure ESLint, Prettier, and environment variables. Set up the basic Express server and database connection configuration (e.g., PostgreSQL using Prisma or TypeORM).
**Expected Outputs:**
- package.json
- tsconfig.json
- src/app.ts
- src/config/database.ts
- .env.example

### Task T002: Database Schema and Migrations
**Description:** Design and implement the database schema for the REST API (assuming a Task Management system with Users and Tasks). Set up the ORM models, relationships, and generate initial database migrations.
**Dependencies:** T001
**Expected Outputs:**
- prisma/schema.prisma
- src/models/index.ts

### Task T004: Core Resource API Endpoints
**Description:** Implement the core CRUD REST endpoints for the primary resources (e.g., Tasks). Include support for pagination, filtering, sorting, and ensure proper separation of concerns (Routes -> Controllers -> Services).
**Dependencies:** T003
**Expected Outputs:**
- src/routes/task.routes.ts
- src/controllers/task.controller.ts
- src/services/task.service.ts

### Task T006: Automated Testing
**Description:** Set up a testing framework (e.g., Jest and Supertest). Write comprehensive integration and unit tests for authentication, authorization, CRUD operations, and error handling edge cases.
**Dependencies:** T005
**Expected Outputs:**
- jest.config.js
- tests/auth.test.ts
- tests/task.test.ts
- tests/error.test.ts

### Task T007: API Documentation and Developer Experience
**Description:** Generate OpenAPI/Swagger documentation for all endpoints. Write a comprehensive README.md detailing how to set up, configure, run, and test the API. Ensure Docker support is included by providing necessary configuration files.
**Dependencies:** T006
**Expected Outputs:**
- src/docs/swagger.ts
- README.md
- Dockerfile
- docker-compose.yml

## Allowed Workspace Path
All files must be created within: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-d309/workspace/agy

## Completion Criteria
1. All expected outputs are present and correct.
2. Code compiles and runs without errors.
3. Exit cleanly upon finishing.
