# Naturalist Pod B

Iteration freeze: pending owner handoffs; no heavy build started in Pod B.

## Scope

- APP-2128 insects and terrestrial invertebrates — PR #28 follow-up, head `8f9be134585a8dd381453ff83c729b9ff39dcdfd` (prior merged baseline #21 retained).
- APP-2129 freshwater and coastal wildlife — PR #23, head `7f30ea2eb97dd5de23cec892c0b1143c9105bf9e`.
- APP-2131 large land animals — PR #26, clean head `8c1678c25a67fbeca042fc9fc27717d9b94e9aa5`, base `07dab0d`; Rhino timer-slot fix `ae3b78e` must be included and verified.
- APP-2132 reptiles and burrowers — owner worktree head `245aae69d13678e8b4b19ae2876d5a2d56146f07`; PR #30 remains separate at `3ccb2e29b8965f9dbaaa9f8132f2552c5e81a2f5` and does not contain the later Mole/pet follow-up.

All source inputs are based on reviewed `main` ancestry; owner handoffs must confirm clean state, exact base, owned files, and remaining acceptance items before inclusion.

## Shared build queue

- Frozen engine: Horizon `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`; MC 26.3; matching SDK; Rust `1.99.1`.
- Pod A candidate reported: `native-0a875da-47ed79e4f5a5`, source snapshot `f56207f`, `libnaturalist.dylib` SHA-256 `f64a71294307c0f7a6ef4f3d195ca06afebbacef696431650064499a94e6e5e4`.
- Launcher: `/Applications/Horizon.app/Contents/MacOS/horizon-launcher`.
- Runtime route: `tools/launcher_selected_headless.py`, isolated world and control socket per owner; actual frames use supported `--offscreen` only.
- Exactly one heavy build slot across Pods A/B/C/Sable/Create. Pod B does not build until the coordinator confirms the slot and the frozen manifest is immutable.

## Acceptance checklist

- [x] Four owner handoffs received and clean heads/base/owned files recorded; APP-2128 and APP-2129 are pending only for the final integrated source/build selection.
- [ ] Shared manifest records source commit, included heads, staged hash, SDK/toolchain, launcher, client/server/resource paths and hashes, supported launch command, and isolated world/socket instructions.
- [ ] Same candidate dispatched to all four owners; each runs its own headless gameplay/LAN/persistence checks and reports source-matched evidence.
- [ ] APP-2159 Firefly juvenile-age seam remains blocked until an actual supported API/fork fix is verified.
- [ ] Tortoise egg `stepOn` callback gap remains explicitly blocked; `fallOn` is separate and must be tested where applicable.
- [ ] No auth/lease bypass, stale ABI mixing, visible-window interaction, force-push, or unrelated source is admitted.
- [ ] Cross-feature loading and conservation smoke reviewed centrally; Pod B does not claim sole full-package acceptance.
- [ ] One combined Pod B PR only after all four member scopes pass their real acceptance gates; existing PRs remain preserved until safely superseded.

