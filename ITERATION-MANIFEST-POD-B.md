# Pod B iteration manifest

Status: `PENDING_OWNER_HANDOFFS`

Engine: Horizon `0a875da91dbb0fdcb7ba8c42c11341aa229af51a` / MC 26.3 / matching SDK / Rust `1.99.1`.

Candidate: Pod A reported `native-0a875da-47ed79e4f5a5`, source `f56207f`, dylib SHA-256 `f64a71294307c0f7a6ef4f3d195ca06afebbacef696431650064499a94e6e5e4`. This is provisional until APP-2135 publishes the final immutable manifest and resource hashes.

Included source heads are provisional and must be confirmed by owners:

| Issue | PR | Head | Runtime status |
| --- | --- | --- | --- |
| APP-2128 | #28 | `8f9be134585a8dd381453ff83c729b9ff39dcdfd` | owner checklist pending; Firefly juvenile blocked by APP-2159 |
| APP-2129 | #23 | `7f30ea2eb97dd5de23cec892c0b1143c9105bf9e` | current-source build/runtime gates pending |
| APP-2131 | #26 | `8c1678c25a67fbeca042fc9fc27717d9b94e9aa5` (base `07dab0d`) | Giraffe source-matched gate recorded; include `ae3b78e` and retest Rhino/Zebra/Lion/Tiger |
| APP-2132 | #30 | owner worktree `245aae69d13678e8b4b19ae2876d5a2d56146f07`; PR head `3ccb2e29b8965f9dbaaa9f8132f2552c5e81a2f5` | fresh Komodo/Lizard/Mole/Tortoise gameplay blocked; PR head lacks later Mole/pet follow-up |

No client/server/resource path or hash is accepted as immutable until APP-2135 confirms the final build output. No owner may claim PASS from historical or unmatched artifacts.
