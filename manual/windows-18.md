# windows-18

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
$p = (At '\x48\x8D\x4D\x07\xFF\x15....\x48\x8D\x4D\xFF\xFF\x15....\x90\x80\x7D\x0F\x00\x0F\x85.........\x48\x8B\xC8\x45\x33\xC0\x33\xD2\xE8....\x85\xC0\x74.') + 31
if ($s.Substring($p, 5) -cnotmatch '(?s)\A\xE8....\z') { throw "unexpected bytes" }
$c = [byte[]](0xe9,0x00,0x00,0x00,0x00)
[BitConverter]::GetBytes([int]((At '\xE8....\x48\x8B\xC8\xE8....\xB3\x01\x48\x8D\x4D\x1F\xFF\x15....') + 0 - $p - 5)).CopyTo($c, 1)
$c.CopyTo($b, $p)
Copy-Item $f "$f.bak"
[IO.File]::WriteAllBytes($f, $b)
}
```

check the result with `Get-FileHash "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe"`:

| version | original | patched |
| --- | --- | --- |
| 18.0.0.36 | `a91e06a1c2f5e7d4d7ce27301c8350f9957dfe985539cec613dd54d9c386bd7d` | `dab6336880fcb771b085a3d1aa1eb9cc731d2624f25f2032345c2fabea7ddea9` |

restore:

```powershell
Copy-Item "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe.bak" "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe"
```
