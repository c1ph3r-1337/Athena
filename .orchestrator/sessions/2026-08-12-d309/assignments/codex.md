# Task Assignment: codex

**Project:** _REST_API_Project
**Session ID:** 2026-08-12-d309
**Workspace:** /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-d309/workspace/codex

## Role Description
You are responsible for completing the tasks listed below. Work strictly within your assigned workspace.

## Assigned Tasks

### Task T003: Authentication and Security Implementation
**Description:** Implement secure user authentication using JWT and bcrypt for password hashing. Create authentication middleware, implement rate limiting, and configure secure HTTP headers (Helmet/CORS). Ensure proper protection against common vulnerabilities.
**Dependencies:** T002
**Expected Outputs:**
- src/controllers/auth.controller.ts
- src/services/auth.service.ts
- src/middleware/auth.middleware.ts
- src/utils/jwt.util.ts

### Task T005: Validation and Centralized Error Handling
**Description:** Implement centralized error handling middleware to format consistent JSON error responses. Implement request body and parameter validation using a library like Zod or Joi to validate all incoming API requests.
**Dependencies:** T004
**Expected Outputs:**
- src/middleware/error.middleware.ts
- src/validators/auth.validator.ts
- src/validators/task.validator.ts
- src/utils/customError.ts

## Allowed Workspace Path
All files must be created within: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-d309/workspace/codex

## Completion Criteria
1. All expected outputs are present and correct.
2. Code compiles and runs without errors.
3. Exit cleanly upon finishing.
