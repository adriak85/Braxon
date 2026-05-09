# Source-first release prep

Braxon should build as much as practical from source.

Package manager installs are bootstrap surfaces only:
- compiler
- linker
- make
- tar/gzip/xz-utils
- headers/libraries needed to build source lanes

Release-prep artifacts belong in packaged/.
Installed build products and full state registry trees are not automatically released.
