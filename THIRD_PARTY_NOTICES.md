# Third-party notices

formal-coverage is MIT licensed (see `LICENSE`), except for the adapted CI
scripts/tests and third-party evidence listed below. Evidence files in `runs/`
are copies of, or are generated from, third-party code and remain under their
own licenses. The CI ports carry Apache-2.0 identifiers and are modified as noted.

| Path | Origin | License |
|---|---|---|
| `ci/coverage*`, `ci/check-coverage`, `ci/write-*.py`, `ci/tests/test_{ratchet,badges,write_coverage}.py` | Adapted from [gumshoe2020/bertie-tls](https://github.com/gumshoe2020/bertie-tls) scripts at `26b54ae` (2026-11-24); modified paths, configuration and CLI options | Apache-2.0 |
| `runs/v1.7.1/*/libs/core`, `libs/rust_primitives`, `libs/hax-lib` | [hax](https://github.com/cryspen/hax) 0.4.2 F* proof libraries | Apache-2.0 |
| `runs/v1.7.1/*/libs/libcrux-models` | [Cryspen](https://github.com/cryspen) libcrux F* models | Apache-2.0 |
| `runs/v1.7.1/*/fresh`, `runs/v1.7.1/*/proofs/fstar/extraction` | F* extracted by hax from [Bertie](https://github.com/cryspen/bertie) (upstream `ef65f76` and the [gumshoe2020/bertie](https://github.com/gumshoe2020/bertie) fork) | Apache-2.0 |

The Apache License 2.0 text is in `LICENSES/Apache-2.0.txt`. Copyright in
these files belongs to their original authors.
