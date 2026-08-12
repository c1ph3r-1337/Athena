# Task Assignment: codex

**Project:** _REST_API_Project
**Session ID:** 2026-08-12-a7c3
**Workspace:** /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex

## Role Description
You are responsible for completing the tasks listed below. Work strictly within your assigned workspace.

## Assigned Tasks

### Task T002: Database Design and ORM Configuration
**Description:** Configure PostgreSQL database connection using an ORM like Prisma or TypeORM. Design the database schema to include User entities and a core domain entity (e.g., Post or Task). Create the initial database migrations and basic seed scripts.
**Dependencies:** T001
**Expected Outputs:**
- prisma/schema.prisma
- src/config/database.ts
- prisma/seed.ts

### Task T003: Authentication and Security Implementation
**Description:** Implement secure authentication using JWT. Create user registration and login services with password hashing (bcrypt). Implement role-based authorization middleware. Add security enhancements including Helmet, CORS configuration, rate limiting, and request validation (using Zod or Joi).
**Dependencies:** T002
**Expected Outputs:**
- src/middleware/auth.ts
- src/middleware/security.ts
- src/services/authService.ts
- src/controllers/authController.ts
- src/routes/authRoutes.ts
- src/validators/authValidator.ts

### Task T005: Automated Testing Suite
**Description:** Set up the testing framework (Jest + Supertest). Write comprehensive unit and integration tests covering authentication flows, core CRUD operations, input validation, and expected error handling scenarios. Ensure the tests can run in a clean environment.
**Dependencies:** T003, T004
**Expected Outputs:**
- jest.config.js
- tests/auth.test.ts
- tests/resource.test.ts
- tests/setup.ts

## Allowed Workspace Path
All files must be created within: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex

## Completion Criteria
1. All expected outputs are present and correct.
2. Code compiles and runs without errors.
3. Exit cleanly upon finishing.
