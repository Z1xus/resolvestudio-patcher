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
$p = at(qr/\x48\x89\xDF\xE8....\x66\x66\x66\x66\x66\x66\x2E\x0F\x1F\x84\x00\x00\x00\x00\x00\x55\x48\x89\xE5....\x5D\xC3\x66\x0F\x1F\x44\x00\x00\x55\x48\x89\xE5\x0F\xB6\x47\x0A\x5D\xC3/s) + 27;
substr($d, $p, 4) =~ /\A\x0F\xB6\x47\x09\z/s or die "unexpected bytes\n";
$c = pack("H*", "31c0b001");
substr($o, $p, 4) = $c;
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
$p = at(qr/....\xC0\x03\x5F\xD6\x00\x28\x40\x39\xC0\x03\x5F\xD6\x08\x20\x40\x39\x1F\x05\x00\x71\xA1\x03\x00\x54/s) + 0;
substr($d, $p, 4) =~ /\A\x00\x24\x40\x39\z/s or die "unexpected bytes\n";
$c = pack("H*", "20008052");
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
