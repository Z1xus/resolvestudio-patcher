# windows-18.6

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
$p = (At '\x48\x8D\x4C\x24\x30\xE8....\x90\x80\x7C\x24\x30\x00\x0F\x85.........\x45\x33\xC0\x33\xD2\x48\x8B\xC8\xE8....\x85\xC0\x0F\x84....') + 22
if ($s.Substring($p, 5) -cnotmatch '(?s)\A\xE8....\z') { throw "unexpected bytes" }
$c = [byte[]](0xe9,0x00,0x00,0x00,0x00)
[BitConverter]::GetBytes([int]((At '\xE8....\x48\x8B\xC8\xE8....\xB3\x01\x48\x8D\x4C\x24.\xFF\x15....') + 0 - $p - 5)).CopyTo($c, 1)
$c.CopyTo($b, $p)
Copy-Item $f "$f.bak"
[IO.File]::WriteAllBytes($f, $b)
}
```

check the result with `Get-FileHash "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe"`:

| version | original | patched |
| --- | --- | --- |
| 18.6.6.7 | `406c9e3b65ae428d8f2598bc64bcfb4c3adb4555b71fc3c0a1e5f4fb2568faba` | `e60da414068b9c0a3d0cc6cae30efc0c56f81dc67d54d9ca436f613f9d7113cd` |

restore:

```powershell
Copy-Item "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe.bak" "C:\Program Files\Blackmagic Design\DaVinci Resolve\Resolve.exe"
```
