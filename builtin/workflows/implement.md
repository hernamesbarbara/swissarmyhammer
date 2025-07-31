---
title: Implement
description: Autonomously run until all issues are resolved
tags:
  - auto
---

## States

```mermaid
stateDiagram-v2
    [*] --> start
    start --> are_issues_complete
    are_issues_complete --> loop
    loop --> done: result.matches("(?i)YES")
    loop --> work: result.matches("(?i)NO")
    work --> are_issues_complete
    done --> [*]
```

## Actions

- start: log "Resolving issues"
- are_issues_complete: execute prompt "are_issues_complete"
- work: run workflow "do_issue"
- done: log "Complete"

## Description

This workflow works on tests until they all pass.
