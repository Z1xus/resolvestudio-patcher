# linux-18

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
$p = at(qr/\x53\x48\x83\xEC\x30.....\x48\x89\xC7\x31\xF6\x31\xD2\xE8....\x85\xC0\x74.\xE8....\x48\x8D\x5C\x24\x08/s) + 5;
substr($d, $p, 5) =~ /\A\xE8....\z/s or die "unexpected bytes\n";
$c = pack("H*", "e900000000");
substr($c, 1, 4) = pack("l<", at(qr/\xE8....\x48\x89\xC7\xE8....\xB0\x01\xEB\x0C\x48\x8D\x7C\x24\x08/s) + 0 - $p - 5);
substr($o, $p, 5) = $c;
seek F, 0, 0; print F $o;
' /opt/resolve/bin/resolve
```

check the result with `sha256sum /opt/resolve/bin/resolve`:

| version | original | patched |
| --- | --- | --- |
| 18.0.0.36 | `4a1cd88ceafdf91bbcdb3157998ab7020fba24dbd37f453dc789b40aca3fb884` | `18db894b2aaf43edfc25041290828f0503e009931fac9493ff2f43a9e21f612b` |
| 18.6.6.7 | `6e4b8e707efe8b5e16d40dbbc22323a4be50e13e63024bac5a9675348b9eb93b` | `c3323be485812ba481a6303bb3d7f721b42d6b67027b6bac8c27279e9fa24fce` |

restore:

```sh
sudo cp -p /opt/resolve/bin/resolve.bak /opt/resolve/bin/resolve
```
