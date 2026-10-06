# macos-21.1

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
$p = at(qr/\x55\x48\x89\xE5\x53\x48\x83\xEC\x48.....\x48\x89\xC7\xE8....\x84\xC0\x75\x19\xE8....\x48\x89\xC7\x31\xF6\x31\xD2\xE8....\x85\xC0\x0F\x84..../s) + 9;
substr($d, $p, 5) =~ /\A\xE8....\z/s or die "unexpected bytes\n";
$c = pack("H*", "e900000000");
substr($c, 1, 4) = pack("l<", at(qr/\xBF\x01\x00\x00\x00\xE8....\xE8....\x48\x89\xC7\xE8....\xB0\x01\x48\x83\xC4\x48\x5B\x5D\xC3/s) + 0 - $p - 5);
substr($o, $p, 5) = $c;
seek F, 0, 0; print F $o;
' "/Applications/DaVinci Resolve/DaVinci Resolve.app/Contents/MacOS/Resolve"
```

arm64:

```sh
sudo perl -e '
open F, "+<:raw", $ARGV[0] or die "$!\n";
$d = $o = do { local $/; <F> };
sub at { my @m; push @m, $-[0] while $d =~ /(?=$_[0])/g; @m == 1 ? $m[0] : die "signature not found\n" }
$p = at(qr/\xFF\x83\x01\xD1\xF4\x4F\x04\xA9\xFD\x7B\x05\xA9\xFD\x43\x01\x91.......\x97\xC0\x00\x00\x37...\x97\x01\x00\x80\x52\x02\x00\x80\xD2...\x94...\x34/s) + 16;
substr($d, $p, 4) =~ /\A...\x97\z/s or die "unexpected bytes\n";
substr($o, $p, 4) = pack("V", 0x14000000 | (at(qr/\x20\x00\x80\x52...\x94...\x94...\x94\x20\x00\x80\x52\xFD\x7B\x45\xA9\xF4\x4F\x44\xA9\xFF\x83\x01\x91\xC0\x03\x5F\xD6/s) + 0 - $p) / 4 & 0x3ffffff);
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
