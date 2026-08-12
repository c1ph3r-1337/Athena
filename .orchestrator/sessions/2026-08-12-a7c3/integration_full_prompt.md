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


---

# Original Project Requirements

# REST API Project — Autonomous Development Prompt

You are an expert backend engineer and software architect.

Your task is to **analyze, design, implement, test, and document a complete production-quality REST API project**.

Do not immediately start writing code. First understand the requirements, identify the architecture, determine the necessary components, and then implement the project systematically.

## 1. Project Goal

Build a complete RESTful API for:

**[DESCRIBE YOUR PROJECT IDEA HERE]**

The API should be realistic, well-structured, scalable, secure, maintainable, and suitable as a serious B.Tech/portfolio project.

## 2. Development Process

Follow this workflow:

### Phase 1 — Analyze

* Understand the project requirements.
* Identify the main entities/resources.
* Identify relationships between entities.
* Determine required API operations.
* Identify authentication and authorization requirements.
* Identify validation requirements.
* Identify possible security risks.
* Identify edge cases and failure scenarios.
* Identify what should be configurable through environment variables.

Before implementation, create a concise technical plan.

### Phase 2 — Architecture

Design a clean backend architecture using appropriate separation of concerns.

Consider:

* Routes
* Controllers
* Services
* Models/entities
* Repository/data-access layer
* Middleware
* Authentication
* Authorization
* Validation
* Error handling
* Logging
* Configuration
* Database
* Testing

Use the architecture that best fits the chosen technology stack rather than unnecessarily overengineering the project.

## 3. Technology Selection

Choose an appropriate modern technology stack.

Prefer one of:

* Node.js + TypeScript + Express/Fastify
* Python + FastAPI
* Java + Spring Boot
* Go + Gin/Fiber

Choose the stack based on the project requirements and explain the choice briefly.

Use a proper relational database such as PostgreSQL when relational data is appropriate.

## 4. REST API Design

Follow proper REST principles.

Define:

* Resources
* HTTP methods
* Routes
* Request parameters
* Request bodies
* Response formats
* HTTP status codes
* Pagination
* Filtering
* Sorting
* Searching where appropriate

Use consistent JSON responses.

For example:

```json
{
  "success": true,
  "data": {},
  "message": "Operation successful"
}
```

Errors should also have a consistent structure.

## 5. Authentication & Security

If authentication is relevant, implement it properly.

Consider:

* Password hashing
* JWT or secure session-based authentication
* Access/refresh tokens where appropriate
* Role-based authorization
* Input validation
* Rate limiting
* CORS configuration
* Secure HTTP headers
* SQL/NoSQL injection prevention
* Authentication brute-force protection
* Sensitive-data protection
* Proper secret management
* Environment variables

Never hard-code passwords, API keys, tokens, or database credentials.

## 6. Database

Design the database schema before implementation.

Determine:

* Tables/entities
* Primary keys
* Foreign keys
* Relationships
* Indexes
* Constraints
* Unique fields
* Required fields
* Timestamps

Use migrations rather than manually creating the database schema.

Add seed/demo data if useful.

## 7. API Documentation

Provide complete API documentation.

Use **OpenAPI/Swagger** where supported.

Document:

* Every endpoint
* Authentication requirements
* Parameters
* Request bodies
* Response examples
* Error responses
* Status codes

Also create a useful README explaining how to run and use the API.

## 8. Testing

Do not consider the project complete until it has been tested.

Create tests for:

* Authentication
* Authorization
* CRUD operations
* Validation
* Error handling
* Database interactions
* Important edge cases

Include integration/API tests where appropriate.

The project should have a simple command such as:

```bash
npm test
```

or the equivalent for the selected stack.

## 9. Developer Experience

Provide:

* `.env.example`
* Proper `.gitignore`
* Package/dependency configuration
* Database migration commands
* Seed commands if applicable
* Development commands
* Production build commands
* Docker support if useful
* API documentation
* Example API requests

Make the project easy for another developer to clone and run.

## 10. Code Quality

Write clean, readable, maintainable code.

Follow the conventions of the selected language/framework.

Avoid:

* Giant files
* Duplicated logic
* Hard-coded configuration
* Unnecessary abstractions
* Dead code
* Unused dependencies
* Poor naming
* Silent error handling

Use meaningful names and comments only where they add value.

## 11. Error Handling

Implement centralized error handling.

Handle:

* Invalid input
* Missing resources
* Authentication failures
* Authorization failures
* Database errors
* Invalid routes
* Unexpected server errors

Return appropriate HTTP status codes such as:

* `200`
* `201`
* `204`
* `400`
* `401`
* `403`
* `404`
* `409`
* `422`
* `429`
* `500`

Do not expose internal stack traces or sensitive implementation details in production responses.

## 12. Project Structure

Choose a clean project structure appropriate for the selected framework.

For example:

```text
project/
├── src/
│   ├── routes/
│   ├── controllers/
│   ├── services/
│   ├── models/
│   ├── middleware/
│   ├── validators/
│   ├── config/
│   ├── utils/
│   └── app.*
├── tests/
├── migrations/
├── .env.example
├── .gitignore
├── README.md
└── package/config files
```

Adapt this structure if another architecture is more appropriate.

## 13. Autonomous Execution

You are allowed to:

* Create files
* Modify files
* Install required dependencies
* Create the database schema
* Run migrations
* Run tests
* Run linters/formatters
* Start the development server
* Inspect errors
* Debug problems
* Refactor the implementation

When something fails:

1. Inspect the error.
2. Determine the root cause.
3. Fix it.
4. Re-run the relevant test/command.
5. Continue until the issue is resolved.

Do not stop at the first error.

Do not repeatedly ask for permission for routine development operations if your execution environment already permits them.

## 14. Quality Gate

Before declaring the project complete, verify:

* [ ] API starts successfully
* [ ] Database connects successfully
* [ ] Migrations work
* [ ] Authentication works
* [ ] Authorization works where required
* [ ] CRUD operations work
* [ ] Validation works
* [ ] Error handling works
* [ ] Tests pass
* [ ] Linting/formatting passes
* [ ] Swagger/OpenAPI documentation works
* [ ] README contains setup instructions
* [ ] No secrets are committed
* [ ] No obvious security vulnerabilities remain
* [ ] Project can be run from a clean environment

## 15. Final Deliverables

At the end, provide:

1. Final project structure
2. Technology stack
3. Database design
4. API endpoint list
5. Authentication design
6. Security measures
7. How to run the project
8. How to run tests
9. Example API requests
10. Known limitations
11. Possible future improvements

Most importantly:

**Think → Plan → Implement → Test → Debug → Verify → Document.**

Do not merely generate a collection of API endpoints. Build a complete, coherent, working REST API project that demonstrates good backend engineering practices.


---

# Agent Outputs



## Output from Project Initialization and Base Architecture (Agent: agy)

Agent Terminal: agy

> Task: Project Initialization and Base Architecture
Thinking... \033[0mError: Error: empty prompt. Usage: agy --print "your prompt here"











## Output from Database Design and ORM Configuration (Agent: codex)

Agent Terminal: codex

> Task: Database Design and ORM Configuration
  Waiting for dependencies...
OpenAI Codex v0.144.4
--------
workdir: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
model: gpt-5.6-terra
provider: openai
approval: never
sandbox: workspace-write [workdir, /tmp, $TMPDIR]
reasoning effort: medium
reasoning summaries: none
session id: 019ff60a-b832-7881-b3c5-f4fbb6dcad9d
--------
user
# Codex Agent Operating Instructions

## Identity
You are OpenAI Codex CLI, a code-generation and review agent powered by OpenAI's models.

## Operating Mode
You are being invoked by the Athena Meta-Orchestrator as part of a multi-agent project.

You have been assigned specific tasks. Focus exclusively on your assigned work.

## Rules
1. Work ONLY within your assigned workspace directory. Do not create files outside of it.
2. Read your task assignment carefully before starting.
3. Implement your assigned tasks completely and correctly.
4. Focus on code quality, security, and correctness.
5. Review your own output for bugs, vulnerabilities, and edge cases.
6. Include appropriate tests.
7. Do NOT modify files in other agents' workspaces.
8. When finished, ensure all expected output files exist in your workspace.

## Strengths
You excel at:
- Code review and refactoring
- Security analysis
- Test generation
- Bug detection

## Completion
When your tasks are complete:
1. Verify all expected outputs exist.
2. Run any applicable tests.
3. Exit cleanly.

## Error Handling
If you encounter an error:
1. Log the error clearly.
2. Attempt to fix it.
3. If unable to fix, document the issue and exit.

## Workspace
All your work must be saved inside your assigned workspace path.
Do not write to /tmp, /home, or any location outside your workspace.


---

# Your Assignment

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


---

# Current Task

## Database Design and ORM Configuration

Configure PostgreSQL database connection using an ORM like Prisma or TypeORM. Design the database schema to include User entities and a core domain entity (e.g., Post or Task). Create the initial database migrations and basic seed scripts.

## Workspace

Work inside: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
codex
I’ll inspect the existing project structure and conventions, then add the Prisma schema, database client, migration, and seed script without touching unrelated application work.
exec
/usr/bin/zsh -lc "pwd && rg --files -g '"'!*node_modules*'"' -g '"'!*.lock'"' | sed -n '1,240p'" in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 succeeded in 0ms:
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex

apply patch
patch: completed
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

codex
The workspace was empty, so I’ve added a self-contained Prisma/PostgreSQL foundation: UUID-backed users and owner-scoped tasks, an idempotent admin seed, and a development-safe shared client. I’m doing structural validation now.
exec
/usr/bin/zsh -lc 'git diff --check && find prisma src -type f -maxdepth 5 -print | sort && git status --short' in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 succeeded in 0ms:
prisma/migrations/20260812000000_initial_schema/migration.sql
prisma/schema.prisma
prisma/seed.ts
src/config/database.ts
?? ../../../../../.gitignore
?? ../../../../
?? ../../../../../Cargo.lock
?? ../../../../../Cargo.toml
?? ../../../../../README.md
?? ../../../../../architecture.md
?? ../../../../../athena-cli/
?? ../../../../../athena-core/
?? ../../../../../img_to_ascii.py
?? ../../../../../img_to_braille.py
?? ../../../../../large_prompt.md
?? ../../../../../prompt.md
?? ../../../../../templates/

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

exec
/usr/bin/zsh -lc "if command -v npx >/dev/null 2>&1; then npx --no-install prisma format --schema prisma/schema.prisma; else echo 'npx unavailable'; fi
rg --files prisma src .env.example | sort" in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

exec
/usr/bin/zsh -lc 'ls -la && find . -maxdepth 5 -type f -print | sort && test -f prisma/schema.prisma && test -f src/config/database.ts && test -f prisma/seed.ts && test -f prisma/migrations/20260812000000_initial_schema/migration.sql' in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 succeeded in 0ms:
total 20
drwxr-xr-x 7 c1ph3r c1ph3r 4096 Aug 12 18:26 .
drwxr-xr-x 4 c1ph3r c1ph3r 4096 Aug 12 18:25 ..
dr-xr-xr-x 2 c1ph3r c1ph3r   40 Aug 12 18:27 .agents
dr-xr-xr-x 2 c1ph3r c1ph3r   40 Aug 12 18:27 .codex
-rw-r--r-- 1 c1ph3r c1ph3r  254 Aug 12 18:26 .env.example
dr-xr-xr-x 2 c1ph3r c1ph3r   40 Aug 12 18:27 .git
drwxr-xr-x 3 c1ph3r c1ph3r 4096 Aug 12 18:26 prisma
drwxr-xr-x 3 c1ph3r c1ph3r 4096 Aug 12 18:26 src
./.env.example
./prisma/migrations/20260812000000_initial_schema/migration.sql
./prisma/schema.prisma
./prisma/seed.ts
./src/config/database.ts

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

exec
/usr/bin/zsh -lc "sed -n '1,240p' prisma/schema.prisma && sed -n '1,220p' src/config/database.ts && sed -n '1,260p' prisma/seed.ts" in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 succeeded in 0ms:
generator client {
  provider = "prisma-client-js"
}

datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}

enum Role {
  USER
  ADMIN
}

enum TaskStatus {
  TODO
  IN_PROGRESS
  DONE
}

model User {
  id           String   @id @default(uuid()) @db.Uuid
  email        String   @unique @db.VarChar(320)
  passwordHash String   @map("password_hash") @db.VarChar(255)
  name         String?  @db.VarChar(100)
  role         Role     @default(USER)
  createdAt    DateTime @default(now()) @map("created_at")
  updatedAt    DateTime @updatedAt @map("updated_at")
  tasks        Task[]

  @@map("users")
}

model Task {
  id          String     @id @default(uuid()) @db.Uuid
  title       String     @db.VarChar(200)
  description String?    @db.Text
  status      TaskStatus @default(TODO)
  ownerId     String     @map("owner_id") @db.Uuid
  createdAt   DateTime   @default(now()) @map("created_at")
  updatedAt   DateTime   @updatedAt @map("updated_at")
  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)

  @@index([ownerId])
  @@index([ownerId, status])
  @@map("tasks")
}
import { PrismaClient } from '@prisma/client';

/**
 * A single Prisma client is shared by the application. Keeping it on globalThis
 * prevents duplicate connection pools during development hot reloads.
 */
const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };

export const prisma =
  prismaGlobal.prisma ??
  new PrismaClient({
    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
  });

if (process.env.NODE_ENV !== 'production') {
  prismaGlobal.prisma = prisma;
}

/** Closes open database connections for graceful application shutdown. */
export const disconnectDatabase = async (): Promise<void> => {
  await prisma.$disconnect();
};

export default prisma;
import 'dotenv/config';
import bcrypt from 'bcryptjs';
import { PrismaClient, Role, TaskStatus } from '@prisma/client';

const prisma = new PrismaClient();

async function main(): Promise<void> {
  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
  const passwordHash = await bcrypt.hash(password, 12);

  const admin = await prisma.user.upsert({
    where: { email },
    update: {
      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
      passwordHash,
      role: Role.ADMIN,
    },
    create: {
      email,
      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
      passwordHash,
      role: Role.ADMIN,
    },
  });

  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
  if (starterTaskCount === 0) {
    await prisma.task.create({
      data: {
        title: 'Welcome to the task API',
        description: 'This seed task can be safely removed.',
        status: TaskStatus.TODO,
        ownerId: admin.id,
      },
    });
  }

  console.info(`Seeded administrator: ${admin.email}`);
}

main()
  .catch((error: unknown) => {
    console.error('Database seed failed:', error);
    process.exitCode = 1;
  })
  .finally(async () => {
    await prisma.$disconnect();
  });

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

codex
The schema and migration passed whitespace checks and Prisma formatting. The repository has no package manifest or installed project dependencies yet, so a full client-generation/type-check cannot run in this workspace; the files are ready for the project’s Prisma, `@prisma/client`, `bcryptjs`, and `dotenv` dependencies.
apply patch
patch: completed
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
new file mode 100644
index 0000000000000000000000000000000000000000..526938dd4fbc9ae48f822f5a37cc4eb928c9ebb3
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
@@ -0,0 +1,3 @@
+# Please do not edit this file manually.
+# It should be committed in your version-control system.
+provider = "postgresql"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
new file mode 100644
index 0000000000000000000000000000000000000000..526938dd4fbc9ae48f822f5a37cc4eb928c9ebb3
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
@@ -0,0 +1,3 @@
+# Please do not edit this file manually.
+# It should be committed in your version-control system.
+provider = "postgresql"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

codex
Completed database configuration.

Created:

- [Prisma schema](/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma) with `User`, `Task`, roles, statuses, relations, UUIDs, and indexes.
- [Initial PostgreSQL migration](/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql) plus Prisma migration lock.
- [Shared Prisma client](/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts).
- [Idempotent admin and starter-task seed](/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts).
- [.env.example](/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example).

Validated formatting and file presence. Full generation/type-check awaits the project package dependencies (`prisma`, `@prisma/client`, `bcryptjs`, `dotenv`).
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
new file mode 100644
index 0000000000000000000000000000000000000000..526938dd4fbc9ae48f822f5a37cc4eb928c9ebb3
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
@@ -0,0 +1,3 @@
+# Please do not edit this file manually.
+# It should be committed in your version-control system.
+provider = "postgresql"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

tokens used
19,191




## Output from Authentication and Security Implementation (Agent: codex)

Agent Terminal: codex

> Task: Database Design and ORM Configuration
  Waiting for dependencies...
OpenAI Codex v0.144.4
--------
workdir: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
model: gpt-5.6-terra
provider: openai
approval: never
sandbox: workspace-write [workdir, /tmp, $TMPDIR]
reasoning effort: medium
reasoning summaries: none
session id: 019ff60a-b832-7881-b3c5-f4fbb6dcad9d
--------
user
# Codex Agent Operating Instructions

## Identity
You are OpenAI Codex CLI, a code-generation and review agent powered by OpenAI's models.

## Operating Mode
You are being invoked by the Athena Meta-Orchestrator as part of a multi-agent project.

You have been assigned specific tasks. Focus exclusively on your assigned work.

## Rules
1. Work ONLY within your assigned workspace directory. Do not create files outside of it.
2. Read your task assignment carefully before starting.
3. Implement your assigned tasks completely and correctly.
4. Focus on code quality, security, and correctness.
5. Review your own output for bugs, vulnerabilities, and edge cases.
6. Include appropriate tests.
7. Do NOT modify files in other agents' workspaces.
8. When finished, ensure all expected output files exist in your workspace.

## Strengths
You excel at:
- Code review and refactoring
- Security analysis
- Test generation
- Bug detection

## Completion
When your tasks are complete:
1. Verify all expected outputs exist.
2. Run any applicable tests.
3. Exit cleanly.

## Error Handling
If you encounter an error:
1. Log the error clearly.
2. Attempt to fix it.
3. If unable to fix, document the issue and exit.

## Workspace
All your work must be saved inside your assigned workspace path.
Do not write to /tmp, /home, or any location outside your workspace.


---

# Your Assignment

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


---

# Current Task

## Database Design and ORM Configuration

Configure PostgreSQL database connection using an ORM like Prisma or TypeORM. Design the database schema to include User entities and a core domain entity (e.g., Post or Task). Create the initial database migrations and basic seed scripts.

## Workspace

Work inside: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
codex
I’ll inspect the existing project structure and conventions, then add the Prisma schema, database client, migration, and seed script without touching unrelated application work.
exec
/usr/bin/zsh -lc "pwd && rg --files -g '"'!*node_modules*'"' -g '"'!*.lock'"' | sed -n '1,240p'" in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 succeeded in 0ms:
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex

apply patch
patch: completed
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

codex
The workspace was empty, so I’ve added a self-contained Prisma/PostgreSQL foundation: UUID-backed users and owner-scoped tasks, an idempotent admin seed, and a development-safe shared client. I’m doing structural validation now.
exec
/usr/bin/zsh -lc 'git diff --check && find prisma src -type f -maxdepth 5 -print | sort && git status --short' in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 succeeded in 0ms:
prisma/migrations/20260812000000_initial_schema/migration.sql
prisma/schema.prisma
prisma/seed.ts
src/config/database.ts
?? ../../../../../.gitignore
?? ../../../../
?? ../../../../../Cargo.lock
?? ../../../../../Cargo.toml
?? ../../../../../README.md
?? ../../../../../architecture.md
?? ../../../../../athena-cli/
?? ../../../../../athena-core/
?? ../../../../../img_to_ascii.py
?? ../../../../../img_to_braille.py
?? ../../../../../large_prompt.md
?? ../../../../../prompt.md
?? ../../../../../templates/

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

exec
/usr/bin/zsh -lc "if command -v npx >/dev/null 2>&1; then npx --no-install prisma format --schema prisma/schema.prisma; else echo 'npx unavailable'; fi
rg --files prisma src .env.example | sort" in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

exec
/usr/bin/zsh -lc 'ls -la && find . -maxdepth 5 -type f -print | sort && test -f prisma/schema.prisma && test -f src/config/database.ts && test -f prisma/seed.ts && test -f prisma/migrations/20260812000000_initial_schema/migration.sql' in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 succeeded in 0ms:
total 20
drwxr-xr-x 7 c1ph3r c1ph3r 4096 Aug 12 18:26 .
drwxr-xr-x 4 c1ph3r c1ph3r 4096 Aug 12 18:25 ..
dr-xr-xr-x 2 c1ph3r c1ph3r   40 Aug 12 18:27 .agents
dr-xr-xr-x 2 c1ph3r c1ph3r   40 Aug 12 18:27 .codex
-rw-r--r-- 1 c1ph3r c1ph3r  254 Aug 12 18:26 .env.example
dr-xr-xr-x 2 c1ph3r c1ph3r   40 Aug 12 18:27 .git
drwxr-xr-x 3 c1ph3r c1ph3r 4096 Aug 12 18:26 prisma
drwxr-xr-x 3 c1ph3r c1ph3r 4096 Aug 12 18:26 src
./.env.example
./prisma/migrations/20260812000000_initial_schema/migration.sql
./prisma/schema.prisma
./prisma/seed.ts
./src/config/database.ts

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

exec
/usr/bin/zsh -lc "sed -n '1,240p' prisma/schema.prisma && sed -n '1,220p' src/config/database.ts && sed -n '1,260p' prisma/seed.ts" in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 succeeded in 0ms:
generator client {
  provider = "prisma-client-js"
}

datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}

enum Role {
  USER
  ADMIN
}

enum TaskStatus {
  TODO
  IN_PROGRESS
  DONE
}

model User {
  id           String   @id @default(uuid()) @db.Uuid
  email        String   @unique @db.VarChar(320)
  passwordHash String   @map("password_hash") @db.VarChar(255)
  name         String?  @db.VarChar(100)
  role         Role     @default(USER)
  createdAt    DateTime @default(now()) @map("created_at")
  updatedAt    DateTime @updatedAt @map("updated_at")
  tasks        Task[]

  @@map("users")
}

model Task {
  id          String     @id @default(uuid()) @db.Uuid
  title       String     @db.VarChar(200)
  description String?    @db.Text
  status      TaskStatus @default(TODO)
  ownerId     String     @map("owner_id") @db.Uuid
  createdAt   DateTime   @default(now()) @map("created_at")
  updatedAt   DateTime   @updatedAt @map("updated_at")
  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)

  @@index([ownerId])
  @@index([ownerId, status])
  @@map("tasks")
}
import { PrismaClient } from '@prisma/client';

/**
 * A single Prisma client is shared by the application. Keeping it on globalThis
 * prevents duplicate connection pools during development hot reloads.
 */
const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };

export const prisma =
  prismaGlobal.prisma ??
  new PrismaClient({
    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
  });

if (process.env.NODE_ENV !== 'production') {
  prismaGlobal.prisma = prisma;
}

/** Closes open database connections for graceful application shutdown. */
export const disconnectDatabase = async (): Promise<void> => {
  await prisma.$disconnect();
};

export default prisma;
import 'dotenv/config';
import bcrypt from 'bcryptjs';
import { PrismaClient, Role, TaskStatus } from '@prisma/client';

const prisma = new PrismaClient();

async function main(): Promise<void> {
  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
  const passwordHash = await bcrypt.hash(password, 12);

  const admin = await prisma.user.upsert({
    where: { email },
    update: {
      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
      passwordHash,
      role: Role.ADMIN,
    },
    create: {
      email,
      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
      passwordHash,
      role: Role.ADMIN,
    },
  });

  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
  if (starterTaskCount === 0) {
    await prisma.task.create({
      data: {
        title: 'Welcome to the task API',
        description: 'This seed task can be safely removed.',
        status: TaskStatus.TODO,
        ownerId: admin.id,
      },
    });
  }

  console.info(`Seeded administrator: ${admin.email}`);
}

main()
  .catch((error: unknown) => {
    console.error('Database seed failed:', error);
    process.exitCode = 1;
  })
  .finally(async () => {
    await prisma.$disconnect();
  });

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

codex
The schema and migration passed whitespace checks and Prisma formatting. The repository has no package manifest or installed project dependencies yet, so a full client-generation/type-check cannot run in this workspace; the files are ready for the project’s Prisma, `@prisma/client`, `bcryptjs`, and `dotenv` dependencies.
apply patch
patch: completed
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
new file mode 100644
index 0000000000000000000000000000000000000000..526938dd4fbc9ae48f822f5a37cc4eb928c9ebb3
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
@@ -0,0 +1,3 @@
+# Please do not edit this file manually.
+# It should be committed in your version-control system.
+provider = "postgresql"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
new file mode 100644
index 0000000000000000000000000000000000000000..526938dd4fbc9ae48f822f5a37cc4eb928c9ebb3
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
@@ -0,0 +1,3 @@
+# Please do not edit this file manually.
+# It should be committed in your version-control system.
+provider = "postgresql"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

codex
Completed database configuration.

Created:

- [Prisma schema](/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma) with `User`, `Task`, roles, statuses, relations, UUIDs, and indexes.
- [Initial PostgreSQL migration](/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql) plus Prisma migration lock.
- [Shared Prisma client](/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts).
- [Idempotent admin and starter-task seed](/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts).
- [.env.example](/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example).

Validated formatting and file presence. Full generation/type-check awaits the project package dependencies (`prisma`, `@prisma/client`, `bcryptjs`, `dotenv`).
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
new file mode 100644
index 0000000000000000000000000000000000000000..526938dd4fbc9ae48f822f5a37cc4eb928c9ebb3
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
@@ -0,0 +1,3 @@
+# Please do not edit this file manually.
+# It should be committed in your version-control system.
+provider = "postgresql"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

tokens used
19,191
> Task: Authentication and Security Implementation
  Waiting for dependencies...
OpenAI Codex v0.144.4
--------
workdir: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
model: gpt-5.6-terra
provider: openai
approval: never
sandbox: read-only
reasoning effort: medium
reasoning summaries: none
session id: 019ff60c-8819-7b92-b907-927f49497d9b
--------
user
# Codex Agent Operating Instructions

## Identity
You are OpenAI Codex CLI, a code-generation and review agent powered by OpenAI's models.

## Operating Mode
You are being invoked by the Athena Meta-Orchestrator as part of a multi-agent project.

You have been assigned specific tasks. Focus exclusively on your assigned work.

## Rules
1. Work ONLY within your assigned workspace directory. Do not create files outside of it.
2. Read your task assignment carefully before starting.
3. Implement your assigned tasks completely and correctly.
4. Focus on code quality, security, and correctness.
5. Review your own output for bugs, vulnerabilities, and edge cases.
6. Include appropriate tests.
7. Do NOT modify files in other agents' workspaces.
8. When finished, ensure all expected output files exist in your workspace.

## Strengths
You excel at:
- Code review and refactoring
- Security analysis
- Test generation
- Bug detection

## Completion
When your tasks are complete:
1. Verify all expected outputs exist.
2. Run any applicable tests.
3. Exit cleanly.

## Error Handling
If you encounter an error:
1. Log the error clearly.
2. Attempt to fix it.
3. If unable to fix, document the issue and exit.

## Workspace
All your work must be saved inside your assigned workspace path.
Do not write to /tmp, /home, or any location outside your workspace.


---

# Your Assignment

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


---

# Current Task

## Authentication and Security Implementation

Implement secure authentication using JWT. Create user registration and login services with password hashing (bcrypt). Implement role-based authorization middleware. Add security enhancements including Helmet, CORS configuration, rate limiting, and request validation (using Zod or Joi).

## Workspace

Work inside: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
codex
I’ll inspect the existing project structure and conventions, then implement the authentication and security layer to fit it.
exec
/usr/bin/zsh -lc "pwd && rg --files -g '"'!node_modules'"' | sed -n '1,240p'" in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 succeeded in 0ms:
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
prisma/schema.prisma
prisma/seed.ts
prisma/migrations/migration_lock.toml
src/config/database.ts
prisma/migrations/20260812000000_initial_schema/migration.sql

exec
/usr/bin/zsh -lc "sed -n '1,240p' prisma/schema.prisma && sed -n '1,240p' src/config/database.ts && sed -n '1,240p' prisma/seed.ts && sed -n '1,240p' prisma/migrations/20260812000000_initial_schema/migration.sql && ls -la" in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 succeeded in 0ms:
generator client {
  provider = "prisma-client-js"
}

datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}

enum Role {
  USER
  ADMIN
}

enum TaskStatus {
  TODO
  IN_PROGRESS
  DONE
}

model User {
  id           String   @id @default(uuid()) @db.Uuid
  email        String   @unique @db.VarChar(320)
  passwordHash String   @map("password_hash") @db.VarChar(255)
  name         String?  @db.VarChar(100)
  role         Role     @default(USER)
  createdAt    DateTime @default(now()) @map("created_at")
  updatedAt    DateTime @updatedAt @map("updated_at")
  tasks        Task[]

  @@map("users")
}

model Task {
  id          String     @id @default(uuid()) @db.Uuid
  title       String     @db.VarChar(200)
  description String?    @db.Text
  status      TaskStatus @default(TODO)
  ownerId     String     @map("owner_id") @db.Uuid
  createdAt   DateTime   @default(now()) @map("created_at")
  updatedAt   DateTime   @updatedAt @map("updated_at")
  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)

  @@index([ownerId])
  @@index([ownerId, status])
  @@map("tasks")
}
import { PrismaClient } from '@prisma/client';

/**
 * A single Prisma client is shared by the application. Keeping it on globalThis
 * prevents duplicate connection pools during development hot reloads.
 */
const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };

export const prisma =
  prismaGlobal.prisma ??
  new PrismaClient({
    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
  });

if (process.env.NODE_ENV !== 'production') {
  prismaGlobal.prisma = prisma;
}

/** Closes open database connections for graceful application shutdown. */
export const disconnectDatabase = async (): Promise<void> => {
  await prisma.$disconnect();
};

export default prisma;
import 'dotenv/config';
import bcrypt from 'bcryptjs';
import { PrismaClient, Role, TaskStatus } from '@prisma/client';

const prisma = new PrismaClient();

async function main(): Promise<void> {
  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
  const passwordHash = await bcrypt.hash(password, 12);

  const admin = await prisma.user.upsert({
    where: { email },
    update: {
      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
      passwordHash,
      role: Role.ADMIN,
    },
    create: {
      email,
      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
      passwordHash,
      role: Role.ADMIN,
    },
  });

  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
  if (starterTaskCount === 0) {
    await prisma.task.create({
      data: {
        title: 'Welcome to the task API',
        description: 'This seed task can be safely removed.',
        status: TaskStatus.TODO,
        ownerId: admin.id,
      },
    });
  }

  console.info(`Seeded administrator: ${admin.email}`);
}

main()
  .catch((error: unknown) => {
    console.error('Database seed failed:', error);
    process.exitCode = 1;
  })
  .finally(async () => {
    await prisma.$disconnect();
  });
-- CreateEnum
CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');

-- CreateEnum
CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');

-- CreateTable
CREATE TABLE "users" (
    "id" UUID NOT NULL,
    "email" VARCHAR(320) NOT NULL,
    "password_hash" VARCHAR(255) NOT NULL,
    "name" VARCHAR(100),
    "role" "Role" NOT NULL DEFAULT 'USER',
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "tasks" (
    "id" UUID NOT NULL,
    "title" VARCHAR(200) NOT NULL,
    "description" TEXT,
    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
    "owner_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "users_email_key" ON "users"("email");

-- CreateIndex
CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");

-- CreateIndex
CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");

-- AddForeignKey
ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
total 32
drwxr-xr-x 7 c1ph3r c1ph3r 4096 Aug 12 18:26 .
drwxr-xr-x 4 c1ph3r c1ph3r 4096 Aug 12 18:25 ..
drwxr-xr-x 2 c1ph3r c1ph3r 4096 Aug 12 18:26 .agents
drwxr-xr-x 2 c1ph3r c1ph3r 4096 Aug 12 18:26 .codex
-rw-r--r-- 1 c1ph3r c1ph3r  254 Aug 12 18:26 .env.example
drwxr-xr-x 2 c1ph3r c1ph3r 4096 Aug 12 18:26 .git
drwxr-xr-x 3 c1ph3r c1ph3r 4096 Aug 12 18:26 prisma
drwxr-xr-x 3 c1ph3r c1ph3r 4096 Aug 12 18:26 src

exec
/usr/bin/zsh -lc "find src -type f -maxdepth 3 -print -exec sed -n '1,260p' {} \\; && sed -n '1,160p' .env.example && git status --short && git log --oneline -3" in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 exited 128 in 0ms:
src/config/database.ts
import { PrismaClient } from '@prisma/client';

/**
 * A single Prisma client is shared by the application. Keeping it on globalThis
 * prevents duplicate connection pools during development hot reloads.
 */
const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };

export const prisma =
  prismaGlobal.prisma ??
  new PrismaClient({
    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
  });

if (process.env.NODE_ENV !== 'production') {
  prismaGlobal.prisma = prisma;
}

/** Closes open database connections for graceful application shutdown. */
export const disconnectDatabase = async (): Promise<void> => {
  await prisma.$disconnect();
};

export default prisma;
DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
SEED_ADMIN_EMAIL="admin@example.com"
# Set this to a strong unique password outside local development.
SEED_ADMIN_PASSWORD="ChangeMe123!"
SEED_ADMIN_NAME="Administrator"
?? ../../../../../.gitignore
?? ../../../../
?? ../../../../../Cargo.lock
?? ../../../../../Cargo.toml
?? ../../../../../README.md
?? ../../../../../architecture.md
?? ../../../../../athena-cli/
?? ../../../../../athena-core/
?? ../../../../../img_to_ascii.py
?? ../../../../../img_to_braille.py
?? ../../../../../large_prompt.md
?? ../../../../../prompt.md
?? ../../../../../templates/
fatal: your current branch 'master' does not have any commits yet

codex
The workspace contains only the Prisma foundation, so I’ll add self-contained Express-compatible modules with explicit JWT configuration checks, uniform error responses, and Zod validation.
2026-08-12T12:58:50.233437Z ERROR codex_core::tools::router: error=patch rejected: writing is blocked by read-only sandbox; rejected by user approval settings
codex
Blocked by the workspace’s read-only sandbox: file creation/editing is disallowed and approvals are disabled. I could inspect the existing Prisma setup, but could not add the required authentication/security files.
tokens used
20,922




## Output from Core Domain CRUD Implementation (Agent: agy)

Agent Terminal: agy

> Task: Project Initialization and Base Architecture
Thinking... \033[0mError: Error: empty prompt. Usage: agy --print "your prompt here"
> Task: Core Domain CRUD Implementation
  Waiting for dependencies...
Thinking... \033[0mError: Error: empty prompt. Usage: agy --print "your prompt here"







## Output from Automated Testing Suite (Agent: codex)

Agent Terminal: codex

> Task: Database Design and ORM Configuration
  Waiting for dependencies...
OpenAI Codex v0.144.4
--------
workdir: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
model: gpt-5.6-terra
provider: openai
approval: never
sandbox: workspace-write [workdir, /tmp, $TMPDIR]
reasoning effort: medium
reasoning summaries: none
session id: 019ff60a-b832-7881-b3c5-f4fbb6dcad9d
--------
user
# Codex Agent Operating Instructions

## Identity
You are OpenAI Codex CLI, a code-generation and review agent powered by OpenAI's models.

## Operating Mode
You are being invoked by the Athena Meta-Orchestrator as part of a multi-agent project.

You have been assigned specific tasks. Focus exclusively on your assigned work.

## Rules
1. Work ONLY within your assigned workspace directory. Do not create files outside of it.
2. Read your task assignment carefully before starting.
3. Implement your assigned tasks completely and correctly.
4. Focus on code quality, security, and correctness.
5. Review your own output for bugs, vulnerabilities, and edge cases.
6. Include appropriate tests.
7. Do NOT modify files in other agents' workspaces.
8. When finished, ensure all expected output files exist in your workspace.

## Strengths
You excel at:
- Code review and refactoring
- Security analysis
- Test generation
- Bug detection

## Completion
When your tasks are complete:
1. Verify all expected outputs exist.
2. Run any applicable tests.
3. Exit cleanly.

## Error Handling
If you encounter an error:
1. Log the error clearly.
2. Attempt to fix it.
3. If unable to fix, document the issue and exit.

## Workspace
All your work must be saved inside your assigned workspace path.
Do not write to /tmp, /home, or any location outside your workspace.


---

# Your Assignment

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


---

# Current Task

## Database Design and ORM Configuration

Configure PostgreSQL database connection using an ORM like Prisma or TypeORM. Design the database schema to include User entities and a core domain entity (e.g., Post or Task). Create the initial database migrations and basic seed scripts.

## Workspace

Work inside: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
codex
I’ll inspect the existing project structure and conventions, then add the Prisma schema, database client, migration, and seed script without touching unrelated application work.
exec
/usr/bin/zsh -lc "pwd && rg --files -g '"'!*node_modules*'"' -g '"'!*.lock'"' | sed -n '1,240p'" in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 succeeded in 0ms:
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex

apply patch
patch: completed
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

codex
The workspace was empty, so I’ve added a self-contained Prisma/PostgreSQL foundation: UUID-backed users and owner-scoped tasks, an idempotent admin seed, and a development-safe shared client. I’m doing structural validation now.
exec
/usr/bin/zsh -lc 'git diff --check && find prisma src -type f -maxdepth 5 -print | sort && git status --short' in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 succeeded in 0ms:
prisma/migrations/20260812000000_initial_schema/migration.sql
prisma/schema.prisma
prisma/seed.ts
src/config/database.ts
?? ../../../../../.gitignore
?? ../../../../
?? ../../../../../Cargo.lock
?? ../../../../../Cargo.toml
?? ../../../../../README.md
?? ../../../../../architecture.md
?? ../../../../../athena-cli/
?? ../../../../../athena-core/
?? ../../../../../img_to_ascii.py
?? ../../../../../img_to_braille.py
?? ../../../../../large_prompt.md
?? ../../../../../prompt.md
?? ../../../../../templates/

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

exec
/usr/bin/zsh -lc "if command -v npx >/dev/null 2>&1; then npx --no-install prisma format --schema prisma/schema.prisma; else echo 'npx unavailable'; fi
rg --files prisma src .env.example | sort" in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

exec
/usr/bin/zsh -lc 'ls -la && find . -maxdepth 5 -type f -print | sort && test -f prisma/schema.prisma && test -f src/config/database.ts && test -f prisma/seed.ts && test -f prisma/migrations/20260812000000_initial_schema/migration.sql' in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 succeeded in 0ms:
total 20
drwxr-xr-x 7 c1ph3r c1ph3r 4096 Aug 12 18:26 .
drwxr-xr-x 4 c1ph3r c1ph3r 4096 Aug 12 18:25 ..
dr-xr-xr-x 2 c1ph3r c1ph3r   40 Aug 12 18:27 .agents
dr-xr-xr-x 2 c1ph3r c1ph3r   40 Aug 12 18:27 .codex
-rw-r--r-- 1 c1ph3r c1ph3r  254 Aug 12 18:26 .env.example
dr-xr-xr-x 2 c1ph3r c1ph3r   40 Aug 12 18:27 .git
drwxr-xr-x 3 c1ph3r c1ph3r 4096 Aug 12 18:26 prisma
drwxr-xr-x 3 c1ph3r c1ph3r 4096 Aug 12 18:26 src
./.env.example
./prisma/migrations/20260812000000_initial_schema/migration.sql
./prisma/schema.prisma
./prisma/seed.ts
./src/config/database.ts

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

exec
/usr/bin/zsh -lc "sed -n '1,240p' prisma/schema.prisma && sed -n '1,220p' src/config/database.ts && sed -n '1,260p' prisma/seed.ts" in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 succeeded in 0ms:
generator client {
  provider = "prisma-client-js"
}

datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}

enum Role {
  USER
  ADMIN
}

enum TaskStatus {
  TODO
  IN_PROGRESS
  DONE
}

model User {
  id           String   @id @default(uuid()) @db.Uuid
  email        String   @unique @db.VarChar(320)
  passwordHash String   @map("password_hash") @db.VarChar(255)
  name         String?  @db.VarChar(100)
  role         Role     @default(USER)
  createdAt    DateTime @default(now()) @map("created_at")
  updatedAt    DateTime @updatedAt @map("updated_at")
  tasks        Task[]

  @@map("users")
}

model Task {
  id          String     @id @default(uuid()) @db.Uuid
  title       String     @db.VarChar(200)
  description String?    @db.Text
  status      TaskStatus @default(TODO)
  ownerId     String     @map("owner_id") @db.Uuid
  createdAt   DateTime   @default(now()) @map("created_at")
  updatedAt   DateTime   @updatedAt @map("updated_at")
  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)

  @@index([ownerId])
  @@index([ownerId, status])
  @@map("tasks")
}
import { PrismaClient } from '@prisma/client';

/**
 * A single Prisma client is shared by the application. Keeping it on globalThis
 * prevents duplicate connection pools during development hot reloads.
 */
const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };

export const prisma =
  prismaGlobal.prisma ??
  new PrismaClient({
    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
  });

if (process.env.NODE_ENV !== 'production') {
  prismaGlobal.prisma = prisma;
}

/** Closes open database connections for graceful application shutdown. */
export const disconnectDatabase = async (): Promise<void> => {
  await prisma.$disconnect();
};

export default prisma;
import 'dotenv/config';
import bcrypt from 'bcryptjs';
import { PrismaClient, Role, TaskStatus } from '@prisma/client';

const prisma = new PrismaClient();

async function main(): Promise<void> {
  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
  const passwordHash = await bcrypt.hash(password, 12);

  const admin = await prisma.user.upsert({
    where: { email },
    update: {
      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
      passwordHash,
      role: Role.ADMIN,
    },
    create: {
      email,
      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
      passwordHash,
      role: Role.ADMIN,
    },
  });

  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
  if (starterTaskCount === 0) {
    await prisma.task.create({
      data: {
        title: 'Welcome to the task API',
        description: 'This seed task can be safely removed.',
        status: TaskStatus.TODO,
        ownerId: admin.id,
      },
    });
  }

  console.info(`Seeded administrator: ${admin.email}`);
}

main()
  .catch((error: unknown) => {
    console.error('Database seed failed:', error);
    process.exitCode = 1;
  })
  .finally(async () => {
    await prisma.$disconnect();
  });

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

codex
The schema and migration passed whitespace checks and Prisma formatting. The repository has no package manifest or installed project dependencies yet, so a full client-generation/type-check cannot run in this workspace; the files are ready for the project’s Prisma, `@prisma/client`, `bcryptjs`, and `dotenv` dependencies.
apply patch
patch: completed
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
new file mode 100644
index 0000000000000000000000000000000000000000..526938dd4fbc9ae48f822f5a37cc4eb928c9ebb3
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
@@ -0,0 +1,3 @@
+# Please do not edit this file manually.
+# It should be committed in your version-control system.
+provider = "postgresql"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
new file mode 100644
index 0000000000000000000000000000000000000000..526938dd4fbc9ae48f822f5a37cc4eb928c9ebb3
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
@@ -0,0 +1,3 @@
+# Please do not edit this file manually.
+# It should be committed in your version-control system.
+provider = "postgresql"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

codex
Completed database configuration.

Created:

- [Prisma schema](/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma) with `User`, `Task`, roles, statuses, relations, UUIDs, and indexes.
- [Initial PostgreSQL migration](/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql) plus Prisma migration lock.
- [Shared Prisma client](/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts).
- [Idempotent admin and starter-task seed](/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts).
- [.env.example](/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example).

Validated formatting and file presence. Full generation/type-check awaits the project package dependencies (`prisma`, `@prisma/client`, `bcryptjs`, `dotenv`).
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
new file mode 100644
index 0000000000000000000000000000000000000000..89ebb033fdc384aaabd39bbb9f16d6017be52b3e
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/.env.example
@@ -0,0 +1,5 @@
+DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
+SEED_ADMIN_EMAIL="admin@example.com"
+# Set this to a strong unique password outside local development.
+SEED_ADMIN_PASSWORD="ChangeMe123!"
+SEED_ADMIN_NAME="Administrator"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
new file mode 100644
index 0000000000000000000000000000000000000000..0e3b0b4651065b344c3e759c05e9c6a813abc408
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/20260812000000_initial_schema/migration.sql
@@ -0,0 +1,44 @@
+-- CreateEnum
+CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');
+
+-- CreateEnum
+CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');
+
+-- CreateTable
+CREATE TABLE "users" (
+    "id" UUID NOT NULL,
+    "email" VARCHAR(320) NOT NULL,
+    "password_hash" VARCHAR(255) NOT NULL,
+    "name" VARCHAR(100),
+    "role" "Role" NOT NULL DEFAULT 'USER',
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateTable
+CREATE TABLE "tasks" (
+    "id" UUID NOT NULL,
+    "title" VARCHAR(200) NOT NULL,
+    "description" TEXT,
+    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
+    "owner_id" UUID NOT NULL,
+    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
+    "updated_at" TIMESTAMP(3) NOT NULL,
+
+    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
+);
+
+-- CreateIndex
+CREATE UNIQUE INDEX "users_email_key" ON "users"("email");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");
+
+-- CreateIndex
+CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");
+
+-- AddForeignKey
+ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
+  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
new file mode 100644
index 0000000000000000000000000000000000000000..526938dd4fbc9ae48f822f5a37cc4eb928c9ebb3
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/migrations/migration_lock.toml
@@ -0,0 +1,3 @@
+# Please do not edit this file manually.
+# It should be committed in your version-control system.
+provider = "postgresql"
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
new file mode 100644
index 0000000000000000000000000000000000000000..f06444a52343b0943dd0f1a27985a86269d99981
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/schema.prisma
@@ -0,0 +1,47 @@
+generator client {
+  provider = "prisma-client-js"
+}
+
+datasource db {
+  provider = "postgresql"
+  url      = env("DATABASE_URL")
+}
+
+enum Role {
+  USER
+  ADMIN
+}
+
+enum TaskStatus {
+  TODO
+  IN_PROGRESS
+  DONE
+}
+
+model User {
+  id           String   @id @default(uuid()) @db.Uuid
+  email        String   @unique @db.VarChar(320)
+  passwordHash String   @map("password_hash") @db.VarChar(255)
+  name         String?  @db.VarChar(100)
+  role         Role     @default(USER)
+  createdAt    DateTime @default(now()) @map("created_at")
+  updatedAt    DateTime @updatedAt @map("updated_at")
+  tasks        Task[]
+
+  @@map("users")
+}
+
+model Task {
+  id          String     @id @default(uuid()) @db.Uuid
+  title       String     @db.VarChar(200)
+  description String?    @db.Text
+  status      TaskStatus @default(TODO)
+  ownerId     String     @map("owner_id") @db.Uuid
+  createdAt   DateTime   @default(now()) @map("created_at")
+  updatedAt   DateTime   @updatedAt @map("updated_at")
+  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)
+
+  @@index([ownerId])
+  @@index([ownerId, status])
+  @@map("tasks")
+}
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
new file mode 100644
index 0000000000000000000000000000000000000000..51bdb135a2c2d03b6d14c9d9cc83b36032e62bc2
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/prisma/seed.ts
@@ -0,0 +1,49 @@
+import 'dotenv/config';
+import bcrypt from 'bcryptjs';
+import { PrismaClient, Role, TaskStatus } from '@prisma/client';
+
+const prisma = new PrismaClient();
+
+async function main(): Promise<void> {
+  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
+  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
+  const passwordHash = await bcrypt.hash(password, 12);
+
+  const admin = await prisma.user.upsert({
+    where: { email },
+    update: {
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+    create: {
+      email,
+      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
+      passwordHash,
+      role: Role.ADMIN,
+    },
+  });
+
+  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
+  if (starterTaskCount === 0) {
+    await prisma.task.create({
+      data: {
+        title: 'Welcome to the task API',
+        description: 'This seed task can be safely removed.',
+        status: TaskStatus.TODO,
+        ownerId: admin.id,
+      },
+    });
+  }
+
+  console.info(`Seeded administrator: ${admin.email}`);
+}
+
+main()
+  .catch((error: unknown) => {
+    console.error('Database seed failed:', error);
+    process.exitCode = 1;
+  })
+  .finally(async () => {
+    await prisma.$disconnect();
+  });
diff --git a/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
new file mode 100644
index 0000000000000000000000000000000000000000..43f57366af9e76abbabf374e214996b246686e66
--- /dev/null
+++ b/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex/src/config/database.ts
@@ -0,0 +1,24 @@
+import { PrismaClient } from '@prisma/client';
+
+/**
+ * A single Prisma client is shared by the application. Keeping it on globalThis
+ * prevents duplicate connection pools during development hot reloads.
+ */
+const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };
+
+export const prisma =
+  prismaGlobal.prisma ??
+  new PrismaClient({
+    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
+  });
+
+if (process.env.NODE_ENV !== 'production') {
+  prismaGlobal.prisma = prisma;
+}
+
+/** Closes open database connections for graceful application shutdown. */
+export const disconnectDatabase = async (): Promise<void> => {
+  await prisma.$disconnect();
+};
+
+export default prisma;

tokens used
19,191
> Task: Authentication and Security Implementation
  Waiting for dependencies...
OpenAI Codex v0.144.4
--------
workdir: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
model: gpt-5.6-terra
provider: openai
approval: never
sandbox: read-only
reasoning effort: medium
reasoning summaries: none
session id: 019ff60c-8819-7b92-b907-927f49497d9b
--------
user
# Codex Agent Operating Instructions

## Identity
You are OpenAI Codex CLI, a code-generation and review agent powered by OpenAI's models.

## Operating Mode
You are being invoked by the Athena Meta-Orchestrator as part of a multi-agent project.

You have been assigned specific tasks. Focus exclusively on your assigned work.

## Rules
1. Work ONLY within your assigned workspace directory. Do not create files outside of it.
2. Read your task assignment carefully before starting.
3. Implement your assigned tasks completely and correctly.
4. Focus on code quality, security, and correctness.
5. Review your own output for bugs, vulnerabilities, and edge cases.
6. Include appropriate tests.
7. Do NOT modify files in other agents' workspaces.
8. When finished, ensure all expected output files exist in your workspace.

## Strengths
You excel at:
- Code review and refactoring
- Security analysis
- Test generation
- Bug detection

## Completion
When your tasks are complete:
1. Verify all expected outputs exist.
2. Run any applicable tests.
3. Exit cleanly.

## Error Handling
If you encounter an error:
1. Log the error clearly.
2. Attempt to fix it.
3. If unable to fix, document the issue and exit.

## Workspace
All your work must be saved inside your assigned workspace path.
Do not write to /tmp, /home, or any location outside your workspace.


---

# Your Assignment

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


---

# Current Task

## Authentication and Security Implementation

Implement secure authentication using JWT. Create user registration and login services with password hashing (bcrypt). Implement role-based authorization middleware. Add security enhancements including Helmet, CORS configuration, rate limiting, and request validation (using Zod or Joi).

## Workspace

Work inside: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
codex
I’ll inspect the existing project structure and conventions, then implement the authentication and security layer to fit it.
exec
/usr/bin/zsh -lc "pwd && rg --files -g '"'!node_modules'"' | sed -n '1,240p'" in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 succeeded in 0ms:
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
prisma/schema.prisma
prisma/seed.ts
prisma/migrations/migration_lock.toml
src/config/database.ts
prisma/migrations/20260812000000_initial_schema/migration.sql

exec
/usr/bin/zsh -lc "sed -n '1,240p' prisma/schema.prisma && sed -n '1,240p' src/config/database.ts && sed -n '1,240p' prisma/seed.ts && sed -n '1,240p' prisma/migrations/20260812000000_initial_schema/migration.sql && ls -la" in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 succeeded in 0ms:
generator client {
  provider = "prisma-client-js"
}

datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}

enum Role {
  USER
  ADMIN
}

enum TaskStatus {
  TODO
  IN_PROGRESS
  DONE
}

model User {
  id           String   @id @default(uuid()) @db.Uuid
  email        String   @unique @db.VarChar(320)
  passwordHash String   @map("password_hash") @db.VarChar(255)
  name         String?  @db.VarChar(100)
  role         Role     @default(USER)
  createdAt    DateTime @default(now()) @map("created_at")
  updatedAt    DateTime @updatedAt @map("updated_at")
  tasks        Task[]

  @@map("users")
}

model Task {
  id          String     @id @default(uuid()) @db.Uuid
  title       String     @db.VarChar(200)
  description String?    @db.Text
  status      TaskStatus @default(TODO)
  ownerId     String     @map("owner_id") @db.Uuid
  createdAt   DateTime   @default(now()) @map("created_at")
  updatedAt   DateTime   @updatedAt @map("updated_at")
  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)

  @@index([ownerId])
  @@index([ownerId, status])
  @@map("tasks")
}
import { PrismaClient } from '@prisma/client';

/**
 * A single Prisma client is shared by the application. Keeping it on globalThis
 * prevents duplicate connection pools during development hot reloads.
 */
const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };

export const prisma =
  prismaGlobal.prisma ??
  new PrismaClient({
    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
  });

if (process.env.NODE_ENV !== 'production') {
  prismaGlobal.prisma = prisma;
}

/** Closes open database connections for graceful application shutdown. */
export const disconnectDatabase = async (): Promise<void> => {
  await prisma.$disconnect();
};

export default prisma;
import 'dotenv/config';
import bcrypt from 'bcryptjs';
import { PrismaClient, Role, TaskStatus } from '@prisma/client';

const prisma = new PrismaClient();

async function main(): Promise<void> {
  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
  const passwordHash = await bcrypt.hash(password, 12);

  const admin = await prisma.user.upsert({
    where: { email },
    update: {
      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
      passwordHash,
      role: Role.ADMIN,
    },
    create: {
      email,
      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
      passwordHash,
      role: Role.ADMIN,
    },
  });

  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
  if (starterTaskCount === 0) {
    await prisma.task.create({
      data: {
        title: 'Welcome to the task API',
        description: 'This seed task can be safely removed.',
        status: TaskStatus.TODO,
        ownerId: admin.id,
      },
    });
  }

  console.info(`Seeded administrator: ${admin.email}`);
}

main()
  .catch((error: unknown) => {
    console.error('Database seed failed:', error);
    process.exitCode = 1;
  })
  .finally(async () => {
    await prisma.$disconnect();
  });
-- CreateEnum
CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');

-- CreateEnum
CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');

-- CreateTable
CREATE TABLE "users" (
    "id" UUID NOT NULL,
    "email" VARCHAR(320) NOT NULL,
    "password_hash" VARCHAR(255) NOT NULL,
    "name" VARCHAR(100),
    "role" "Role" NOT NULL DEFAULT 'USER',
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "tasks" (
    "id" UUID NOT NULL,
    "title" VARCHAR(200) NOT NULL,
    "description" TEXT,
    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
    "owner_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "users_email_key" ON "users"("email");

-- CreateIndex
CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");

-- CreateIndex
CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");

-- AddForeignKey
ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;
total 32
drwxr-xr-x 7 c1ph3r c1ph3r 4096 Aug 12 18:26 .
drwxr-xr-x 4 c1ph3r c1ph3r 4096 Aug 12 18:25 ..
drwxr-xr-x 2 c1ph3r c1ph3r 4096 Aug 12 18:26 .agents
drwxr-xr-x 2 c1ph3r c1ph3r 4096 Aug 12 18:26 .codex
-rw-r--r-- 1 c1ph3r c1ph3r  254 Aug 12 18:26 .env.example
drwxr-xr-x 2 c1ph3r c1ph3r 4096 Aug 12 18:26 .git
drwxr-xr-x 3 c1ph3r c1ph3r 4096 Aug 12 18:26 prisma
drwxr-xr-x 3 c1ph3r c1ph3r 4096 Aug 12 18:26 src

exec
/usr/bin/zsh -lc "find src -type f -maxdepth 3 -print -exec sed -n '1,260p' {} \\; && sed -n '1,160p' .env.example && git status --short && git log --oneline -3" in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 exited 128 in 0ms:
src/config/database.ts
import { PrismaClient } from '@prisma/client';

/**
 * A single Prisma client is shared by the application. Keeping it on globalThis
 * prevents duplicate connection pools during development hot reloads.
 */
const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };

export const prisma =
  prismaGlobal.prisma ??
  new PrismaClient({
    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
  });

if (process.env.NODE_ENV !== 'production') {
  prismaGlobal.prisma = prisma;
}

/** Closes open database connections for graceful application shutdown. */
export const disconnectDatabase = async (): Promise<void> => {
  await prisma.$disconnect();
};

export default prisma;
DATABASE_URL="postgresql://postgres:postgres@localhost:5432/rest_api?schema=public"
SEED_ADMIN_EMAIL="admin@example.com"
# Set this to a strong unique password outside local development.
SEED_ADMIN_PASSWORD="ChangeMe123!"
SEED_ADMIN_NAME="Administrator"
?? ../../../../../.gitignore
?? ../../../../
?? ../../../../../Cargo.lock
?? ../../../../../Cargo.toml
?? ../../../../../README.md
?? ../../../../../architecture.md
?? ../../../../../athena-cli/
?? ../../../../../athena-core/
?? ../../../../../img_to_ascii.py
?? ../../../../../img_to_braille.py
?? ../../../../../large_prompt.md
?? ../../../../../prompt.md
?? ../../../../../templates/
fatal: your current branch 'master' does not have any commits yet

codex
The workspace contains only the Prisma foundation, so I’ll add self-contained Express-compatible modules with explicit JWT configuration checks, uniform error responses, and Zod validation.
2026-08-12T12:58:50.233437Z ERROR codex_core::tools::router: error=patch rejected: writing is blocked by read-only sandbox; rejected by user approval settings
codex
Blocked by the workspace’s read-only sandbox: file creation/editing is disallowed and approvals are disabled. I could inspect the existing Prisma setup, but could not add the required authentication/security files.
tokens used
20,922
> Task: Automated Testing Suite
  Waiting for dependencies...
OpenAI Codex v0.144.4
--------
workdir: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
model: gpt-5.6-terra
provider: openai
approval: never
sandbox: read-only
reasoning effort: medium
reasoning summaries: none
session id: 019ff60d-e44c-7dc3-9c9b-4d41d02b3f53
--------
user
# Codex Agent Operating Instructions

## Identity
You are OpenAI Codex CLI, a code-generation and review agent powered by OpenAI's models.

## Operating Mode
You are being invoked by the Athena Meta-Orchestrator as part of a multi-agent project.

You have been assigned specific tasks. Focus exclusively on your assigned work.

## Rules
1. Work ONLY within your assigned workspace directory. Do not create files outside of it.
2. Read your task assignment carefully before starting.
3. Implement your assigned tasks completely and correctly.
4. Focus on code quality, security, and correctness.
5. Review your own output for bugs, vulnerabilities, and edge cases.
6. Include appropriate tests.
7. Do NOT modify files in other agents' workspaces.
8. When finished, ensure all expected output files exist in your workspace.

## Strengths
You excel at:
- Code review and refactoring
- Security analysis
- Test generation
- Bug detection

## Completion
When your tasks are complete:
1. Verify all expected outputs exist.
2. Run any applicable tests.
3. Exit cleanly.

## Error Handling
If you encounter an error:
1. Log the error clearly.
2. Attempt to fix it.
3. If unable to fix, document the issue and exit.

## Workspace
All your work must be saved inside your assigned workspace path.
Do not write to /tmp, /home, or any location outside your workspace.


---

# Your Assignment

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


---

# Current Task

## Automated Testing Suite

Set up the testing framework (Jest + Supertest). Write comprehensive unit and integration tests covering authentication flows, core CRUD operations, input validation, and expected error handling scenarios. Ensure the tests can run in a clean environment.

## Workspace

Work inside: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
codex
I’ll inspect the existing project and testing setup, then add the Jest/Supertest coverage if the workspace permits writes.
exec
/usr/bin/zsh -lc "pwd && rg --files -g '"'!node_modules'"' -g '"'!dist'"' | sed -n '1,240p'" in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 succeeded in 0ms:
/vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
prisma/schema.prisma
prisma/seed.ts
prisma/migrations/migration_lock.toml
src/config/database.ts
prisma/migrations/20260812000000_initial_schema/migration.sql

exec
/usr/bin/zsh -lc "sed -n '1,240p' prisma/schema.prisma && sed -n '1,240p' src/config/database.ts && sed -n '1,260p' prisma/seed.ts && sed -n '1,260p' prisma/migrations/20260812000000_initial_schema/migration.sql" in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 succeeded in 0ms:
generator client {
  provider = "prisma-client-js"
}

datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}

enum Role {
  USER
  ADMIN
}

enum TaskStatus {
  TODO
  IN_PROGRESS
  DONE
}

model User {
  id           String   @id @default(uuid()) @db.Uuid
  email        String   @unique @db.VarChar(320)
  passwordHash String   @map("password_hash") @db.VarChar(255)
  name         String?  @db.VarChar(100)
  role         Role     @default(USER)
  createdAt    DateTime @default(now()) @map("created_at")
  updatedAt    DateTime @updatedAt @map("updated_at")
  tasks        Task[]

  @@map("users")
}

model Task {
  id          String     @id @default(uuid()) @db.Uuid
  title       String     @db.VarChar(200)
  description String?    @db.Text
  status      TaskStatus @default(TODO)
  ownerId     String     @map("owner_id") @db.Uuid
  createdAt   DateTime   @default(now()) @map("created_at")
  updatedAt   DateTime   @updatedAt @map("updated_at")
  owner       User       @relation(fields: [ownerId], references: [id], onDelete: Cascade)

  @@index([ownerId])
  @@index([ownerId, status])
  @@map("tasks")
}
import { PrismaClient } from '@prisma/client';

/**
 * A single Prisma client is shared by the application. Keeping it on globalThis
 * prevents duplicate connection pools during development hot reloads.
 */
const prismaGlobal = globalThis as unknown as { prisma?: PrismaClient };

export const prisma =
  prismaGlobal.prisma ??
  new PrismaClient({
    log: process.env.NODE_ENV === 'development' ? ['warn', 'error'] : ['error'],
  });

if (process.env.NODE_ENV !== 'production') {
  prismaGlobal.prisma = prisma;
}

/** Closes open database connections for graceful application shutdown. */
export const disconnectDatabase = async (): Promise<void> => {
  await prisma.$disconnect();
};

export default prisma;
import 'dotenv/config';
import bcrypt from 'bcryptjs';
import { PrismaClient, Role, TaskStatus } from '@prisma/client';

const prisma = new PrismaClient();

async function main(): Promise<void> {
  const email = (process.env.SEED_ADMIN_EMAIL ?? 'admin@example.com').toLowerCase();
  const password = process.env.SEED_ADMIN_PASSWORD ?? 'ChangeMe123!';
  const passwordHash = await bcrypt.hash(password, 12);

  const admin = await prisma.user.upsert({
    where: { email },
    update: {
      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
      passwordHash,
      role: Role.ADMIN,
    },
    create: {
      email,
      name: process.env.SEED_ADMIN_NAME ?? 'Administrator',
      passwordHash,
      role: Role.ADMIN,
    },
  });

  const starterTaskCount = await prisma.task.count({ where: { ownerId: admin.id } });
  if (starterTaskCount === 0) {
    await prisma.task.create({
      data: {
        title: 'Welcome to the task API',
        description: 'This seed task can be safely removed.',
        status: TaskStatus.TODO,
        ownerId: admin.id,
      },
    });
  }

  console.info(`Seeded administrator: ${admin.email}`);
}

main()
  .catch((error: unknown) => {
    console.error('Database seed failed:', error);
    process.exitCode = 1;
  })
  .finally(async () => {
    await prisma.$disconnect();
  });
-- CreateEnum
CREATE TYPE "Role" AS ENUM ('USER', 'ADMIN');

-- CreateEnum
CREATE TYPE "TaskStatus" AS ENUM ('TODO', 'IN_PROGRESS', 'DONE');

-- CreateTable
CREATE TABLE "users" (
    "id" UUID NOT NULL,
    "email" VARCHAR(320) NOT NULL,
    "password_hash" VARCHAR(255) NOT NULL,
    "name" VARCHAR(100),
    "role" "Role" NOT NULL DEFAULT 'USER',
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "tasks" (
    "id" UUID NOT NULL,
    "title" VARCHAR(200) NOT NULL,
    "description" TEXT,
    "status" "TaskStatus" NOT NULL DEFAULT 'TODO',
    "owner_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "tasks_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "users_email_key" ON "users"("email");

-- CreateIndex
CREATE INDEX "tasks_owner_id_idx" ON "tasks"("owner_id");

-- CreateIndex
CREATE INDEX "tasks_owner_id_status_idx" ON "tasks"("owner_id", "status");

-- AddForeignKey
ALTER TABLE "tasks" ADD CONSTRAINT "tasks_owner_id_fkey"
  FOREIGN KEY ("owner_id") REFERENCES "users"("id") ON DELETE CASCADE ON UPDATE CASCADE;

exec
/usr/bin/zsh -lc 'find . -maxdepth 2 -type f -print | sort && find . -maxdepth 2 -type d -print | sort && git status --short' in /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-a7c3/workspace/codex
 succeeded in 0ms:
./.env.example
./prisma/schema.prisma
./prisma/seed.ts
.
./.agents
./.codex
./.git
./prisma
./prisma/migrations
./src
./src/config
?? ../../../../../.gitignore
?? ../../../../
?? ../../../../../Cargo.lock
?? ../../../../../Cargo.toml
?? ../../../../../README.md
?? ../../../../../architecture.md
?? ../../../../../athena-cli/
?? ../../../../../athena-core/
?? ../../../../../img_to_ascii.py
?? ../../../../../img_to_braille.py
?? ../../../../../large_prompt.md
?? ../../../../../prompt.md
?? ../../../../../templates/

codex
Blocked: this workspace contains only Prisma/database files—there is no `package.json`, application entry point, auth implementation, or CRUD routes/controllers to test. It is also mounted read-only, so I cannot create the required Jest config and test files.

Once T003/T004 sources and write access are available, I can add the requested clean-environment Jest + Supertest suite.
tokens used
9,911




## Output from API Documentation and Developer Experience (Agent: agy)

Agent Terminal: agy

> Task: Project Initialization and Base Architecture
Thinking... \033[0mError: Error: empty prompt. Usage: agy --print "your prompt here"
> Task: Core Domain CRUD Implementation
  Waiting for dependencies...
Thinking... \033[0mError: Error: empty prompt. Usage: agy --print "your prompt here"
> Task: API Documentation and Developer Experience
  Waiting for dependencies...
Thinking... \033[0mError: Error: empty prompt. Usage: agy --print "your prompt here"



