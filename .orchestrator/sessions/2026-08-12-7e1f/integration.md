# Integration Instructions for Project: Build_a_lightweight_Markdown

## Original Project Requirements Summary
Integrate the outputs of all agents to produce the final deliverable.

## Agent Workspaces

- **codex**: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-7e1f/workspace/codex
- **agy**: /vault/Projects/Athena/.orchestrator/sessions/2026-08-12-7e1f/workspace/agy

## Assigned Work

- **Task T001** (Build Core Parser Module) assigned to: codex
- **Task T002** (Develop CLI Wrapper) assigned to: agy
- **Task T003** (Implement Test Suite) assigned to: agy

## Dependencies Between Work

- Task T002 depends on: T001
- Task T003 depends on: T001

## Artifacts Produced

- From Task T001:
  - parser.py
- From Task T002:
  - cli.py
- From Task T003:
  - test_parser.py

## Integration Requirements
1. Verify all dependencies are met before merging code.
2. Ensure cross-agent functionality works as intended.

## Validation Requirements
1. Final build completes successfully.
2. Integration tests pass.
