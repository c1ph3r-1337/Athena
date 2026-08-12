# Task Assignment: codex

**Project:** _REST_API_Project
**Session ID:** 2026-08-12-3b25
**Workspace:** /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-3b25/workspace/codex

## Role Description
You are responsible for completing the tasks listed below. Work strictly within your assigned workspace.

## Assigned Tasks

### Task T004: Authentication and Security Layer
**Description:** Implement secure authentication endpoints (register, login, refresh). Use bcrypt/argon2 for password hashing and JWT for stateless sessions. Create role-based authorization middleware. Add security enhancements including Helmet, CORS, and rate limiting.
**Dependencies:** T003, T002
**Expected Outputs:**
- src/middleware/auth.ts
- src/middleware/security.ts
- src/services/authService.ts
- src/controllers/authController.ts
- src/routes/authRoutes.ts

### Task T006: Automated Testing Implementation
**Description:** Set up a testing framework (Jest + Supertest). Write comprehensive integration tests for authentication flows and resource CRUD endpoints. Write unit tests for critical business logic and error handlers. Ensure test database setup/teardown works seamlessly.
**Dependencies:** T004, T005
**Expected Outputs:**
- jest.config.js
- tests/setup.ts
- tests/integration/auth.test.ts
- tests/integration/resource.test.ts
- tests/unit/error.test.ts

## Allowed Workspace Path
All files must be created within: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-3b25/workspace/codex

## Completion Criteria
1. All expected outputs are present and correct.
2. Code compiles and runs without errors.
3. Exit cleanly upon finishing.
