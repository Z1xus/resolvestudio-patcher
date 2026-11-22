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
$p = at(qr/\x48\x8B\x7C\x24\x20\xBE\x02\x00\x00\x00\xBA\x08\x00\x00\x00\xE8....\x41\x09\xEE..\x48\x89\xDF\xE8/s) + 23;
substr($d, $p, 2) =~ /\A\x75\x60\z/s or die "unexpected bytes\n";
$c = pack("H*", "eb60");
substr($o, $p, 2) = $c;
for $p (all(qr/\x48\xB9\xCC\xCC\xCC\xCC\xCC\xCC\xCC\x0C\x48\x39\xC8\x0F\x87....\xE8....\x48\x89\xC7\xE8....\x48\x3D/s)) { $p += 34;
substr($d, $p, 4) =~ /\A\xC1\x27\x09\x00\z/s or die "unexpected bytes\n";
substr($o, $p, 4) = pack("H*", "ffffff7f"); }
seek F, 0, 0; print F $o;
' /opt/resolve/bin/resolve
```

check the result with `sha256sum /opt/resolve/bin/resolve`:

| version | original | patched |
| --- | --- | --- |
| 21.2.0.6 | `e38b3f27b8b15c1cd844753e9126741dd5dfdb2c56dade4cc4d5b573963cdc56` | `fa4d58d03c2e66cf3e599691bedc04a89fd2ccead248bd2ef13ff402307671c6` |

restore:

```sh
sudo cp -p /opt/resolve/bin/resolve.bak /opt/resolve/bin/resolve
```
