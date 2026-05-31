# Agent Task Template

Use this template for substantial delegated work.

## Task

- Title:
- Owner:
- Date:
- Milestone:
- Scope:

## Goal

What should be true when this task is complete?

What user-visible feature or normal program-visible behavior moves forward?
If the answer is only "a diagnostic/proof exists", explain the blocking feature
unknown that makes a diagnostic necessary.

## Acceptance Criteria

- What observable behavior, artifact, or evidence must exist before this task can be accepted?
- How does the evidence exercise the real feature path directly?
- Which validation level is required: static inspection, fmt/lint/typecheck, tests, QEMU/substitute, lab-controller API, serial hardware boot/output, or repeated hardware run?
- For hardware tasks, what serial output, lab-controller result, or boot classification proves success?
- If diagnostic-only evidence is used, what feature did it unblock, and what is
  the retirement or promotion plan for that diagnostic?

## Context

Relevant docs, files, hardware notes, prior decisions, and constraints.

## Work Performed

What changed or what was investigated?

## Evidence

References, commands, logs, tests, serial output, or source material used.

## Review

- Pre-hardware review findings:
- Hardware test evidence, if required:
- Post-hardware review findings:

## Result

What was completed?

What can a user, shell, program, or kernel subsystem do now that it could not do
before?

## Follow-Up

Open questions, risks, blocked work, and recommended next tasks.
