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
