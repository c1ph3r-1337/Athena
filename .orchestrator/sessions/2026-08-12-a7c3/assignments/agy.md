# Task Assignment: agy

**Project:** _REST_API_Project
**Session ID:** 2026-08-12-a7c3
**Workspace:** /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/agy

## Role Description
You are responsible for completing the tasks listed below. Work strictly within your assigned workspace.

## Assigned Tasks

### Task T001: Project Initialization and Base Architecture
**Description:** Initialize a Node.js + TypeScript + Express project. Configure TypeScript, ESLint, Prettier, and environment variables. Set up the basic Express application structure, including the main server file, global error handling middleware, and basic routing framework.
**Expected Outputs:**
- package.json
- tsconfig.json
- src/app.ts
- src/server.ts
- .env.example
- .gitignore
- src/middleware/errorHandler.ts

### Task T004: Core Domain CRUD Implementation
**Description:** Implement full CRUD REST API endpoints for the core domain entity. This includes creating robust controllers, business logic services, and routing. Implement proper pagination, filtering, and sorting for GET requests. Ensure all inputs are validated and standard JSON responses are returned.
**Dependencies:** T002
**Expected Outputs:**
- src/controllers/resourceController.ts
- src/services/resourceService.ts
- src/routes/resourceRoutes.ts
- src/validators/resourceValidator.ts

### Task T006: API Documentation and Developer Experience
**Description:** Implement OpenAPI/Swagger documentation detailing all API endpoints, request/response schemas, and authentication methods. Create a comprehensive README.md with setup, execution, and testing instructions. Verify code quality and perform final refactoring.
**Dependencies:** T003, T004
**Expected Outputs:**
- src/config/swagger.ts
- README.md

## Allowed Workspace Path
All files must be created within: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/agy

## Completion Criteria
1. All expected outputs are present and correct.
2. Code compiles and runs without errors.
3. Exit cleanly upon finishing.
