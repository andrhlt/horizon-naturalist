# Pod B iteration manifest

Status: `SCOPED_DELIVERY_READY`

Engine: Horizon `0a875da91dbb0fdcb7ba8c42c11341aa229af51a` / MC 26.3 / matching SDK / Rust `1.99.1`.

Candidate: `native-0a875da-7df1b669a7d6`, source `7f38943`, installed at `/private/tmp/pod-b-dev-launcher/versions/native-0a875da-7df1b669a7d6`.

Developer base: `developer-e05268bea02f4384b113d9b9b02f0ce6310883561ccc41fea947e097148ade16`; Horizon `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`; MC 26.3; Rust `1.99.1`. Resources: `/private/tmp/pod-b-dev-launcher/versions/developer-e05268bea02f4384b113d9b9b02f0ce6310883561ccc41fea947e097148ade16/.local/horizon-client/26.3`; tree SHA-256 `75ebb227edcf56bcd67b11a983e08eafc34339864ba27b05ae6bd469bc6f5a8c`.

Artifact hashes: `libnaturalist.dylib` `c39122d4f7c7cb6e9e45dc87417fff4059e5db65dd0e2b8175fd03fb9e0aa456`; `libhorizon_patch.dylib` `947441b6d761cd70191f479d48e2451f43c5f94edd30a10476d5fb99383465d5`; client `012bf6dbfecb095bfb27f9b84452105639501cc5444aca15517360cd90547935`; server `50cfae57221e4af69c520145db739bd5ab05ea0fe48b314c749430226d37a172`; native-mods `12fac7f6ed255586ebb8359faf0f7193347a29a195c4f2d694bd09d933c6ada3`.

Included source heads are provisional and must be confirmed by owners:

| Issue | PR | Head | Runtime status |
| --- | --- | --- | --- |
| APP-2128 | #28 | `8f9be134585a8dd381453ff83c729b9ff39dcdfd` | owner checklist pending; Firefly juvenile blocked by APP-2159 |
| APP-2129 | #23 | `7f30ea2eb97dd5de23cec892c0b1143c9105bf9e` | current-source build/runtime gates pending |
| APP-2131 | #26 | `8c1678c25a67fbeca042fc9fc27717d9b94e9aa5` (base `07dab0d`) | Giraffe source-matched gate recorded; include `ae3b78e` and retest Rhino/Zebra/Lion/Tiger |
| APP-2132 | #30 | follow-up owner handoff: `f89215c`, `993743d`, `37fb443` | Mole movement/trails pass on owner candidate; Lizard ordered sitting blocked by frozen SDK |

Supported launch: managed developer Play with isolated `HORIZON_LAUNCHER_HOME`, `HORIZON_HOME`, `HORIZON_LAUNCHER_DEVELOPER_OFFLINE=1`, headless control socket, and matching prepared MC26.3 resources. Offline developer mode is single-player-only; no account sign-in or cache-lease bypass was used.
