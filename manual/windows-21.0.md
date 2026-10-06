# windows-21.0

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
$p = (At '\x40\x53\x48\x83\xEC\x50............................\x4C\x8D\x05....\x48\x8D\x15....\x48\x8D\x4C\x24\x60\xFF\x15....\x90') + 6
if ($s.Substring($p, 28) -cnotmatch '(?s)\A\xE8....\x84\xC0\x74\x08\xB0\x01\x48\x83\xC4\x50\x5B\xC3\xC7\x44\x24\x20\xFF\xFF\xFF\xFF\x45\x33\xC9\z') { throw "unexpected bytes" }
$c = [byte[]](0xe8,0x00,0x00,0x00,0x00,0x48,0x8b,0xc8,0xe8,0x00,0x00,0x00,0x00,0xb0,0x01,0x48,0x83,0xc4,0x50,0x5b,0xc3,0x90,0x90,0x90,0x90,0x90,0x90,0x90)
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
| 21.0.0.48 | `470f6a5ab8a5053a9489c88c5627ae922eab760d496d9fa84a694c784042bcb9` | `6b3996090aa790800a74dbbef8136b099333de3021882c0bb972499aa64af04a` |

restore:

```powershell
Copy-Item "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe.bak" "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe"
```
