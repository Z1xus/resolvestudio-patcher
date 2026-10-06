# windows-19

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
$p = (At '\x48\x89\x5C\x24\x10\x57\x48\x81\xEC\x80\x00\x00\x00\x33\xFF.....\x45\x33\xC0\x33\xD2\x48\x8B\xC8\xE8....') + 15
if ($s.Substring($p, 5) -cnotmatch '(?s)\A\xE8....\z') { throw "unexpected bytes" }
$c = [byte[]](0xe9,0x00,0x00,0x00,0x00)
[BitConverter]::GetBytes([int]((At '\xE8....\x48\x8B\xC8\xE8....\xB0\x01\x48\x8B\x9C\x24\x98\x00\x00\x00\x48\x81\xC4\x80\x00\x00\x00\x5F\xC3') + 0 - $p - 5)).CopyTo($c, 1)
$c.CopyTo($b, $p)
Copy-Item $f "$f.bak"
[IO.File]::WriteAllBytes($f, $b)
}
```

check the result with `Get-FileHash "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe"`:

| version | original | patched |
| --- | --- | --- |
| 19.0.0.69 | `36ba6b56c540ba74cce5c2525519f95f9c8864ff69b835e920aada4ddd9c3996` | `ac03226900011a171f5fe6dc648ac03113d769c9f35ccad56b310c47152eceef` |

restore:

```powershell
Copy-Item "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe.bak" "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe"
```
