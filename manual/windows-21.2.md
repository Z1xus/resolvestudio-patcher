# windows-21.2

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
$t = (At '\xE8....\x48\x8B\xC8\xE8....\xB3\x01\x48\x8D\x4C\x24.\xFF\x15....') + 0
if ($b[$t] -ne 0xe8) { throw "invalid call reference" }
$t += 5 + [BitConverter]::ToInt32($b, $t + 1)
[BitConverter]::GetBytes([int]($t - $p - 5)).CopyTo($c, 1)
$t = (At '\xE8....\x48\x8B\xC8\xE8....\xB3\x01\x48\x8D\x4C\x24.\xFF\x15....') + 8
if ($b[$t] -ne 0xe8) { throw "invalid call reference" }
$t += 5 + [BitConverter]::ToInt32($b, $t + 1)
[BitConverter]::GetBytes([int]($t - $p - 13)).CopyTo($c, 9)
$c.CopyTo($b, $p)
$m = [regex]::Matches($s, "(?s)(?=\xB8\x67\x66\x66\x66\xF7\xE9\xC1\xFA\x03\x8B\xC2\xC1\xE8\x1F\x03\xD0\x8D\x04\x92\xC1\xE0\x02\x3B\xC8\x0F\x85....\xE8....\x48\x8B\xC8\xE8....\x3D)")
if ($m.Count -lt 1) { throw "signature not found" }
foreach ($x in $m) {
$p = $x.Index + 45
if ($s.Substring($p, 4) -cnotmatch '(?s)\A\xC0\x27\x09\x00\z') { throw "unexpected bytes" }
$c = [byte[]](0xff,0xff,0xff,0x7f)
$c.CopyTo($b, $p)
}
Copy-Item $f "$f.bak"
[IO.File]::WriteAllBytes($f, $b)
}
```

check the result with `Get-FileHash "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe"`:

| version | original | patched |
| --- | --- | --- |
| 21.2.0.6 | `58ea8835bc2f3a8b5266978178f0a3833474ad3095e4e9c1b319d9a7e50ae150` | `2908c31e9a3135e34beffa7e73e7ebf1e3699ef23d443823bac75677623f71fa` |

restore:

```powershell
Copy-Item "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe.bak" "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe"
```
