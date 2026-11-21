# linux-21.2

generated from `src/profiles/` by `cargo run --example manual`

close resolve, then back up the executable:

```sh
sudo cp -pn /opt/resolve/bin/resolve /opt/resolve/bin/resolve.bak
```

patch:

```sh
sudo perl -e '
open F, "+<:raw", $ARGV[0] or die "$!\n";
$d = $o = do { local $/; <F> };
sub at { my @m; push @m, $-[0] while $d =~ /(?=$_[0])/g; @m == 1 ? $m[0] : die "signature not found\n" }
sub all { my @m; push @m, $-[0] while $d =~ /(?=$_[0])/g; @m or die "signature not found\n"; @m }
$p = at(qr/.....\x48\x89\xC7\xE8....\x84\xC0\x75\x19\xE8....\x48\x89\xC7\x31\xF6\x31\xD2\xE8....\x85\xC0\x0F\x84..../s) + 0;
substr($d, $p, 5) =~ /\A\xE8....\z/s or die "unexpected bytes\n";
$c = pack("H*", "e900000000");
substr($c, 1, 4) = pack("l<", at(qr/\xBF\x01\x00\x00\x00\xE8....\xE8....\x48\x89\xC7\xE8....\xB0\x01/s) + 0 - $p - 5);
substr($o, $p, 5) = $c;
$p = at(qr/\x48\x89\xC3\x48\x8D\x7C\x24\x18\xE8....\x48\x8D\x7C\x24\x10\xE8....\x48\x8D\x7C\x24\x08\xE8....\x48\x89\xDF\xE8....\x0F\x1F\x80\x00\x00\x00\x00.....\x66\x66\x2E\x0F\x1F\x84\x00\x00\x00\x00\x00/s) + 48;
substr($d, $p, 5) =~ /\A\x0F\xB6\x47\x09\xC3\z/s or die "unexpected bytes\n";
$c = pack("H*", "b001c39090");
substr($o, $p, 5) = $c;
for $p (all(qr/\x48\xB9\xCC\xCC\xCC\xCC\xCC\xCC\xCC\x0C\x48\x39\xC8\x0F\x87....\xE8....\x48\x89\xC7\xE8....\x48\x3D/s)) { $p += 34;
substr($d, $p, 4) =~ /\A\xC1\x27\x09\x00\z/s or die "unexpected bytes\n";
substr($o, $p, 4) = pack("H*", "ffffff7f"); }
seek F, 0, 0; print F $o;
' /opt/resolve/bin/resolve
```

check the result with `sha256sum /opt/resolve/bin/resolve`:

| version | original | patched |
| --- | --- | --- |
| 21.2.0.6 | `e38b3f27b8b15c1cd844753e9126741dd5dfdb2c56dade4cc4d5b573963cdc56` | `bfd76d22d3659b9808ecac25d2d9125081f5cb70cb84bea6ea09124ae2d7d32b` |

restore:

```sh
sudo cp -p /opt/resolve/bin/resolve.bak /opt/resolve/bin/resolve
```
