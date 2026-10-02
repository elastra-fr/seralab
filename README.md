# SERALab

SERALab is an experimental Rust project focused on local system, device and network automation.

The core idea will follow a simple flow:

**Sense → Evaluate → React → Act**

The project will aim to detect changes in local or network resources, evaluate configurable conditions, and trigger actions accordingly.

## Status

Early development / proof of concept.

## Goals

SERALab will aim to be:

- local-first
- lightweight
- Linux-oriented
- modular
- terminal-friendly
- independent from any mandatory web stack

## License

TBD

## Commit convention

SERALab follows the Conventional Commits format:

type(scope): description

Common types used in the project:
- feat : new functionality or capability
- fix : bug fix
- refactor : internal code changes without behavior changes
- docs : documentation only
- test : tests
- chore : project maintenance
- build : build system or dependency changes
- ci : CI configuration
- perf : performance improvements
- style : formatting-only changes

Examples:

```
feat(sense): add observation data model
feat(sense): implement TCP availability probe
fix(sense): handle invalid ARP entries
refactor(core): move observation types into dedicated module
docs: update project goals
test(sense): add ARP parser cases
chore: update gitignore
```