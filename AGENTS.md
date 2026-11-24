# Project guidance

## Current priority: practical playability

The current acceptance target is a reliable private family-and-friends play
session. For this mod-port and compatibility-fork work, this section supersedes
older instructions that require exhaustive Java parity or lengthy evidence
before delivery. It does not change the upstream project's long-term parity goal.

### Completion threshold

- Build and load the intended package successfully.
- Exercise a short normal-player scenario covering each feature family's main
  implemented mechanics. For animals, cover applicable movement, interactions,
  feeding/taming/breeding/capture. For machinery, cover placement, power, work,
  stopping and recovery. Do not substitute registration or helper tests for use.
- Check representative save/reopen persistence of ownership, inventories,
  settings and meaningful progress.
- Check representative offline host/guest behavior when ownership, interaction
  or synchronization depends on multiple players. Use distinct authoritative
  player identities, not two aliases for one UUID. No Microsoft sign-ins or
  device-code flows are required for this verification target.
- Fix reproducible crashes, duplication, item/entity loss, save corruption and
  broken main gameplay mechanics. Missing ordinary functionality stays open;
  do not hide it as a cosmetic follow-up or claim the full feature works.

Exact tick/packet timing, RNG sequences, exhaustive variant combinations, pixel
comparisons, minor animation, interpolation and shadow differences are follow-up
work unless they materially affect gameplay. They do not block this baseline.
Do not describe a representative gameplay pass as exhaustive 1:1 parity.

### Efficient verification and delivery

- Use the same identified source/build artifact for a check. Record the commit,
  artifact identity, short command/result and actual remaining bugs. Reuse valid
  unchanged evidence; avoid repeated broad audits, verbose reports or new test
  frameworks. Run fast checks first and rebuild only for relevant source changes.
- Feature owners implement and fix their features through acceptance; a patch
  submission or failed test report is not completion. Repair recoverable fixture,
  build and setup problems. Park only a concrete external dependency with a
  precise next action, while continuing useful independent work.
- Naturalist pods batch contributor commits into one immutable build per pod
  iteration, then return it to each owner for their own isolated tests. Pods may
  build concurrently. Create workers independently own their builds and tests;
  they do not wait for a pod or central build queue. Isolate writable caches,
  launcher/game homes, worlds and control sockets; reuse verified immutable
  inputs where supported. Never delete another worker's locks or stop its jobs.
- All automated gameplay, server and guest checks are headless. Actual frame
  checks use a supported genuinely offscreen mode without visible windows,
  focus changes, system pointer capture/warping or desktop input. Use supported
  offline/developer test paths; do not bypass authentication or managed leases.
- Preserve the agreed engine/SDK pin, source/artifact compatibility, other
  workers' changes, worlds and evidence. A fork cohort must be explicitly
  identified; do not silently upgrade the engine or mix incompatible SDKs.
- Deliver a focused reviewed PR once the practical baseline passes. Honor pod
  delivery grouping. Resolve required checks/review, merge through normal repo
  rules, then close the covered baseline issues and report any release limits.
  Do not waive a known serious defect to close a ticket. Exact parity follow-ups
  can remain open without blocking a correctly scoped baseline delivery.
- Before deployment, run one combined smoke session with the selected mods:
  representative gameplay, an early Create production chain where included,
  save/reopen and an offline guest reconnect. A component pass alone does not
  establish the complete deployment works.
