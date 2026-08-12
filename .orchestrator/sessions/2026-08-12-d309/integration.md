# Integration Instructions for Project: _REST_API_Project

## Original Project Requirements Summary
Integrate the outputs of all agents to produce the final deliverable.

## Agent Workspaces

- **agy**: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-d309/workspace/agy
- **codex**: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-d309/workspace/codex

## Assigned Work

- **Task T001** (Project Initialization and Architecture Setup) assigned to: agy
- **Task T002** (Database Schema and Migrations) assigned to: agy
- **Task T003** (Authentication and Security Implementation) assigned to: codex
- **Task T004** (Core Resource API Endpoints) assigned to: agy
- **Task T005** (Validation and Centralized Error Handling) assigned to: codex
- **Task T006** (Automated Testing) assigned to: agy
- **Task T007** (API Documentation and Developer Experience) assigned to: agy

## Dependencies Between Work

- Task T002 depends on: T001
- Task T003 depends on: T002
- Task T004 depends on: T003
- Task T005 depends on: T004
- Task T006 depends on: T005
- Task T007 depends on: T006

## Artifacts Produced

- From Task T001:
  - package.json
  - tsconfig.json
  - src/app.ts
  - src/config/database.ts
  - .env.example
- From Task T002:
  - prisma/schema.prisma
  - src/models/index.ts
- From Task T003:
  - src/controllers/auth.controller.ts
  - src/services/auth.service.ts
  - src/middleware/auth.middleware.ts
  - src/utils/jwt.util.ts
- From Task T004:
  - src/routes/task.routes.ts
  - src/controllers/task.controller.ts
  - src/services/task.service.ts
- From Task T005:
  - src/middleware/error.middleware.ts
  - src/validators/auth.validator.ts
  - src/validators/task.validator.ts
  - src/utils/customError.ts
- From Task T006:
  - jest.config.js
  - tests/auth.test.ts
  - tests/task.test.ts
  - tests/error.test.ts
- From Task T007:
  - src/docs/swagger.ts
  - README.md
  - Dockerfile
  - docker-compose.yml

## Integration Requirements
1. Verify all dependencies are met before merging code.
2. Ensure cross-agent functionality works as intended.

## Validation Requirements
1. Final build completes successfully.
2. Integration tests pass.
