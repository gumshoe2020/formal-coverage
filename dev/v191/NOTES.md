# v1.9.1 notes

## 2026-11-25T15:30:09.218228+00:00 — M0 started

Read PLAN.md fully, src/fstar.rs (mapping, numbered-impl rule and tests), src/cfg.rs, and dev/v185/replay.py. All staged receipt files/fresh/checked/patches and combined log hashes validated. No hax or extraction run.

Shared memory search and filing were not available to this run; decisions are recorded here only.

## M0 patterns and provenance

Provenance: pinned hax commit `5ccf89dd2e996c9f88f2267eaa2a64739c4ca41a`, inspected read-only under the explicitly supplied toolchain's `source/hax-<commit>/`. References below are relative to that checkout. Staged F* references are relative to `target/v191-inputs/bertie/target/ci-logs/fstar/`.

A: `engine/lib/concrete_ident/concrete_ident.ml:335-342` implements `disambiguator_escape`: split on `_`, append `_` if there are at least two segments and the last parses as an integer. Zero-disambiguator names use that escape. The staged cases establish underscore-separated final decimal segments; do not generalize to all digit-ending names on this evidence. Exact example: `let two_pow_256_ (_: Prims.unit) =` at `fresh/Bertie_tls.Der.fst:6`; `let next_keys_c_2_` at `fresh/Bertie_tls.Tls13keyscheduler.fst:164`.

B: receipt `extraction_command` is only `["python3", "hax-driver.py", "extract-fstar"]`, not an expanded cargo command, and the receipt has no feature/cfg field. Its `files["hax-driver.py"]` binds the script containing `--no-default-features` and `--features defensive,std,client,sha256,chacha20poly1305,ecdsa-p256,x25519,p256,mlkem768` in `cargo_hax_into`. Thus there is no explicit receipt feature set for a general rule to evaluate. Per PLAN B, keep cfg twins unmapped and log the limitation; do not infer a cfg set by interpreting arbitrary Python. Exact patterns: `let aead_encrypt (k: t_AeadKey) ... =` at Tls13crypto.fst:184, `let aead_decrypt ... =` at :269, and `let tlserr (#v_T: Type0) (err: u8) =` at Tls13utils.fst:33. The cfg-inactive twins must remain unchanged.

C: `engine/backends/fstar/fstar_backend.ml:1641-1734` renders `Impl`: trait and generic arguments form the type application; each `IIFn` becomes a field with lambda body, `IIType` becomes an associated type field, and the record receives `tcinstance`. Exact example at Key_schedule.fst:545-546: `[@@ FStar.Tactics.Typeclasses.tcinstance]` then `let impl: t_KeySchedule t_TLSkeyscheduler Bertie_tls.Tls13keyscheduler.Key_names.t_TLSnames =`. Tls13utils.fst:141-142 has `let impl_13 (v_C: usize) : Core_models.Convert.t_From Bertie_tls.Tls13utils.Byte_types.t_Bytes (t_Array u8 v_C) =`; :159-160 repeats that type under impl_14, so generic identity cannot be guessed from declaration order.

D: the same renderer chooses `~fsti:(ctx.interface_mode && has_type)` for a record that includes an associated type. Tls13formats.fsti:240-241 has the tcinstance attribute and `let impl_1: Core_models.Convert.t_TryFrom Bertie_tls.Tls13formats.Wire_types.t_AlertLevel u8 = { ... }`; :276 is `impl_3` for AlertDescription. Each contains `f_Error`, `f_try_from_pre`, `f_try_from_post`, and `f_try_from = fun (value: u8) -> ...`. Canary inserts `let v191_canary : (x: Prims.int { x > 0 }) = 0 in` into impl_1's method body, retaining its original match expression. It checks only `fresh/Bertie_tls.Tls13formats.fst` with the receipt's exact include arguments, image `fc-verify-fstar-amd64:v17`, F* 2025.10.06 and z3 4.13.3 as verify-fstar.py does; no hax, extraction, or Rust toolchain override. Toolchain mounted read-only; writable mount confined to this repo. Copies and logs: `target/v191/canary/`.

E: `engine/lib/concrete_ident/concrete_ident.ml:505-539,554-578` uses a pretty type only if namespace, simple-type, and generic constraints allow it. If unavailable with disambiguator zero, the impl chunk is just `impl`; concatenating its associated method produces `impl__new`. Staged `fresh/Bertie_tls.Server.fst:12-29` defines `let impl__new ... = { ... } <: Bertie_tls.Server.Server_db.t_ServerDB`. The whole-token type guard is present. Do not infer type from the method name.

### Bertie definitions in these forms

This inventory uses the staged CI report's definition names and source locations; broad instance inventory includes hazard modules, marker records and trusted records which the rules must refuse or leave trusted. It is not a promised credit list.

A:

- `Bertie_tls.Der.two_pow_256_` — `fresh/Bertie_tls.Der.fst:6` (unmapped).
- `Bertie_tls.Der.be_value_prefix_upper_128_` — `fresh/Bertie_tls.Der.fst:285` (unmapped).
- `Bertie_tls.Der.be_value_prefix_lower_128_` — `fresh/Bertie_tls.Der.fst:295` (unmapped).
- `Bertie_tls.Der.two_pow_256_` — `fresh/Bertie_tls.Der.fsti:10` (unmapped).
- `Bertie_tls.Der.be_value_prefix_upper_128_` — `fresh/Bertie_tls.Der.fsti:204` (unmapped).
- `Bertie_tls.Der.be_value_prefix_lower_128_` — `fresh/Bertie_tls.Der.fsti:222` (unmapped).
- `Bertie_tls.Tls13keyscheduler.next_keys_c_2_` — `fresh/Bertie_tls.Tls13keyscheduler.fst:164` (unmapped).
- `Bertie_tls.Tls13keyscheduler.next_keys_c_2_` — `fresh/Bertie_tls.Tls13keyscheduler.fsti:40` (unmapped).

B:

- `Bertie_tls.Tls13crypto.aead_encrypt` — `fresh/Bertie_tls.Tls13crypto.fst:184` (unmapped).
- `Bertie_tls.Tls13crypto.aead_decrypt` — `fresh/Bertie_tls.Tls13crypto.fst:269` (unmapped).
- `Bertie_tls.Tls13crypto.aead_encrypt` — `fresh/Bertie_tls.Tls13crypto.fsti:103` (unmapped).
- `Bertie_tls.Tls13crypto.aead_decrypt` — `fresh/Bertie_tls.Tls13crypto.fsti:109` (unmapped).
- `Bertie_tls.Tls13utils.tlserr` — `fresh/Bertie_tls.Tls13utils.fst:33` (unmapped).
- `Bertie_tls.Tls13utils.tlserr` — `fresh/Bertie_tls.Tls13utils.fsti:59` (unmapped).

C:

- `Bertie_tls.Server.Server_db.impl_1` — `fresh/Bertie_tls.Server.Server_db.fst:8` (trusted).
- `Bertie_tls.Server.Server_db.impl_1` — `fresh/Bertie_tls.Server.Server_db.fst:10` (hazard).
- `Bertie_tls.Tls13cert.impl` — `fresh/Bertie_tls.Tls13cert.fst:13` (unmapped).
- `Bertie_tls.Tls13cert.impl_1` — `fresh/Bertie_tls.Tls13cert.fst:21` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_1` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:14` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_1` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:16` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_3` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:20` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_3` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:22` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_4` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:26` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_4` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:28` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_2` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:32` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_2` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:34` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_5` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:38` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_5` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:40` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_6` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:44` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_6` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:46` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_8` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:59` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_8` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:61` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_9` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:65` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_9` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:67` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_10` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:71` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_10` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:73` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_11` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:77` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_11` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:79` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_12` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:83` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_12` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:85` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_13` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:89` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_13` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:91` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_15` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:95` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_15` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:97` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_18` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:107` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_18` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:109` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_19` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:113` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_19` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:115` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_20` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:119` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_20` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:121` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_21` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:125` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_21` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:127` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_23` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:137` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_23` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:139` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_24` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:143` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_24` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:145` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_25` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:149` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_25` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:151` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_26` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:155` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_26` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:157` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_28` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:161` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_28` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:163` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_29` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:167` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_29` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:169` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_30` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:173` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_30` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:175` (hazard).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_31` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:179` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_31` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fst:181` (hazard).
- `Bertie_tls.Tls13formats.Handshake_data.impl_1` — `fresh/Bertie_tls.Tls13formats.Handshake_data.fst:294` (unmapped).
- `Bertie_tls.Tls13formats.Handshake_type.impl_1` — `fresh/Bertie_tls.Tls13formats.Handshake_type.fst:22` (trusted).
- `Bertie_tls.Tls13formats.Handshake_type.impl_1` — `fresh/Bertie_tls.Tls13formats.Handshake_type.fst:24` (hazard).
- `Bertie_tls.Tls13formats.Handshake_type.impl_2` — `fresh/Bertie_tls.Tls13formats.Handshake_type.fst:28` (trusted).
- `Bertie_tls.Tls13formats.Handshake_type.impl_2` — `fresh/Bertie_tls.Tls13formats.Handshake_type.fst:30` (hazard).
- `Bertie_tls.Tls13formats.Handshake_type.impl_3` — `fresh/Bertie_tls.Tls13formats.Handshake_type.fst:34` (trusted).
- `Bertie_tls.Tls13formats.Handshake_type.impl_3` — `fresh/Bertie_tls.Tls13formats.Handshake_type.fst:36` (hazard).
- `Bertie_tls.Tls13formats.Wire_types.impl_1` — `fresh/Bertie_tls.Tls13formats.Wire_types.fst:13` (trusted).
- `Bertie_tls.Tls13formats.Wire_types.impl_1` — `fresh/Bertie_tls.Tls13formats.Wire_types.fst:15` (hazard).
- `Bertie_tls.Tls13formats.Wire_types.impl_2` — `fresh/Bertie_tls.Tls13formats.Wire_types.fst:19` (trusted).
- `Bertie_tls.Tls13formats.Wire_types.impl_2` — `fresh/Bertie_tls.Tls13formats.Wire_types.fst:21` (hazard).
- `Bertie_tls.Tls13formats.Wire_types.impl_3` — `fresh/Bertie_tls.Tls13formats.Wire_types.fst:25` (trusted).
- `Bertie_tls.Tls13formats.Wire_types.impl_3` — `fresh/Bertie_tls.Tls13formats.Wire_types.fst:27` (hazard).
- `Bertie_tls.Tls13formats.Wire_types.impl_5` — `fresh/Bertie_tls.Tls13formats.Wire_types.fst:73` (trusted).
- `Bertie_tls.Tls13formats.Wire_types.impl_5` — `fresh/Bertie_tls.Tls13formats.Wire_types.fst:75` (hazard).
- `Bertie_tls.Tls13formats.Wire_types.impl_6` — `fresh/Bertie_tls.Tls13formats.Wire_types.fst:79` (trusted).
- `Bertie_tls.Tls13formats.Wire_types.impl_6` — `fresh/Bertie_tls.Tls13formats.Wire_types.fst:81` (hazard).
- `Bertie_tls.Tls13formats.Wire_types.impl_7` — `fresh/Bertie_tls.Tls13formats.Wire_types.fst:85` (trusted).
- `Bertie_tls.Tls13formats.Wire_types.impl_7` — `fresh/Bertie_tls.Tls13formats.Wire_types.fst:87` (hazard).
- `Bertie_tls.Tls13formats.Wire_types.impl_9` — `fresh/Bertie_tls.Tls13formats.Wire_types.fst:99` (trusted).
- `Bertie_tls.Tls13formats.Wire_types.impl_9` — `fresh/Bertie_tls.Tls13formats.Wire_types.fst:101` (hazard).
- `Bertie_tls.Tls13formats.Wire_types.impl_10` — `fresh/Bertie_tls.Tls13formats.Wire_types.fst:105` (trusted).
- `Bertie_tls.Tls13formats.Wire_types.impl_10` — `fresh/Bertie_tls.Tls13formats.Wire_types.fst:107` (hazard).
- `Bertie_tls.Tls13formats.Wire_types.impl_11` — `fresh/Bertie_tls.Tls13formats.Wire_types.fst:111` (trusted).
- `Bertie_tls.Tls13formats.Wire_types.impl_11` — `fresh/Bertie_tls.Tls13formats.Wire_types.fst:113` (hazard).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fst:30` (trusted).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fst:32` (hazard).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_2` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fst:36` (trusted).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_2` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fst:38` (hazard).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_3` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fst:42` (trusted).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_3` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fst:44` (hazard).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_4` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fst:48` (trusted).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_4` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fst:50` (hazard).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_5` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fst:54` (trusted).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_5` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fst:56` (hazard).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_6` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fst:76` (trusted).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_6` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fst:78` (hazard).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_7` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fst:82` (trusted).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_7` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fst:84` (hazard).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_8` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fst:88` (trusted).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_8` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fst:90` (hazard).
- `Bertie_tls.Tls13keyscheduler.Key_schedule.impl` — `fresh/Bertie_tls.Tls13keyscheduler.Key_schedule.fst:546` (unmapped).
- `Bertie_tls.Tls13utils.Byte_types.impl_1` — `fresh/Bertie_tls.Tls13utils.Byte_types.fst:8` (trusted).
- `Bertie_tls.Tls13utils.Byte_types.impl_1` — `fresh/Bertie_tls.Tls13utils.Byte_types.fst:10` (hazard).
- `Bertie_tls.Tls13utils.Byte_types.impl_2` — `fresh/Bertie_tls.Tls13utils.Byte_types.fst:14` (trusted).
- `Bertie_tls.Tls13utils.Byte_types.impl_2` — `fresh/Bertie_tls.Tls13utils.Byte_types.fst:16` (hazard).
- `Bertie_tls.Tls13utils.Byte_types.impl_3` — `fresh/Bertie_tls.Tls13utils.Byte_types.fst:20` (trusted).
- `Bertie_tls.Tls13utils.Byte_types.impl_3` — `fresh/Bertie_tls.Tls13utils.Byte_types.fst:22` (hazard).
- `Bertie_tls.Tls13utils.Byte_types.impl_4` — `fresh/Bertie_tls.Tls13utils.Byte_types.fst:26` (trusted).
- `Bertie_tls.Tls13utils.Byte_types.impl_4` — `fresh/Bertie_tls.Tls13utils.Byte_types.fst:28` (hazard).
- `Bertie_tls.Tls13utils.Byte_types.impl_5` — `fresh/Bertie_tls.Tls13utils.Byte_types.fst:32` (trusted).
- `Bertie_tls.Tls13utils.Byte_types.impl_5` — `fresh/Bertie_tls.Tls13utils.Byte_types.fst:34` (hazard).
- `Bertie_tls.Tls13utils.Byte_types.impl_6` — `fresh/Bertie_tls.Tls13utils.Byte_types.fst:38` (trusted).
- `Bertie_tls.Tls13utils.Byte_types.impl_6` — `fresh/Bertie_tls.Tls13utils.Byte_types.fst:40` (hazard).
- `Bertie_tls.Tls13utils.impl_2` — `fresh/Bertie_tls.Tls13utils.fst:43` (unmapped).
- `Bertie_tls.Tls13utils.impl_12` — `fresh/Bertie_tls.Tls13utils.fst:124` (unmapped).
- `Bertie_tls.Tls13utils.impl_13` — `fresh/Bertie_tls.Tls13utils.fst:141` (unmapped).
- `Bertie_tls.Tls13utils.impl_14` — `fresh/Bertie_tls.Tls13utils.fst:159` (unmapped).
- `Bertie_tls.Tls13utils.impl_6` — `fresh/Bertie_tls.Tls13utils.fst:482` (unmapped).
- `Bertie_tls.Tls13utils.impl_7` — `fresh/Bertie_tls.Tls13utils.fst:501` (unmapped).
- `Bertie_tls.Tls13utils.impl_8` — `fresh/Bertie_tls.Tls13utils.fst:521` (unmapped).
- `Bertie_tls.Tls13utils.impl_9` — `fresh/Bertie_tls.Tls13utils.fst:545` (unmapped).
- `Bertie_tls.Tls13utils.impl` — `fresh/Bertie_tls.Tls13utils.fst:563` (unmapped).
- `Bertie_tls.Tls13utils.impl_1` — `fresh/Bertie_tls.Tls13utils.fst:571` (unmapped).

D:

- `Bertie_tls.Server.Server_db.impl` — `fresh/Bertie_tls.Server.Server_db.fsti:15` (trusted).
- `Bertie_tls.Server.Server_db.impl_1` — `fresh/Bertie_tls.Server.Server_db.fsti:19` (unmapped).
- `Bertie_tls.Server.impl_1` — `fresh/Bertie_tls.Server.fsti:36` (trusted).
- `Bertie_tls.Tls13cert.impl` — `fresh/Bertie_tls.Tls13cert.fsti:18` (unmapped).
- `Bertie_tls.Tls13cert.impl_1` — `fresh/Bertie_tls.Tls13cert.fsti:21` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:15` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_1` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:19` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_3` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:22` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_4` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:25` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_2` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:28` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_5` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:31` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_6` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:34` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_7` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:49` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_8` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:53` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_9` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:56` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_10` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:59` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_11` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:62` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_12` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:65` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_13` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:74` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_14` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:76` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_15` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:86` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_16` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:88` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_17` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:100` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_18` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:104` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_19` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:107` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_20` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:110` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_21` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:113` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_22` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:124` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_23` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:128` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_24` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:131` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_25` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:134` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_26` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:137` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_27` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:151` (trusted).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_28` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:155` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_29` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:158` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_30` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:161` (unmapped).
- `Bertie_tls.Tls13crypto.Algorithm_types.impl_31` — `fresh/Bertie_tls.Tls13crypto.Algorithm_types.fsti:164` (unmapped).
- `Bertie_tls.Tls13crypto.impl_3` — `fresh/Bertie_tls.Tls13crypto.fsti:362` (trusted).
- `Bertie_tls.Tls13formats.Handshake_data.impl_1` — `fresh/Bertie_tls.Tls13formats.Handshake_data.fsti:124` (unmapped).
- `Bertie_tls.Tls13formats.Handshake_type.impl` — `fresh/Bertie_tls.Tls13formats.Handshake_type.fsti:60` (trusted).
- `Bertie_tls.Tls13formats.Handshake_type.impl_1` — `fresh/Bertie_tls.Tls13formats.Handshake_type.fsti:64` (unmapped).
- `Bertie_tls.Tls13formats.Handshake_type.impl_2` — `fresh/Bertie_tls.Tls13formats.Handshake_type.fsti:67` (unmapped).
- `Bertie_tls.Tls13formats.Handshake_type.impl_3` — `fresh/Bertie_tls.Tls13formats.Handshake_type.fsti:70` (unmapped).
- `Bertie_tls.Tls13formats.Wire_types.impl` — `fresh/Bertie_tls.Tls13formats.Wire_types.fsti:23` (trusted).
- `Bertie_tls.Tls13formats.Wire_types.impl_1` — `fresh/Bertie_tls.Tls13formats.Wire_types.fsti:27` (unmapped).
- `Bertie_tls.Tls13formats.Wire_types.impl_2` — `fresh/Bertie_tls.Tls13formats.Wire_types.fsti:30` (unmapped).
- `Bertie_tls.Tls13formats.Wire_types.impl_3` — `fresh/Bertie_tls.Tls13formats.Wire_types.fsti:33` (unmapped).
- `Bertie_tls.Tls13formats.Wire_types.impl_4` — `fresh/Bertie_tls.Tls13formats.Wire_types.fsti:152` (trusted).
- `Bertie_tls.Tls13formats.Wire_types.impl_5` — `fresh/Bertie_tls.Tls13formats.Wire_types.fsti:156` (unmapped).
- `Bertie_tls.Tls13formats.Wire_types.impl_6` — `fresh/Bertie_tls.Tls13formats.Wire_types.fsti:159` (unmapped).
- `Bertie_tls.Tls13formats.Wire_types.impl_7` — `fresh/Bertie_tls.Tls13formats.Wire_types.fsti:162` (unmapped).
- `Bertie_tls.Tls13formats.Wire_types.impl_8` — `fresh/Bertie_tls.Tls13formats.Wire_types.fsti:194` (trusted).
- `Bertie_tls.Tls13formats.Wire_types.impl_9` — `fresh/Bertie_tls.Tls13formats.Wire_types.fsti:198` (unmapped).
- `Bertie_tls.Tls13formats.Wire_types.impl_10` — `fresh/Bertie_tls.Tls13formats.Wire_types.fsti:201` (unmapped).
- `Bertie_tls.Tls13formats.Wire_types.impl_11` — `fresh/Bertie_tls.Tls13formats.Wire_types.fsti:204` (unmapped).
- `Bertie_tls.Tls13formats.impl_1` — `fresh/Bertie_tls.Tls13formats.fsti:241` (trusted).
- `Bertie_tls.Tls13formats.impl_3` — `fresh/Bertie_tls.Tls13formats.fsti:276` (trusted).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_1` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fsti:29` (trusted).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fsti:33` (unmapped).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_2` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fsti:36` (unmapped).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_3` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fsti:39` (unmapped).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_4` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fsti:42` (unmapped).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_5` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fsti:45` (unmapped).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_6` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fsti:65` (unmapped).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_7` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fsti:68` (unmapped).
- `Bertie_tls.Tls13keyscheduler.Key_names.impl_8` — `fresh/Bertie_tls.Tls13keyscheduler.Key_names.fsti:71` (unmapped).
- `Bertie_tls.Tls13keyscheduler.Key_schedule.impl` — `fresh/Bertie_tls.Tls13keyscheduler.Key_schedule.fsti:110` (unmapped).
- `Bertie_tls.Tls13keyscheduler.Key_schedule.impl_1` — `fresh/Bertie_tls.Tls13keyscheduler.Key_schedule.fsti:118` (trusted).
- `Bertie_tls.Tls13keyscheduler.Key_schedule.impl_2` — `fresh/Bertie_tls.Tls13keyscheduler.Key_schedule.fsti:147` (trusted).
- `Bertie_tls.Tls13utils.Byte_types.impl` — `fresh/Bertie_tls.Tls13utils.Byte_types.fsti:9` (trusted).
- `Bertie_tls.Tls13utils.Byte_types.impl_1` — `fresh/Bertie_tls.Tls13utils.Byte_types.fsti:13` (unmapped).
- `Bertie_tls.Tls13utils.Byte_types.impl_2` — `fresh/Bertie_tls.Tls13utils.Byte_types.fsti:16` (unmapped).
- `Bertie_tls.Tls13utils.Byte_types.impl_3` — `fresh/Bertie_tls.Tls13utils.Byte_types.fsti:19` (unmapped).
- `Bertie_tls.Tls13utils.Byte_types.impl_4` — `fresh/Bertie_tls.Tls13utils.Byte_types.fsti:22` (unmapped).
- `Bertie_tls.Tls13utils.Byte_types.impl_5` — `fresh/Bertie_tls.Tls13utils.Byte_types.fsti:27` (unmapped).
- `Bertie_tls.Tls13utils.Byte_types.impl_6` — `fresh/Bertie_tls.Tls13utils.Byte_types.fsti:30` (unmapped).
- `Bertie_tls.Tls13utils.impl_10` — `fresh/Bertie_tls.Tls13utils.fsti:14` (trusted).
- `Bertie_tls.Tls13utils.impl_2` — `fresh/Bertie_tls.Tls13utils.fsti:74` (unmapped).
- `Bertie_tls.Tls13utils.impl_15` — `fresh/Bertie_tls.Tls13utils.fsti:103` (trusted).
- `Bertie_tls.Tls13utils.impl_16` — `fresh/Bertie_tls.Tls13utils.fsti:123` (trusted).
- `Bertie_tls.Tls13utils.impl_12` — `fresh/Bertie_tls.Tls13utils.fsti:225` (unmapped).
- `Bertie_tls.Tls13utils.impl_13` — `fresh/Bertie_tls.Tls13utils.fsti:228` (unmapped).
- `Bertie_tls.Tls13utils.impl_14` — `fresh/Bertie_tls.Tls13utils.fsti:232` (unmapped).
- `Bertie_tls.Tls13utils.impl_6` — `fresh/Bertie_tls.Tls13utils.fsti:759` (unmapped).
- `Bertie_tls.Tls13utils.impl_7` — `fresh/Bertie_tls.Tls13utils.fsti:762` (unmapped).
- `Bertie_tls.Tls13utils.impl_8` — `fresh/Bertie_tls.Tls13utils.fsti:766` (unmapped).
- `Bertie_tls.Tls13utils.impl_9` — `fresh/Bertie_tls.Tls13utils.fsti:770` (unmapped).
- `Bertie_tls.Tls13utils.impl` — `fresh/Bertie_tls.Tls13utils.fsti:781` (unmapped).
- `Bertie_tls.Tls13utils.impl_1` — `fresh/Bertie_tls.Tls13utils.fsti:784` (unmapped).

E:

- `Bertie_tls.Server.impl__new` — `fresh/Bertie_tls.Server.fst:12` (unmapped).
- `Bertie_tls.Server.impl__new` — `fresh/Bertie_tls.Server.fsti:14` (unmapped).

## 2026-11-25T15:33:23.840410+00:00 — M0 canary result and scope decisions

Broken-body run exited 1 with Error 19, Assertion failed at Tls13formats.fsti:241-274, referring to the modified method body. Restored-body run exited 0 with Verified module: Bertie_tls.Tls13formats and All verification conditions discharged successfully. The restored file is byte-identical to the staged original. D is enabled by this canary. Generated caches were removed between runs.

B remains unmapped because the receipt has no explicit feature/cfg set. Positive B credit tests cannot be implemented honestly with the current receipt schema; tests will exercise visible refusal of twins instead. C must also refuse any block requiring cfg disambiguation, erased reference/alias distinctions or ambiguous binders; the numerical estimate is not a target. A will require a final underscore-separated decimal segment, as established by hax source. No form was found to contradict the plan.

## 2026-11-25T15:41:49.807513+00:00 — M1 implementation decisions

New fallbacks live in src/fstar/naming.rs. Existing direct/value/numbered lookups retain precedence. Proposals are collected before credit; distinct F* owners colliding with each other or any existing mapping are refused. Checked siblings with the same module/name are one logical owner. A requires an underscore-separated decimal segment and refuses an existing ambiguous unstripped spelling. E delegates to the numbered block matcher with a synthetic group key and preserves apostrophe/type-token guards.

B has no positive credit path without an explicit extraction cfg set; duplicate direct identities get a JSON/Markdown reason. C/D resolve explicit Rust imports and qualified paths, local types, primitive arguments, and known core trait paths. Wildcards do not supply identities. Conditional imports, cfg-dependent blocks/methods, references, aliases and generic binder identities without an implemented correspondence are refused visibly, rather than collapsing erased Rust distinctions. Trait-default methods receive no credit.

A matched record emits one definition row per explicit Rust method body field. A strictly checked typed trait method supplies a safety contract even when the record declaration has no standalone Tot/Pure annotation; these newly mapped records get safety-only, with the explicit postcondition text 'Typed trait instance method; member postcondition not separately decoded'. No functional specification is inferred. D retains the existing trusted classification and only maps with the canaried F* version 2025.10.06.

New unit and receipt-fixture tests cover suffix limits/precedence, absent-cfg twin refusal, trait/Self/argument mismatches, pre/post-only fields, unsupported generic binders, conditional/shadowed/ambiguous traits, the D version guard, whole E groups/type guards/apostrophes, injectivity, failed modules and hazards. B's requested positive case cannot be exercised honestly with this staged receipt schema (M0 finding explicitly allowed by PLAN). No existing assertion was edited or weakened.

Initial targeted runs exposed a syn 3 AST mismatch, impl scopes inside Unit.module, a missing synthetic interface signature, and clippy's owned comparison; all corrected. Targeted v191 tests and final M1 clippy now exit 0.

## 2026-11-25T15:49:29.135488+00:00 — sibling-group audit correction

Stopped the preliminary replay with SIGTERM (exit -15), before compare.py ran: final source review found E was grouping each checked file separately instead of combining fst/fsti declarations like v1.8.5. E now uses module-wide sibling declarations; a new regression test refuses methods split over two inherent blocks even when the F* names are split across sibling files. New fallbacks also consult hazards across both siblings. These guard corrections do not use observed Bertie totals, and no comparison check had run or failed. Targeted v191 tests and clippy exit 0 after correction.

M0/M1 logs formerly under target/v191 are archived under logs/v191/pre-replay; canary logs are under its canary subdirectory. The preliminary replay will be discarded and restarted from empty output.

## 2026-11-25T15:55:38.532522+00:00 — HONEST-FINDING: stopped at replay (b)

Replay check (b) failed: `bertie_tls::der::be_value_prefix_upper_128` (`src/der.rs:470-480`) changed from `unverified` to `verified`. Form A maps its strictly checked `be_value_prefix_upper_128_` definition. The existing v1.8.1 classifier assigns `verified` to extraction-only cfg(hax) proof units; check (b) explicitly permits only proven-extracted (or D trust). Preserving that classifier and applying A to this helper conflicts with the written replay acceptance condition. No rule or assertion was adjusted to suppress the failure. A spec decision is required before continuing.

The empty-output replay exited 1. Check (a) passed before (b) failed; (c), (d), and (e) did not complete. Both ghost-mode reports and all control reports were generated, but report generation is not an equality-check pass. Full failure logs remain in logs/v191/replay.log and target/v191/compare.log. Diagnostic changed-unit tables are in compare.md.

Final acceptance commands 1–4 and 6 exited 0, timestamped in PROGRESS.md; version stdout was formal-coverage 1.9.1. M2 is incomplete and V1.9.1-READY is deliberately absent. No hax, additional F*, push, tag or post-failure rule changes. Work stopped after documenting and committing this blocker.

## 2026-11-25T15:57:30.248251+00:00 — M2 run 2

Run 2 scope: Dudley/Andrew authorized verified destinations only for units that remain proof-code and have a form A–E target. Added an explicit zero runtime-delta assertion for changed proof-code units; existing totals arithmetic is unchanged. README already states B refusal without a receipt cfg set and D trusted classification; Cargo.toml/Cargo.lock already carry 1.9.1. No src/ changes. Shared-memory search/diary tools are unavailable in the connected catalog.

## 2026-11-25T16:04:41.089760+00:00 — M2 run 2 acceptance passed

The approved check (b) amendment resolves run 1's HONEST-FINDING. Fresh replay exited 0, all checks (a)–(e) passed, and all sixteen control/mode pairs have exact JSON and Markdown equality. Fifteen source units change in each ghost mode. Both 10-SLoC proof-code helpers remain proof-code and become verified, with zero runtime-delta contribution; existing totals arithmetic passes unchanged. Separate: 4,027 → 4,111 proven-extracted / 5,078 (79.30% → 80.96%); runtime: 4,382 → 4,467 / 5,448 (80.43% → 81.99%). Trusted runtime SLoC: 118 → 161 in both modes, all new trust attributed to D.

README v1.9.1 reviewed against the final reports: B remains refused for lack of an explicit receipt cfg set; D retains trusted classification. Existing README and 1.9.1 version files already match, so neither needed edits. compare.md regenerated with the amendment note and passing tables. Acceptance 1–6 each exited 0 on current stable Rust, with RUSTUP_TOOLCHAIN unset; version stdout is formal-coverage 1.9.1. Timestamps/logs are in PROGRESS.md. No src/ changes, no other checks/rules/assertions edited, no hax/F* run, no push or tag. Independent Andrew/Dudley rerun and line-by-line source review remain outstanding.
