# linux-21.0

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
$p = at(qr/\x53\x48\x83\xEC\x40.....\x48\x89\xC7\xE8....\x84\xC0\x75.\xE8....\x48\x89\xC7\x31\xF6\x31\xD2\xE8....\x85\xC0\x74./s) + 5;
substr($d, $p, 5) =~ /\A\xE8....\z/s or die "unexpected bytes\n";
$c = pack("H*", "e900000000");
substr($c, 1, 4) = pack("l<", at(qr/\xE8....\x48\x89\xC7\xE8....\xB0\x01\x48\x83\xC4\x40\x5B\xC3/s) + 0 - $p - 5);
substr($o, $p, 5) = $c;
seek F, 0, 0; print F $o;
' /opt/resolve/bin/resolve
```

check the result with `sha256sum /opt/resolve/bin/resolve`:

| version | original | patched |
| --- | --- | --- |
| 21.0.0.48 | `83201c3fefb5b76275abda255621db27196d0e9b22eee0e1fd491862642ea389` | `b59c39785fab2bde57d5aff37457988542e189150aad1672366aa6f979c16b54` |

restore:

```sh
sudo cp -p /opt/resolve/bin/resolve.bak /opt/resolve/bin/resolve
```
