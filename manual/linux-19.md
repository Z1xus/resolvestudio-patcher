# linux-19

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
$p = at(qr/\x53\x48\x83\xEC\x40.....\x48\x89\xC7\x31\xF6\x31\xD2\xE8....\x85\xC0\x74.\xE8....\x48\x89\xC7\xE8....\x84\xC0\x74./s) + 5;
substr($d, $p, 5) =~ /\A\xE8....\z/s or die "unexpected bytes\n";
$c = pack("H*", "e900000000");
substr($c, 1, 4) = pack("l<", at(qr/\xE8....\x48\x89\xC7\xE8....\xB0\x01\xE9....\x48\xC7\xC0/s) + 0 - $p - 5);
substr($o, $p, 5) = $c;
seek F, 0, 0; print F $o;
' /opt/resolve/bin/resolve
```

check the result with `sha256sum /opt/resolve/bin/resolve`:

| version | original | patched |
| --- | --- | --- |
| 19.0.0.69 | `137766a536d50b9fec37ec0e53115445db85f285f09d9e187e94858a8ccd7594` | `2b1c8b943444fd9a732ec077df20967c6ce1fb68878c85eb1a800607320f1257` |
| 19.1.4.11 | `bbd78d9c768c7d2ff284134bb7e900fe35c889a435a09ab69d7ccb46aa6d44ae` | `b1d028e74985262b9a48b01c1a2af9a1e8319dc4e952af3e2e36378624af3aed` |

restore:

```sh
sudo cp -p /opt/resolve/bin/resolve.bak /opt/resolve/bin/resolve
```
