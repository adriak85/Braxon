#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../../.." && pwd -P)"
PREFIX="${PREFIX:-/data/data/com.termux/files/usr}"
STAMP="$(date +%Y%m%d_%H%M%S)"

STATE="$ROOT/state/nsq/cpan"
STAMP_STATE="$ROOT/state/nsq/stamps"
WRAPBIN="$ROOT/tools/nsq/cpan/bin"

mkdir -p "$STATE" "$STAMP_STATE" "$WRAPBIN"

REPORT="$STATE/cpan_surface_attach_report_$STAMP.txt"
MODULES="$STATE/cpan_module_manifest.tsv"
COMMANDS="$STATE/cpan_command_manifest.tsv"
STAMPS="$STAMP_STATE/cpan_command_surface_stamps.jsonl"
LATEST="$STATE/latest_cpan_surface_attach.txt"

tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT

{
  echo "=== NSQ CPAN SURFACE ATTACH ==="
  echo "date: $(date)"
  echo "root: $ROOT"
  echo "prefix: $PREFIX"
  echo

  echo "=== visible commands before ==="
  for c in perl cpan cpanm cpm carton prove perldoc corelist fatpack minicpan archer ark; do
    printf '%-12s ' "$c"
    command -v "$c" || true
  done
  echo

  echo "=== perl identity ==="
  perl -V:version -V:archname -V:prefix -V:siteprefix
  echo
} | tee "$REPORT"

perl -MConfig -MFile::Find -MCwd=abs_path -e '
  print "class\tmodule\tversion\tpath\n";
  my @roots;
  my %rs;
  for my $inc (@INC) {
    next unless defined $inc && -d $inc;
    my $abs = abs_path($inc) || $inc;
    next if $rs{$abs}++;
    push @roots, $abs;
  }

  sub cls {
    my ($p)=@_;
    return "SITE-CPAN" if $Config::Config{sitearch} && index($p,$Config::Config{sitearch})==0;
    return "SITE-CPAN" if $Config::Config{sitelib} && index($p,$Config::Config{sitelib})==0;
    return "VENDOR" if $Config::Config{archlib} && index($p,$Config::Config{archlib})==0;
    return "VENDOR" if $Config::Config{privlib} && index($p,$Config::Config{privlib})==0;
    return "UNKNOWN";
  }

  sub ver {
    my ($p)=@_;
    return "unknown" unless open my $fh, "<", $p;
    local $/;
    my $t=<$fh>;
    close $fh;
    return $1 if $t =~ /\$VERSION\s*=\s*["'"'"']([^"'"'"']+)["'"'"']/;
    return $1 if $t =~ /\$VERSION\s*=\s*(v?[0-9][0-9A-Za-z_.]*)/;
    return "unknown";
  }

  my %seen;
  my $arch = $Config::Config{archname} || "";

  for my $root (@roots) {
    find({
      no_chdir => 1,
      wanted => sub {
        return unless /\.pm$/;
        my $p = $File::Find::name;
        my $rel = $p;
        $rel =~ s/^\Q$root\E\/?//;
        return if $arch && $rel =~ /^\Q$arch\E\//;

        my $m = $rel;
        $m =~ s/\//::/g;
        $m =~ s/\.pm$//;
        return if $seen{$m}++;

        print cls($p), "\t", $m, "\t", ver($p), "\t", $p, "\n";
      }
    }, $root);
  }
' > "$MODULES"

for c in cpan cpanm cpm carton prove perldoc corelist fatpack minicpan archer ark; do
  command -v "$c" >> "$tmp" 2>/dev/null || true
done

find "$PREFIX/bin" -maxdepth 1 -type f -perm -u+x 2>/dev/null >> "$tmp" || true
sort -u "$tmp" -o "$tmp"

printf 'selected\ttype\tcommand\ttarget\twrapper\tsha256\troute\n' > "$COMMANDS"
: > "$STAMPS"

while IFS= read -r target; do
  [ -f "$target" ] || continue
  cmd="$(basename "$target")"
  wrapper="$WRAPBIN/$cmd"
  sha="$(sha256sum "$target" 2>/dev/null | awk '{print $1}')"
  [ -n "$sha" ] || sha="sha256-unavailable"

  {
    echo '#!/data/data/com.termux/files/usr/bin/bash'
    echo 'set -e'
    printf 'exec %q "$@"\n' "$target"
  } > "$wrapper"
  chmod +x "$wrapper"

  printf 'yes\tcommand\t%s\t%s\t%s\t%s\tnsq.surface.cpan.command\n' \
    "$cmd" "$target" "$wrapper" "$sha" >> "$COMMANDS"

  printf '{"stamp_family":"cpan_command_surface","command":"%s","target":"%s","wrapper":"%s","sha256":"%s","route":"nsq.surface.cpan.command","rule":"real commands only"}\n' \
    "$cmd" "$target" "$wrapper" "$sha" >> "$STAMPS"
done < "$tmp"

{
  echo
  echo "=== attach result ==="
  echo "modules:  $MODULES"
  echo "commands: $COMMANDS"
  echo "stamps:   $STAMPS"
  echo "wrappers: $WRAPBIN"
  echo "module_count:  $(tail -n +2 "$MODULES" | wc -l | tr -d ' ')"
  echo "command_count: $(tail -n +2 "$COMMANDS" | wc -l | tr -d ' ')"
  echo "stamp_count:   $(wc -l < "$STAMPS" | tr -d ' ')"
  echo
  echo "=== missing app command check ==="
  for pair in \
    "App::cpanminus cpanm" \
    "App::cpm cpm" \
    "App::FatPacker fatpack" \
    "CPAN::Mini minicpan" \
    "Archer archer" \
    "Ark ark"
  do
    mod="${pair% *}"
    cmd="${pair#* }"
    if perl -M"$mod" -e 'exit 0' >/dev/null 2>&1; then
      if [ -x "$WRAPBIN/$cmd" ] || command -v "$cmd" >/dev/null 2>&1; then
        echo "OK $mod -> $cmd"
      else
        echo "MODULE_PRESENT_COMMAND_MISSING $mod -> $cmd"
      fi
    else
      echo "MODULE_NOT_LOADABLE $mod -> $cmd"
    fi
  done
  echo
  echo "Activate:"
  echo "source \"$ROOT/tools/nsq/cpan/activate_cpan_surface.sh\""
} | tee -a "$REPORT"

{
  echo "report=$REPORT"
  echo "modules=$MODULES"
  echo "commands=$COMMANDS"
  echo "stamps=$STAMPS"
  echo "wrappers=$WRAPBIN"
} > "$LATEST"
