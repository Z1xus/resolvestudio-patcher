# windows-20

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
| 20.0.0.49 | `8b2dc96b4118c22df51c5aa9c07175c34b014e624259a8a4da591f452a360c20` | `9e92359dec5adce36e38cd768773880f424937e68a2c363180bb63882875faf9` |
| 20.3.3.10 | `9f800eb095d702adbb224c3e2b55f1ad0c55f1a116aaa7a2a936cd88c45a3abc` | `efdfa2c6cc9b5d744393ce6ab8be368896f2fb266800ab5c1e4470badc6bd5a5` |

restore:

```powershell
Copy-Item "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe.bak" "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe"
```
