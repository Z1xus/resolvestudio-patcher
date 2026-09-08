# resolvestudio-patcher

a patcher for davinci resolve studio


| platform | tested versions | version range |
| --- | --- | --- |
| linux x86-64 | 21.1.0.0014 | 21.1.x |
| windows x86-64 | 21.1.0.0014 | 21.1.x |

other versions in the range may be supported but are untested.  
(please open an issue with any working versions missing from the tested list so i can add them)

you can download studio releases from [blackmagic design support](https://www.blackmagicdesign.com/support/)  

> [!NOTE]
> this project is for educational and research purposes only. it does not endorse bypassing software licenses or using software without a valid license.

## usage

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

(backups are saved in `<path>.backups/`)

to check a build outside the version range:

```sh
resolvestudio-patcher check <path> --try-profile linux-21.1
```

use the same option with `patch` to apply it  
use `windows-21.1` for windows  
a signature match does not guarantee that resolve will work!!!

## trust

the release workflow builds from source and includes sha256 checksums and the
source commit and compiler version in `BUILD.txt`. you can build from source
with `.github/build.sh` using the same commit and compiler to compare binaries

published releases are immutable.

## build

requires rust stable and a c++ toolchain (visual studio c++ build tools on windows).

```sh
cargo build --release --locked
```

if you wanna contribute signatures, add them to `src/profiles/`. i don't have docs for it yet, but it should be pretty straightforward
