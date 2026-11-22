# macos-21.2

generated from `src/profiles/` by `cargo run --example manual`

close resolve, then back up the executable:

```sh
sudo cp -pn "/Applications/DaVinci Resolve/DaVinci Resolve.app/Contents/MacOS/Resolve" "/Applications/DaVinci Resolve/Resolve.bak"
```

patch each slice that `lipo -archs "/Applications/DaVinci Resolve/DaVinci Resolve.app/Contents/MacOS/Resolve"` lists.

x86_64:

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
$p = at(qr/\x48\xD1\xC9\x48\x39\xC1......\x48\x8D\x3D/s) + 6;
substr($d, $p, 6) =~ /\A\x0F\x87\x20\x03\x00\x00\z/s or die "unexpected bytes\n";
$c = pack("H*", "e92103000090");
substr($o, $p, 6) = $c;
for $p (all(qr/\xE8....\x48\x89\xC7\xE8....\x48\x3D/s)) { $p += 15;
substr($d, $p, 4) =~ /\A\xC1\x27\x09\x00\z/s or die "unexpected bytes\n";
substr($o, $p, 4) = pack("H*", "ffffff7f"); }
seek F, 0, 0; print F $o;
' "/Applications/DaVinci Resolve/DaVinci Resolve.app/Contents/MacOS/Resolve"
```

arm64:

```sh
sudo perl -e '
open F, "+<:raw", $ARGV[0] or die "$!\n";
$d = $o = do { local $/; <F> };
sub at { my @m; push @m, $-[0] while $d =~ /(?=$_[0])/g; @m == 1 ? $m[0] : die "signature not found\n" }
sub all { my @m; push @m, $-[0] while $d =~ /(?=$_[0])/g; @m or die "signature not found\n"; @m }
$p = at(qr/.......\x97\xC0\x00\x00\x37...\x97\x01\x00\x80\x52\x02\x00\x80\xD2...\x94...\x34/s) + 0;
substr($d, $p, 4) =~ /\A...\x97\z/s or die "unexpected bytes\n";
substr($o, $p, 4) = pack("V", 0x14000000 | (at(qr/\x20\x00\x80\x52...\x94...\x94...\x94\x20\x00\x80\x52/s) + 0 - $p) / 4 & 0x3ffffff);
$p = at(qr/\xE8\xE7\x02\xB2\xA8\x99\x99\xF2\xE9\xE7\x01\xB2\x29\xED\x7D\x92\x08\x24\x08\x9B\x08\x05\xC8\x93\x1F\x01\x09\xEB..../s) + 28;
substr($d, $p, 4) =~ /\A\xA8\x17\x00\x54\z/s or die "unexpected bytes\n";
$c = pack("H*", "bd000014");
substr($o, $p, 4) = $c;
for $p (all(qr/...\x94...\x94\x28\xF8\x84\x52....\x1F\x00\x08\xEB/s)) { $p += 12;
substr($d, $p, 4) =~ /\A\x28\x01\xA0\x72\z/s or die "unexpected bytes\n";
substr($o, $p, 4) = pack("H*", "e8ffaf72"); }
seek F, 0, 0; print F $o;
' "/Applications/DaVinci Resolve/DaVinci Resolve.app/Contents/MacOS/Resolve"
```

sign:

```sh
sudo codesign --force --sign - --preserve-metadata=entitlements "/Applications/DaVinci Resolve/DaVinci Resolve.app/Contents/MacOS/Resolve"
```

restore:

```sh
sudo cp -p "/Applications/DaVinci Resolve/Resolve.bak" "/Applications/DaVinci Resolve/DaVinci Resolve.app/Contents/MacOS/Resolve"
```
