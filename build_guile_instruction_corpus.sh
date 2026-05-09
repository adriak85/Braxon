#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail

ROOT="$HOME/Braxon"
TC="$ROOT/state/full_android_language_toolchain"
SRC="$TC/source_forge"
LANE="$SRC/guile_nsq_logic"
CORPUS="$LANE/instruction_corpus"
PACKAGED="$ROOT/packaged"
STAMP="$(date +%Y%m%d_%H%M%S)"

mkdir -p "$CORPUS"/{raw,index,compiled,reports,locks} "$PACKAGED/guile-instructions"

echo "=== collect docs into Guile instruction corpus ==="

find "$ROOT" \
  \( -path "$ROOT/.git" -o -path "$ROOT/target" -o -path "$ROOT/state/full_android_language_toolchain/source_forge/install" \) -prune -o \
  -type f \
  \( -name '*.md' -o -name '*.txt' -o -name '*.json' -o -name '*.scm' -o -name '*.7' -o -name '*.1' -o -name '*.8' \) \
  -print | sort > "$CORPUS/index/source_files.txt"

while IFS= read -r f; do
  rel="${f#$ROOT/}"
  safe="$(printf '%s' "$rel" | tr '/ ' '__')"
  cp -f "$f" "$CORPUS/raw/$safe"
done < "$CORPUS/index/source_files.txt"

cat > "$CORPUS/index/instruction_index.scm" <<EOF
;; generated Braxon Guile instruction corpus index
(define-module (braxon instruction-index)
  #:export (braxon-instruction-files braxon-instruction-root))

(define braxon-instruction-root "$CORPUS/raw")

(define braxon-instruction-files
'(
EOF

while IFS= read -r f; do
  rel="${f#$ROOT/}"
  safe="$(printf '%s' "$rel" | tr '/ ' '__')"
  printf '  ("%s" . "%s")\n' "$rel" "$CORPUS/raw/$safe"
done < "$CORPUS/index/source_files.txt" >> "$CORPUS/index/instruction_index.scm"

cat >> "$CORPUS/index/instruction_index.scm" <<'EOF'
))
EOF

cat > "$CORPUS/index/query_instructions.scm" <<'EOF'
(use-modules (ice-9 regex) (ice-9 rdelim) (braxon instruction-index))

(define (file-contains? path needle)
  (call-with-input-file path
    (lambda (port)
      (let loop ((line (read-line port)))
        (cond
          ((eof-object? line) #f)
          ((string-contains (string-downcase line) needle) #t)
          (else (loop (read-line port))))))))

(define (main args)
  (let ((needle (if (> (length args) 1)
                    (string-downcase (cadr args))
                    "braxon")))
    (for-each
      (lambda (pair)
        (let ((name (car pair))
              (path (cdr pair)))
          (catch #t
            (lambda ()
              (when (file-contains? path needle)
                (display name)
                (newline)))
            (lambda _ #f))))
      braxon-instruction-files)))

(main (command-line))
EOF

cat > "$ROOT/scripts/query_braxon_guile_instructions.sh" <<EOF
#!/data/data/com.termux/files/usr/bin/bash
set -euo pipefail
ROOT="\$HOME/Braxon"
TC="\$ROOT/state/full_android_language_toolchain"
SRC="\$TC/source_forge"
LANE="\$SRC/guile_nsq_logic"
source "\$LANE/guile_nsq_env" 2>/dev/null || true
export GUILE_LOAD_PATH="\$LANE/instruction_corpus/index:\${GUILE_LOAD_PATH:-}"
guile "\$LANE/instruction_corpus/index/query_instructions.scm" "\${1:-braxon}"
EOF
chmod +x "$ROOT/scripts/query_braxon_guile_instructions.sh"

"$ROOT/scripts/query_braxon_guile_instructions.sh" source > "$CORPUS/reports/query_source.txt" || true
"$ROOT/scripts/query_braxon_guile_instructions.sh" nsq > "$CORPUS/reports/query_nsq.txt" || true
"$ROOT/scripts/query_braxon_guile_instructions.sh" watermark > "$CORPUS/reports/query_watermark.txt" || true

{
  echo "BRAXON_GUILE_INSTRUCTION_CORPUS_LOCK=1"
  date
  echo "corpus=$CORPUS"
  echo "files=$(wc -l < "$CORPUS/index/source_files.txt")"
} > "$CORPUS/locks/LOCKED_GUILE_INSTRUCTION_CORPUS.txt"

find "$CORPUS" -type f -print0 | sort -z | xargs -0 sha256sum > "$CORPUS/locks/manifest.sha256"

rsync -a "$CORPUS/index" "$CORPUS/reports" "$CORPUS/locks" "$PACKAGED/guile-instructions/"

git add scripts/query_braxon_guile_instructions.sh packaged/guile-instructions
git status --short

echo "DONE"
echo "query: scripts/query_braxon_guile_instructions.sh nsq"
