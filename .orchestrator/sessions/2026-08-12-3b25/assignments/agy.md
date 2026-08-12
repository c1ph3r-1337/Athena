# Task Assignment: agy

**Project:** _REST_API_Project
**Session ID:** 2026-08-12-3b25
**Workspace:** /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-3b25/workspace/agy

## Role Description
You are responsible for completing the tasks listed below. Work strictly within your assigned workspace.

## Assigned Tasks

### Task T003: Database Design and Migrations
**Description:** Configure the database connection for PostgreSQL using an ORM/Query Builder (e.g., Prisma or Drizzle). Design the initial schema for Users and a primary business resource. Generate initial migration files and create database seed scripts.
**Dependencies:** T001
**Expected Outputs:**
- src/config/database.ts
- prisma/schema.prisma
- migrations/001_init.sql
- prisma/seed.ts

### Task T005: Core Business Logic and Resource Endpoints
**Description:** Develop full CRUD operations for the primary business entities. Implement robust service layers handling business logic and controllers handling HTTP contexts. Ensure endpoints support standardized pagination, filtering, and sorting.
**Dependencies:** T003, T002
**Expected Outputs:**
- src/controllers/resourceController.ts
- src/services/resourceService.ts
- src/routes/resourceRoutes.ts
- src/validators/resourceSchema.ts

## Allowed Workspace Path
All files must be created within: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-3b25/workspace/agy

## Completion Criteria
1. All expected outputs are present and correct.
2. Code compiles and runs without errors.
3. Exit cleanly upon finishing.
