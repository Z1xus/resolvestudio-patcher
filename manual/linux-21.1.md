# linux-21.1

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
$p = at(qr/\x53\x48\x83\xEC\x40.....\x48\x89\xC7\xE8....\x84\xC0\x75\x19\xE8....\x48\x89\xC7\x31\xF6\x31\xD2\xE8....\x85\xC0\x0F\x84..../s) + 5;
substr($d, $p, 5) =~ /\A\xE8....\z/s or die "unexpected bytes\n";
$c = pack("H*", "e900000000");
substr($c, 1, 4) = pack("l<", at(qr/\xBF\x01\x00\x00\x00\xE8....\xE8....\x48\x89\xC7\xE8....\xB0\x01\x48\x83\xC4\x40\x5B\xC3/s) + 0 - $p - 5);
substr($o, $p, 5) = $c;
seek F, 0, 0; print F $o;
' /opt/resolve/bin/resolve
```

check the result with `sha256sum /opt/resolve/bin/resolve`:

| version | original | patched |
| --- | --- | --- |
| 21.0.4.5 | `1092888fe6fc8e12339a9ea0fc38985089d486a8f9b97bc8ec922649e866fbbd` | `8e068bc61b1f3e9ecd031f67463a01a17d08f99dc5fcf94ec18dee1ee62a465e` |
| 21.1.0.14 | `23d1bedf6f87fc26979cdaf1b4b5c3beef3bbcbefb34207a750450adef29fea2` | `3861df318072d83a187dd8a6aaf9136c055137c611fa28e1915fa61ff1e3e8a3` |

restore:

```sh
sudo cp -p /opt/resolve/bin/resolve.bak /opt/resolve/bin/resolve
```
