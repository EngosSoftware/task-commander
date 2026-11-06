# ADR-0001 Build task runner in Rust

## Date
 
**2026-01-03**

## Status

**Accepted**

## Context

Rust projects typically require a command runner to orchestrate common development workflows
such as building, testing, linting, generating coverage reports, and releasing crates.

The de facto standard for this purpose is [go-task], a mature and widely adopted task runner
implemented in Go. It is configured through `Taskfile.yml` and distributed as a standalone binary.

We have used [go-task] for several years across multiple non-trivial Rust projects and have gained
substantial operational experience with it. This experience has helped us identify both the essential
capabilities required from a task runner in the Rust ecosystem and the limitations of [go-task].

These observations motivated an evaluation of whether a purpose-built solution could better address
the needs of Rust projects while retaining the simplicity of a standalone command runner.

## Decision

We build a new, independent task runner written in Rust for Rust project workflows.

## Decision Drivers

### Toolchain purity

[go-task] is implemented in Go. Even though it ships as a single binary,
depending on it still means a Go-originated tool sits in the middle of an otherwise
all-Rust toolchain and release pipeline.

### Distribution as a Rust crate
 
Shipping task runner as a installable crate for projects that are already standardized on `cargo`.

### A different task-definition format

[go-task]'s `Taskfile.yml` is YAML. The intent with a new task runner is to move
away from YAML for task definitions in favor of a syntax better suited to the tool's
own semantics without a need to build a DSL.

### Full control as a from-scratch project
 
Building task runner from the ground up means owning the feature roadmap, file format,
and execution model directly. This also serves as a deliberate opportunity to design
and ship a complete Rust CLI tool end-to-end.

## Consequences

### Positive

- The full toolchain for Rust-centric repositories can stay Rust-only,
  with no Go dependency required to build or run project tasks.
- Full freedom to define the task file format and feature set to fit the
  workflows actually needed, rather than working within [go-task]'s existing model.

### Negative

- All maintenance burden shifts from an established upstream project onto
  task runner own development — no inherited community, issue triage, or battle-testing.
- Task runner starts with a smaller feature set and ecosystem than [go-task]
  and will need time to reach maturity.
- Anyone adopting a new task runner has to learn a new, non-standard task-definition
  syntax instead of reusing the widely known `Taskfile.yml` format.

[go-task]: https://taskfile.dev
