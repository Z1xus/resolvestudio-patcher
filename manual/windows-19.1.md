# windows-19.1

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
$p = (At '\x48\x89\x5C\x24\x10\x57\x48\x81\xEC\x80\x00\x00\x00\x33\xDB.....\x45\x33\xC0\x33\xD2\x48\x8B\xC8\xE8....\x85\xC0\x0F\x84....') + 15
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
| 19.1.4.11 | `50bbfe69234a81302f123a63e84b0ff7f62327ae62b6909c97261590a003366b` | `93e2499a61a9c2e972009d535fe85db0ee769094611693e0e378924fefd75ced` |

restore:

```powershell
Copy-Item "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe.bak" "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe"
```
