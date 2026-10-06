# linux-20

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
$p = at(qr/\x53\x48\x83\xEC\x40.....\x48\x89\xC7\x31\xF6\x31\xD2\xE8....\x85\xC0\x74./s) + 5;
substr($d, $p, 5) =~ /\A\xE8....\z/s or die "unexpected bytes\n";
$c = pack("H*", "e900000000");
substr($c, 1, 4) = pack("l<", at(qr/\xE8....\x48\x89\xC7\xE8....\x84\xC0\x74.\xE8....\x48\x89\xC7\xE8....\xB0\x01/s) + 17 - $p - 5);
substr($o, $p, 5) = $c;
seek F, 0, 0; print F $o;
' /opt/resolve/bin/resolve
```

check the result with `sha256sum /opt/resolve/bin/resolve`:

| version | original | patched |
| --- | --- | --- |
| 20.0.0.49 | `f4e259d4bc9a8db672745c73cb94e63fb31ffff4a7688ff8dd5b073ef3346c8b` | `6e1d522b36c3141d84efb90c51373462908cc1c8ada5701c36c5da0806766cfe` |
| 20.3.3.10 | `e5ca88cce85a6aa1d749bc24ed59c22fbd06918fb0a40c926abc9d99cf5eaf48` | `d76f99cbab46272c2833cc6a72f8cd45435ca0d3b0efa1db47d8fddf5093a95d` |

restore:

```sh
sudo cp -p /opt/resolve/bin/resolve.bak /opt/resolve/bin/resolve
```
