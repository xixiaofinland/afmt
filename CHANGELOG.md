# Changelog

## [1.0.0](https://github.com/xixiaofinland/afmt/compare/v0.12.2...v1.0.0) (2026-08-04)


### ⚠ BREAKING CHANGES

* **cli:** stdout now carries only the formatted document. `--time` diagnostics move from stdout to stderr, and formatted output ends with exactly one newline instead of an unconditionally appended one.

### Features

* add configurable brace and indentation styles ([7637dc5](https://github.com/xixiaofinland/afmt/commit/7637dc5cd13e0beb3f840c3fcb08effcb128e6c4))
* add configurable javadoc star column ([7bd1ff1](https://github.com/xixiaofinland/afmt/commit/7bd1ff166f22db2c34af777d7dd3e16f50a04f2a))
* **cli:** format Apex source from stdin ([62b64fc](https://github.com/xixiaofinland/afmt/commit/62b64fc212b24fff318c294829b747cc1c438414))
* **formatter:** normalize Apex annotation casing ([27c1d92](https://github.com/xixiaofinland/afmt/commit/27c1d9267d54d55f66fdf9b6087a187b15fdaa53))
* support project-wide formatting ([6ebaf3e](https://github.com/xixiaofinland/afmt/commit/6ebaf3e140eb5687a814fbab1157a9342d82a3f8))


### Bug Fixes

* **ci:** resolve consolidated branch failures ([df9c4f1](https://github.com/xixiaofinland/afmt/commit/df9c4f1c4110f7ffc26ee69c11c042b51be58d2e))
* format anonymous Apex with top-level statements ([90769f7](https://github.com/xixiaofinland/afmt/commit/90769f7797b2921c59f31adcf86f3167478e015b))
* **formatter:** account for tab indentation width ([af625db](https://github.com/xixiaofinland/afmt/commit/af625db22128f13a6b3f6da4375187a7137c2929))
* **formatter:** suppress spaces before pending line breaks ([9d81ab6](https://github.com/xixiaofinland/afmt/commit/9d81ab675974ad393d70d37501ed5733c1ffb5bc))
* **formatter:** validate indentation configuration ([a2fb275](https://github.com/xixiaofinland/afmt/commit/a2fb275ed154a48b9371052f03113bfa0a94e14c))
* keep Apex name paths glued across chains ([64d02e3](https://github.com/xixiaofinland/afmt/commit/64d02e3fe59fe5643a8fb5b3f42e2b637f3e28be))
* prune excluded directories during discovery ([2626bb0](https://github.com/xixiaofinland/afmt/commit/2626bb06c9752131391a0f5dc8c08fe778340a00))
* stabilize comment placement idempotency ([3395471](https://github.com/xixiaofinland/afmt/commit/3395471e735be6e17e315a921f5a683ede9eb0c0))


### Documentation

* **cli:** record the stdout contract changes ([66c95bf](https://github.com/xixiaofinland/afmt/commit/66c95bf7fcdab73e6b37ef98201e41dbf6180b64))

## [0.12.2](https://github.com/xixiaofinland/afmt/compare/v0.12.1...v0.12.2) (2025-04-07)


### Bug Fixes

* allow comments to attach to 'else' node to fix idempotent issue ([8afe1c3](https://github.com/xixiaofinland/afmt/commit/8afe1c3c6ba6d22822929fb5eca3105436d56630))

## [0.12.1](https://github.com/xixiaofinland/afmt/compare/v0.12.0...v0.12.1) (2025-04-05)


### Bug Fixes

* idempotent issue [#97](https://github.com/xixiaofinland/afmt/issues/97) ([cdc84cf](https://github.com/xixiaofinland/afmt/commit/cdc84cf8115b65521f00722a418856c4f86cec67))

## [0.12.0](https://github.com/xixiaofinland/afmt/compare/v0.11.0...v0.12.0) (2025-04-04)


### Features

* add idempotent feature in battle test script ([8a9c097](https://github.com/xixiaofinland/afmt/commit/8a9c09756b4e7234ca15dd87ae93ad68cb75de28))


### Bug Fixes

* idempotent issue for continue/break/new with comments [#114](https://github.com/xixiaofinland/afmt/issues/114) ([f503c22](https://github.com/xixiaofinland/afmt/commit/f503c22a7fc63cbfd30a892961f1ada5a4be368b))
* use ai enhanced logo ([8619f16](https://github.com/xixiaofinland/afmt/commit/8619f16d085f712b3dbb0cf0b35ff05e5e076830))

## [0.11.0](https://github.com/xixiaofinland/afmt/compare/v0.10.0...v0.11.0) (2025-03-19)


### Features

* add --check flag ([7eea4ba](https://github.com/xixiaofinland/afmt/commit/7eea4bafb406f0aa5ea05ef7a875a9dae2b54242))

## [0.10.0](https://github.com/xixiaofinland/afmt/compare/v0.9.0...v0.10.0) (2025-03-18)


### Features

* show execution time in -t parameter ([4774761](https://github.com/xixiaofinland/afmt/commit/47747613ff75bde70408b25d36d195c6488b2d4c))

## [0.9.0](https://github.com/xixiaofinland/afmt/compare/v0.8.2...v0.9.0) (2025-03-14)


### Features

* support java doc ([752116e](https://github.com/xixiaofinland/afmt/commit/752116e62014dccd6ca140a305ad3128a6a23526))

## [0.8.2](https://github.com/xixiaofinland/afmt/compare/v0.8.1...v0.8.2) (2025-03-11)


### Bug Fixes

* [#67](https://github.com/xixiaofinland/afmt/issues/67) idempotent format issue ([92083f5](https://github.com/xixiaofinland/afmt/commit/92083f591f31f47f891480c01c2dda26edbc7c1e))

## [0.8.1](https://github.com/xixiaofinland/afmt/compare/v0.8.0...v0.8.1) (2025-03-04)


### Bug Fixes

* [#92](https://github.com/xixiaofinland/afmt/issues/92) indent issue for line after super() ([40fc109](https://github.com/xixiaofinland/afmt/commit/40fc1094293f926cbf3fa994ea12ad7c620933fb))

## [0.8.0](https://github.com/xixiaofinland/afmt/compare/v0.7.1...v0.8.0) (2025-02-18)


### Features

* temporarily remove extra prints in dry-run for web-service backend ([9dd430b](https://github.com/xixiaofinland/afmt/commit/9dd430bcc87d28343f18db112c4df24eb63d5f97))

## [0.7.1](https://github.com/xixiaofinland/afmt/compare/v0.7.0...v0.7.1) (2025-01-21)


### Bug Fixes

* bugfixing class_literal missing dot in print ([8d632e8](https://github.com/xixiaofinland/afmt/commit/8d632e8c8f94fb8fde55dbb27ff6980e22c9fa26))

## [0.7.0](https://github.com/xixiaofinland/afmt/compare/v0.6.0...v0.7.0) (2025-01-19)


### Features

* add github action to auto publish to creates.io when tagging ([b539614](https://github.com/xixiaofinland/afmt/commit/b5396141b888e7afa6f835422a04fba8a2ed7973))

## [0.6.0](https://github.com/xixiaofinland/afmt/compare/v0.5.4...v0.6.0) (2025-01-19)


### Features

* add install scripts for win/macos/linux and update readme ([f829f2b](https://github.com/xixiaofinland/afmt/commit/f829f2b9ec7a4584922477aa9d560b5b946663c7))


### Bug Fixes

* install-afmt.sh can't see the read section reading from web ([1bddc23](https://github.com/xixiaofinland/afmt/commit/1bddc2325298352d2e8fb8ed8deaa3088242b361))

## [0.5.4](https://github.com/xixiaofinland/afmt/compare/v0.5.3...v0.5.4) (2025-01-19)


### Bug Fixes

* binary release logic ([6c40850](https://github.com/xixiaofinland/afmt/commit/6c40850bd1b1a3364ab74c00d39e0b04809373e0))

## [0.5.3](https://github.com/xixiaofinland/afmt/compare/v0.5.2...v0.5.3) (2025-01-19)


### Bug Fixes

* binary logic ([17df967](https://github.com/xixiaofinland/afmt/commit/17df9670af25c2fcd9889236ca179db965e4ef32))

## [0.5.2](https://github.com/xixiaofinland/afmt/compare/v0.5.1...v0.5.2) (2025-01-19)


### Bug Fixes

* github action binary fix2 ([ad1b3b6](https://github.com/xixiaofinland/afmt/commit/ad1b3b6a9245dd37eba1cf23bd4196414f4b6c4e))

## [0.5.1](https://github.com/xixiaofinland/afmt/compare/v0.5.0...v0.5.1) (2025-01-19)


### Bug Fixes

* github action binary build fails ([d93aa79](https://github.com/xixiaofinland/afmt/commit/d93aa799365559b0caf549a1980253441b96b3c2))

## [0.5.0](https://github.com/xixiaofinland/afmt/compare/v0.4.0...v0.5.0) (2025-01-19)


### Features

* add install-script for linux/macOS ([0b4cc24](https://github.com/xixiaofinland/afmt/commit/0b4cc249bef5484a38443cd2bbc2b362e50c06c1))
* update release binary logic ([787451a](https://github.com/xixiaofinland/afmt/commit/787451a847ce955dcc88031e95dad74500bbdf24))

## [0.4.0](https://github.com/xixiaofinland/afmt/compare/v0.3.0...v0.4.0) (2025-01-17)


### Features

* add battle tests ([c57e16e](https://github.com/xixiaofinland/afmt/commit/c57e16ef9d260c0a01054190aadafa3b1f4b27b2))
* set github actions into parallel jobs ([ffdbc3f](https://github.com/xixiaofinland/afmt/commit/ffdbc3fc9b4c67b7006a22a9ee7b00d6193951d3))
* update CI script ([eea8ccd](https://github.com/xixiaofinland/afmt/commit/eea8ccd8237f7e5e675e50daab6d0810b82644c2))

## [0.3.0](https://github.com/xixiaofinland/afmt/compare/v0.2.0...v0.3.0) (2025-01-17)


### Features

* stream line version management ([#53](https://github.com/xixiaofinland/afmt/issues/53)) ([4a6f8b9](https://github.com/xixiaofinland/afmt/commit/4a6f8b9c07362b77b24304c8c8f8ca9cea9608e6))

## [0.2.0](https://github.com/xixiaofinland/afmt/compare/v0.1.0...v0.2.0) (2025-01-17)


### Features

* refine github action release-please ([f3705a1](https://github.com/xixiaofinland/afmt/commit/f3705a115e604abc69f8ac2ad197f89eb7160431))
* refine github actions ([6c02f7d](https://github.com/xixiaofinland/afmt/commit/6c02f7d51e69bb8a8248dd602ad73b676c601806))
* show the version info in afmt -h ([decd85c](https://github.com/xixiaofinland/afmt/commit/decd85c8a3fc5c9619e3473b3527de2b61890346))
