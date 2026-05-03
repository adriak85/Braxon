#!/data/data/com.termux/files/usr/bin/bash
# Source this before building DBD::mysql:
#   source "/data/data/com.termux/files/home/Braxon/tools/nsq/cpan/mysql_mariadb_env.sh"

export MYSQL_CONFIG="/data/data/com.termux/files/home/Braxon/tools/nsq/cpan/bin/mysql_config"
export DBD_MYSQL_CONFIG="/data/data/com.termux/files/home/Braxon/tools/nsq/cpan/bin/mysql_config"

export BRAXON_MARIADB_LIBDIR="/data/data/com.termux/files/usr/lib/aarch64-linux-android/"
export BRAXON_MARIADB_FOUND_DIR="/data/data/com.termux/files/usr/lib/aarch64-linux-android"

export PATH="/data/data/com.termux/files/usr/bin:$PATH"

export LD_LIBRARY_PATH="/data/data/com.termux/files/usr/lib/aarch64-linux-android/:/data/data/com.termux/files/usr/lib/aarch64-linux-android:/data/data/com.termux/files/usr/lib:/data/data/com.termux/files/usr/lib/aarch64-linux-android${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
export LIBRARY_PATH="/data/data/com.termux/files/usr/lib/aarch64-linux-android/:/data/data/com.termux/files/usr/lib/aarch64-linux-android:/data/data/com.termux/files/usr/lib:/data/data/com.termux/files/usr/lib/aarch64-linux-android${LIBRARY_PATH:+:$LIBRARY_PATH}"

export CFLAGS="-I/data/data/com.termux/files/usr/include/mariadb -I/data/data/com.termux/files/usr/include/mariadb/mysql ${CFLAGS:-}"
export CPPFLAGS="-I/data/data/com.termux/files/usr/include/mariadb -I/data/data/com.termux/files/usr/include/mariadb/mysql ${CPPFLAGS:-}"
export LDFLAGS="-L/data/data/com.termux/files/usr/lib/aarch64-linux-android/ -L/data/data/com.termux/files/usr/lib/aarch64-linux-android -L/data/data/com.termux/files/usr/lib -L/data/data/com.termux/files/usr/lib/aarch64-linux-android -Wl,-rpath=/data/data/com.termux/files/usr/lib/aarch64-linux-android/ -Wl,-rpath=/data/data/com.termux/files/usr/lib/aarch64-linux-android ${LDFLAGS:-}"

echo "MariaDB / DBD::mysql env active"
echo "  MYSQL_CONFIG=$MYSQL_CONFIG"
echo "  BRAXON_MARIADB_LIBDIR=$BRAXON_MARIADB_LIBDIR"
echo "  BRAXON_MARIADB_FOUND_DIR=$BRAXON_MARIADB_FOUND_DIR"
echo "  LD_LIBRARY_PATH=$LD_LIBRARY_PATH"
