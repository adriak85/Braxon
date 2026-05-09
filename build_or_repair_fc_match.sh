#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="/data/data/com.termux/files/home/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
OUT="$TC/build_or_repair_fc_match_$(date +%Y%m%d_%H%M%S).log"
LOCKDIR="$TC/locks/braxon_fontconfig_fc_match"

mkdir -p "$TC/tmp" "$LOCKDIR"

{
  echo "=== repair fontconfig / fc-match ==="
  date

  echo
  echo "=== current probe ==="
  command -v fc-match || true
  pkg files fontconfig 2>/dev/null | grep -E '/fc-match$|/fontconfig$|/libfontconfig' || true

  echo
  echo "=== reinstall fontconfig ==="
  pkg update -y
  pkg reinstall -y fontconfig || pkg install -y fontconfig

  echo
  echo "=== locate fc-match ==="
  command -v fc-match
  fc-match --version
  fc-match sans | head -n 5

  echo
  echo "=== pkg-config fontconfig ==="
  pkg-config --exists fontconfig
  pkg-config --modversion fontconfig
  pkg-config --cflags fontconfig
  pkg-config --libs fontconfig

  echo
  echo "=== C compile smoke against fontconfig ==="
  TMP="$TC/tmp/fontconfig_probe"
  rm -rf "$TMP"
  mkdir -p "$TMP"

  cat > "$TMP/fc_probe.c" <<'C'
#include <stdio.h>
#include <fontconfig/fontconfig.h>

int main(void) {
    if (!FcInit()) {
        fprintf(stderr, "FcInit failed\n");
        return 2;
    }

    FcPattern *pat = FcNameParse((const FcChar8 *)"sans");
    FcConfigSubstitute(NULL, pat, FcMatchPattern);
    FcDefaultSubstitute(pat);

    FcResult result;
    FcPattern *font = FcFontMatch(NULL, pat, &result);
    if (!font) {
        fprintf(stderr, "FcFontMatch failed\n");
        FcPatternDestroy(pat);
        FcFini();
        return 3;
    }

    FcChar8 *file = NULL;
    if (FcPatternGetString(font, FC_FILE, 0, &file) == FcResultMatch) {
        printf("fontconfig match ok: %s\n", file);
    } else {
        printf("fontconfig match ok: no file field\n");
    }

    FcPatternDestroy(font);
    FcPatternDestroy(pat);
    FcFini();
    return 0;
}
C

  clang "$TMP/fc_probe.c" \
    $(pkg-config --cflags fontconfig) \
    $(pkg-config --libs fontconfig) \
    -o "$TMP/fc_probe"

  "$TMP/fc_probe"
  file "$TMP/fc_probe"

  echo
  echo "=== lock fc-match ==="
  {
    echo "BRAXON_FONTCONFIG_FC_MATCH_LOCK=1"
    date
    command -v fc-match
    fc-match --version
    pkg-config --modversion fontconfig
    fc-match sans | head -n 5
  } > "$LOCKDIR/LOCKED_FONTCONFIG_FC_MATCH.txt"

  find "$(command -v fc-match)" "$TMP/fc_probe" \
    -type f -print0 | sort -z | xargs -0 sha256sum > "$LOCKDIR/manifest.sha256"

  echo
  echo "DONE"
  echo "log: $OUT"
  echo "lock: $LOCKDIR/LOCKED_FONTCONFIG_FC_MATCH.txt"
} 2>&1 | tee "$OUT"

ln -sf "$OUT" "$TC/build_or_repair_fc_match_latest.log"
