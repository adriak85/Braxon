#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/source_forge"
LANE="$SRC/alien_lanes/findutils"
VER="${FINDUTILS_VERSION:-4.10.0}"
PREFIX="$LANE/prefix"
BUILD="$LANE/build/findutils-$VER"
TARBALL="$SRC/downloads/findutils-$VER.tar.xz"
TERMUX_BIN="/data/data/com.termux/files/usr/bin"
STAMP="$(date +%Y%m%d_%H%M%S)"
BACKUP="$LANE/backups/termux-findutils-$STAMP.tar.gz.bak"
OUT="$TC/go_gold_alien_findutils_lane_$STAMP.log"
JOBS="${JOBS:-7}"

mkdir -p "$LANE"/{alienfile.d,src,build,prefix,reports,locks,backups,tmp} "$SRC/downloads"

{
echo "=== GO GOLD: Alien-style Braxon findutils lane ==="
date
echo "no staging; build/prove/backup/overlay/lock"

echo "=== bootstrap only ==="
pkg install -y perl clang lld make curl tar gzip xz-utils patch gettext libiconv >/dev/null 2>&1 || true

echo "=== write alienfile recipe ==="
cat > "$LANE/alienfile" <<EOF
use alienfile;

plugin 'Probe::CommandLine' => (
  command => 'find',
  args => ['--version'],
  match => qr/GNU findutils/,
);

share {
  start_url 'https://ftp.gnu.org/gnu/findutils/findutils-$VER.tar.xz';
  plugin 'Download';
  plugin 'Extract' => 'tar.xz';
  build [
    '%{configure} --prefix=%{.install.prefix} --host=aarch64-linux-android --build=aarch64-linux-android --disable-nls',
    '%{make}',
    '%{make} install',
  ];
};
EOF

echo "=== backup Termux originals before any overlay ==="
tar -czf "$BACKUP" -C "$TERMUX_BIN" find xargs 2>/dev/null
test -s "$BACKUP"
sha256sum "$BACKUP" > "$BACKUP.sha256"

echo "=== fetch source ==="
[ -f "$TARBALL" ] || curl -L -o "$TARBALL" "https://ftp.gnu.org/gnu/findutils/findutils-$VER.tar.xz"
sha256sum "$TARBALL" > "$LANE/reports/source_tarball.sha256"

echo "=== unpack clean ==="
rm -rf "$BUILD"
tar -xJf "$TARBALL" -C "$LANE/build"

echo "=== configure/build/install native prefix ==="
cd "$BUILD"

export CC="${CC:-/data/data/com.termux/files/usr/bin/clang}"
export CXX="${CXX:-/data/data/com.termux/files/usr/bin/clang++}"
export AR="${AR:-/data/data/com.termux/files/usr/bin/llvm-ar}"
export RANLIB="${RANLIB:-/data/data/com.termux/files/usr/bin/llvm-ranlib}"
export CFLAGS="-O2 -fPIC"
export CPPFLAGS="-I/data/data/com.termux/files/usr/include"
export LDFLAGS="-L/data/data/com.termux/files/usr/lib"

./configure \
  --prefix="$PREFIX" \
  --host=aarch64-linux-android \
  --build=aarch64-linux-android \
  --disable-nls

make -j "$JOBS"
make install

echo "=== prove prefix binaries ==="
"$PREFIX/bin/find" --version | tee "$LANE/reports/prefix_find_version.txt"
"$PREFIX/bin/xargs" --version | tee "$LANE/reports/prefix_xargs_version.txt"
"$PREFIX/bin/find" "$ROOT" -maxdepth 1 -type f | head -30 | tee "$LANE/reports/prefix_find_probe.txt"

echo "=== overlay Termux find/xargs ==="
install -m 0755 "$PREFIX/bin/find" "$TERMUX_BIN/find"
install -m 0755 "$PREFIX/bin/xargs" "$TERMUX_BIN/xargs"

echo "=== post-overlay proof ==="
find --version | tee "$LANE/reports/overlay_find_version.txt"
xargs --version | tee "$LANE/reports/overlay_xargs_version.txt"
find "$ROOT" -maxdepth 1 -type f | head -30 | tee "$LANE/reports/overlay_find_probe.txt"

echo "=== write env ==="
cat > "$LANE/alien_findutils_env" <<EOF
export BRAXON_ALIEN_FINDUTILS_LANE="$LANE"
export BRAXON_ALIEN_FINDUTILS_PREFIX="$PREFIX"
export PATH="$PREFIX/bin:\$PATH"
EOF
chmod +x "$LANE/alien_findutils_env"

echo "=== lock ==="
{
  echo "BRAXON_ALIEN_FINDUTILS_GO_GOLD_LOCK=1"
  date
  echo "version=$VER"
  echo "prefix=$PREFIX"
  echo "backup=$BACKUP"
  echo "overlay=find,xargs"
  find --version | head -1
  xargs --version | head -1
} > "$LANE/locks/LOCKED_ALIEN_FINDUTILS_GO_GOLD.txt"

find "$LANE/alienfile" "$LANE/alien_findutils_env" "$PREFIX/bin" "$LANE/reports" "$LANE/locks" -type f -print0 \
  | sort -z | xargs -0 sha256sum > "$LANE/locks/manifest.sha256"

echo "DONE"
echo "backup=$BACKUP"
echo "restore_if_needed:"
echo "tar -xzf \"$BACKUP\" -C \"$TERMUX_BIN\""
echo "log=$OUT"
} 2>&1 | tee "$OUT"

ln -sf "$OUT" "$TC/go_gold_alien_findutils_lane_latest.log"
