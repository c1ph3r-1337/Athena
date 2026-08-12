# Task Assignment: claude

**Project:** _REST_API_Project
**Session ID:** 2026-08-12-3b25
**Workspace:** /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-3b25/workspace/claude

## Role Description
You are responsible for completing the tasks listed below. Work strictly within your assigned workspace.

## Assigned Tasks

### Task T001: Project Initialization and Architecture
**Description:** Initialize a Node.js + TypeScript + Express project. Set up the core directory structure (src/routes, src/controllers, src/services, src/models, etc.), configure TypeScript, set up ESLint/Prettier, and create the basic Express app entry point. Provide a docker-compose.yml for a local PostgreSQL database setup.
**Expected Outputs:**
- package.json
- tsconfig.json
- src/app.ts
- src/server.ts
- docker-compose.yml
- .env.example

### Task T002: Error Handling, Logging, and Validation
**Description:** Implement centralized error handling middleware. Set up structured logging (e.g., Pino or Winston). Create generic request validation middleware using a schema validator (like Zod) and establish standardized JSON response and error formats.
**Dependencies:** T001
**Expected Outputs:**
- src/middleware/errorHandler.ts
- src/utils/logger.ts
- src/utils/responseFormatter.ts
- src/middleware/validator.ts
- src/utils/AppError.ts

### Task T007: API Documentation and Polish
**Description:** Implement Swagger/OpenAPI documentation for all REST endpoints, defining request/response bodies, parameters, and auth requirements. Write a comprehensive README.md detailing project setup, execution commands, and architectural decisions.
**Dependencies:** T006
**Expected Outputs:**
- swagger.yaml
- src/routes/docs.ts
- README.md

## Allowed Workspace Path
All files must be created within: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-3b25/workspace/claude

## Completion Criteria
1. All expected outputs are present and correct.
2. Code compiles and runs without errors.
3. Exit cleanly upon finishing.
