# SERALab

SERALab is an experimental Rust project focused on local system, device and network automation.

## Why S.E.R.A

SERALab follows a simple event-driven flow:

**Sense → Evaluate → Resolve → Act**

Each stage has a distinct responsibility and should remain as independent as possible from the others.

### Sense

Collect raw observations from the local system, connected devices and network environment.

Possible sources include:

- network discovery
- ARP, mDNS, SSDP and other discovery mechanisms
- TCP / HTTP availability checks
- USB and udev events
- processes and system services
- files, mounts and local devices

The Sense layer reports facts only. It should not decide what those observations mean or what action should follow.

### Evaluate

Interpret and correlate observations in order to build and update the current state of known resources.

This stage may:

- merge observations coming from different sources
- identify or qualify resources
- compare current and previous states
- detect state transitions
- evaluate configured conditions
- apply logical operators such as `AND`, `OR` and `NOT`
- handle timing constraints such as delays, persistence or cooldowns

The Evaluate layer answers questions such as:

> Is this the same resource we observed before?

> Has its state changed?

> Are the conditions of a rule currently satisfied?

### Resolve

Determine what should happen when evaluated conditions produce a meaningful result.

This stage connects state changes and satisfied rules to configured responses.

It may:

- match events against rules
- resolve which actions should be triggered
- prevent duplicate or conflicting executions
- apply priorities or execution policies
- decide whether an action should run immediately, later, or not at all

The Resolve layer does not perform the action itself. It turns evaluation results into explicit execution decisions.

### Act

Execute the resolved actions.

Possible actions include:

- launch a local application
- run a command
- send a desktop notification
- start or stop a system service
- mount or unmount a resource
- execute a remote command over SSH

The Act layer should focus only on execution and reporting the result of that execution.

### Flow

A typical flow may look like this:

```text
Sense
  ↓
Observation
  ↓
Evaluate
  ↓
State change / rule match
  ↓
Resolve
  ↓
Action decision
  ↓
Act
  ↓
Execution result
```

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

## Commit convention

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