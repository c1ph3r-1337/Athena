# Integration Instructions for Project: _REST_API_Project

## Original Project Requirements Summary
Integrate the outputs of all agents to produce the final deliverable.

## Agent Workspaces

- **claude**: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-3b25/workspace/claude
- **agy**: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-3b25/workspace/agy
- **codex**: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-3b25/workspace/codex

## Assigned Work

- **Task T001** (Project Initialization and Architecture) assigned to: claude
- **Task T002** (Error Handling, Logging, and Validation) assigned to: claude
- **Task T003** (Database Design and Migrations) assigned to: agy
- **Task T004** (Authentication and Security Layer) assigned to: codex
- **Task T005** (Core Business Logic and Resource Endpoints) assigned to: agy
- **Task T006** (Automated Testing Implementation) assigned to: codex
- **Task T007** (API Documentation and Polish) assigned to: claude

## Dependencies Between Work

- Task T002 depends on: T001
- Task T003 depends on: T001
- Task T004 depends on: T003, T002
- Task T005 depends on: T003, T002
- Task T006 depends on: T004, T005
- Task T007 depends on: T006

## Artifacts Produced

- From Task T001:
  - package.json
  - tsconfig.json
  - src/app.ts
  - src/server.ts
  - docker-compose.yml
  - .env.example
- From Task T002:
  - src/middleware/errorHandler.ts
  - src/utils/logger.ts
  - src/utils/responseFormatter.ts
  - src/middleware/validator.ts
  - src/utils/AppError.ts
- From Task T003:
  - src/config/database.ts
  - prisma/schema.prisma
  - migrations/001_init.sql
  - prisma/seed.ts
- From Task T004:
  - src/middleware/auth.ts
  - src/middleware/security.ts
  - src/services/authService.ts
  - src/controllers/authController.ts
  - src/routes/authRoutes.ts
- From Task T005:
  - src/controllers/resourceController.ts
  - src/services/resourceService.ts
  - src/routes/resourceRoutes.ts
  - src/validators/resourceSchema.ts
- From Task T006:
  - jest.config.js
  - tests/setup.ts
  - tests/integration/auth.test.ts
  - tests/integration/resource.test.ts
  - tests/unit/error.test.ts
- From Task T007:
  - swagger.yaml
  - src/routes/docs.ts
  - README.md

## Integration Requirements
1. Verify all dependencies are met before merging code.
2. Ensure cross-agent functionality works as intended.

## Validation Requirements
1. Final build completes successfully.
2. Integration tests pass.
