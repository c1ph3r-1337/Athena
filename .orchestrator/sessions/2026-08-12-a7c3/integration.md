# Integration Instructions for Project: _REST_API_Project

## Original Project Requirements Summary
Integrate the outputs of all agents to produce the final deliverable.

## Agent Workspaces

- **agy**: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/agy
- **codex**: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex

## Assigned Work

- **Task T001** (Project Initialization and Base Architecture) assigned to: agy
- **Task T002** (Database Design and ORM Configuration) assigned to: codex
- **Task T003** (Authentication and Security Implementation) assigned to: codex
- **Task T004** (Core Domain CRUD Implementation) assigned to: agy
- **Task T005** (Automated Testing Suite) assigned to: codex
- **Task T006** (API Documentation and Developer Experience) assigned to: agy

## Dependencies Between Work

- Task T002 depends on: T001
- Task T003 depends on: T002
- Task T004 depends on: T002
- Task T005 depends on: T003, T004
- Task T006 depends on: T003, T004

## Artifacts Produced

- From Task T001:
  - package.json
  - tsconfig.json
  - src/app.ts
  - src/server.ts
  - .env.example
  - .gitignore
  - src/middleware/errorHandler.ts
- From Task T002:
  - prisma/schema.prisma
  - src/config/database.ts
  - prisma/seed.ts
- From Task T003:
  - src/middleware/auth.ts
  - src/middleware/security.ts
  - src/services/authService.ts
  - src/controllers/authController.ts
  - src/routes/authRoutes.ts
  - src/validators/authValidator.ts
- From Task T004:
  - src/controllers/resourceController.ts
  - src/services/resourceService.ts
  - src/routes/resourceRoutes.ts
  - src/validators/resourceValidator.ts
- From Task T005:
  - jest.config.js
  - tests/auth.test.ts
  - tests/resource.test.ts
  - tests/setup.ts
- From Task T006:
  - src/config/swagger.ts
  - README.md

## Integration Requirements
1. Verify all dependencies are met before merging code.
2. Ensure cross-agent functionality works as intended.

## Validation Requirements
1. Final build completes successfully.
2. Integration tests pass.
