# resolvestudio-patcher

<picture><source media="(prefers-color-scheme: dark)" srcset="https://www.shieldcn.dev/github/downloads/Z1xus/resolvestudio-patcher.svg?variant=secondary&amp;size=xs&amp;mode=dark"><img alt="Total downloads" src="https://www.shieldcn.dev/github/downloads/Z1xus/resolvestudio-patcher.svg?variant=secondary&amp;size=xs&amp;mode=light"></picture>
<picture><source media="(prefers-color-scheme: dark)" srcset="https://www.shieldcn.dev/github/last-commit/Z1xus/resolvestudio-patcher.svg?variant=secondary&amp;size=xs&amp;mode=dark"><img alt="Last commit" src="https://www.shieldcn.dev/github/last-commit/Z1xus/resolvestudio-patcher.svg?variant=secondary&amp;size=xs&amp;mode=light"></picture>

a patcher for davinci resolve studio


| platform | tested versions | version range |
| --- | --- | --- |
| linux x86-64 | 18.0.0.0036, 18.6.6.0007, 19.0.0.0069, 19.1.4.0011, 20.0.0.0049, 20.3.3.0010, 21.0.0.0048, 21.0.4.0005, 21.1.0.0014 | 18.x, 19.x, 20.x, 21.0.x, 21.1.x |
| windows x86-64 | 18.0.0.0036, 18.6.6.0007, 19.0.0.0069, 19.1.4.0011, 20.0.0.0049, 20.3.3.0010, 21.0.0.0048, 21.0.4.0005, 21.1.0.0014 | 18.0.x, 18.6.x, 19.0.x, 19.1.x, 20.x, 21.0.x, 21.1.x |
| macos x86-64 and arm64 | 18.0.0.0036, 18.6.6.0007, 19.0.0.0069, 19.1.4.0011, 20.0.0.0049, 20.3.3.0010, 21.0.0.0048, 21.0.4.0005, 21.1.0.0014 | 18.x, 19.x, 20.x, 21.0.x, 21.1.x |

other versions in the range may be supported but are untested.  
(please open an issue with any working versions missing from the tested list so i can add them)

you can download studio releases from [blackmagic design support](https://www.blackmagicdesign.com/support/)  

> [!NOTE]
> this project is for educational and research purposes only. it does not endorse bypassing software licenses or using software without a valid license.

## usage

`<path>` is the resolve executable, its install folder or `.app` bundle.

first you might want to run a check to verify signatures:

```sh
resolvestudio-patcher check <path>
```

to apply the patch, close resolve and run:

```sh
resolvestudio-patcher patch <path>
```

to undo the patch, run:

```sh
resolvestudio-patcher restore <path>
```

(backups are saved in `<executable>.backups/`)

to remove backups:

```sh
resolvestudio-patcher cleanup <path>
```

to check a build outside the version range:

```sh
resolvestudio-patcher check <path> --try-profile linux-21.1
```

use the same option with `patch` to apply it  
a signature match does not guarantee that resolve will work!!!

## for all the paranoid freaks

every release is built from source by the [release workflow](.github/workflows/nightly-release.yml), so no binary is ever uploaded by hand. the builds are [reproducible](https://reproducible-builds.org/docs/definition/), which means you can run [`.github/build.sh`](.github/build.sh) on the same commit with the same compiler (both are written down in `BUILD.txt`) and compare your binary against `SHA256SUMS` from the [release](https://github.com/Z1xus/resolvestudio-patcher/releases/latest)

published releases are also [immutable](https://docs.github.com/en/code-security/concepts/supply-chain-security/immutable-releases), so i can't quietly swap an asset later

and if you still do not trust resolvestudio-patcher, please proceed to [`manual/`](manual/). it has the same patches as plain perl and powershell commands (one file per profile) that you can read before you run them, generated from `src/profiles/` with `cargo run --example manual`

## build

requires rust stable and a c++ toolchain (visual studio c++ build tools on windows).

```sh
cargo build --release --locked
```

if you wanna contribute signatures, add them to `src/profiles/`. i don't have docs for it yet, but it should be pretty straightforward
