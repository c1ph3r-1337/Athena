# Orchestration Plan

## Tasks: 3

### T001 - Build Core Parser Module
- **Agent**: codex
- **Dependencies**: None
- **Description**: Develop a Python module containing the core logic to convert basic Markdown elements (headers, bold, italics, and lists) into corresponding HTML strings. The module should expose a clean API (e.g., a `parse_markdown` function) that takes a Markdown string and returns an HTML string.

### T002 - Develop CLI Wrapper
- **Agent**: agy
- **Dependencies**: T001
- **Description**: Create a Python script using the `argparse` module that serves as a command-line interface. It should accept an input `.md` file path and an optional output `.html` file path, read the input file, pass its contents to the core parser module, and write the resulting HTML to the output file.

### T003 - Implement Test Suite
- **Agent**: agy
- **Dependencies**: T001
- **Description**: Write a comprehensive test suite using `pytest` to verify the accuracy of the Markdown conversions performed by the core parser module. The tests should cover all supported Markdown elements (headers, bold, italics, lists) and handle edge cases gracefully.

## Available Agents

- **agy** (/home/c1ph3r/.local/bin/agy): coding, architecture, debugging, testing, documentation
- **codex** (/usr/bin/codex): coding, refactoring, review, security
