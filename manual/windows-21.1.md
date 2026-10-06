# windows-21.1

generated from `src/profiles/` by `cargo run --example manual`

close resolve, then run in powershell as administrator. the backup is saved as `Resolve.exe.bak`:

```powershell
& {
$f = "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe"
$b = [IO.File]::ReadAllBytes($f)
$s = [Text.Encoding]::GetEncoding(28591).GetString($b)
function At($re) {
    $m = [regex]::Matches($s, "(?s)(?=$re)")
    if ($m.Count -ne 1) { throw "signature not found" }
    $m[0].Index
}
$p = (At '\x40\x53\x48\x81\xEC\x80\x00\x00\x00...............................\x4C\x8D\x05....\x48\x8D\x15....\x48\x8D\x8C\x24\x90\x00\x00\x00\xFF\x15....\x90') + 9
if ($s.Substring($p, 31) -cnotmatch '(?s)\A\xE8....\x84\xC0\x74\x0B\xB0\x01\x48\x81\xC4\x80\x00\x00\x00\x5B\xC3\xC7\x44\x24\x20\xFF\xFF\xFF\xFF\x45\x33\xC9\z') { throw "unexpected bytes" }
$c = [byte[]](0xe8,0x00,0x00,0x00,0x00,0x48,0x8b,0xc8,0xe8,0x00,0x00,0x00,0x00,0xb0,0x01,0x48,0x81,0xc4,0x80,0x00,0x00,0x00,0x5b,0xc3,0x90,0x90,0x90,0x90,0x90,0x90,0x90)
[BitConverter]::GetBytes([int]((At '\x48\x89\x5C\x24\x10\x57\x48\x83\xEC\x30\xE8....\x48\x8B\xD8\x48\x83\x38\x00\x75.\x48\x8D\x78\x08\x48\x89\x7C\x24\x20\xC6\x44\x24\x28\x00\x48\x8B\xCF\xE8....\x85\xC0\x75.\x81\x7F\x4C\xFF\xFF\xFF\x7F\x74.\xC6\x44\x24\x28\x01\x48\x83\x3B\x00\x75.\xB9\xB0\x08\x00\x00') + 0 - $p - 5)).CopyTo($c, 1)
[BitConverter]::GetBytes([int]((At '\x48\x8D\x41\x30\xB9\x00\x04\x00\x00\x0F\x1F\x80\x00\x00\x00\x00\xC6\x80\x00\x04\x00\x00\x01\xC6\x00\x01\x48\x8D\x40\x01\x48\x83\xE9\x01\x75\xEC\xC3') + 0 - $p - 13)).CopyTo($c, 9)
$c.CopyTo($b, $p)
Copy-Item $f "$f.bak"
[IO.File]::WriteAllBytes($f, $b)
}
```

check the result with `Get-FileHash "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe"`:

| version | original | patched |
| --- | --- | --- |
| 21.0.4.5 | `5dd37102210dc1907dd196240c2cf47186a1cb2686cab0357754927b7e688d39` | `ba4ec5e96adb7089d0611c1fad9e6c39198c4f81fb97487f310d836481077402` |
| 21.1.0.14 | `0a8f20c70851e629c1753ed537801d1df0b5162ad98a5ab399e4abc7b77df236` | `b9676fae37c87f3df8ccf67dd65e2e82c8e22ebac10d4b19adc7981e8ba72c11` |

restore:

```powershell
Copy-Item "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe.bak" "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe"
```
