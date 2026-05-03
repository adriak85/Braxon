#!/data/data/com.termux/files/usr/bin/bash
# Source this before native CPAN/CMake builds:
#   source "/data/data/com.termux/files/home/Braxon/tools/nsq/cpan/native_cmake_mariadb_env.sh"

export PREFIX="/data/data/com.termux/files/usr"

export CC="/data/data/com.termux/files/usr/bin/clang"
export CXX="/data/data/com.termux/files/usr/bin/clang++"
export MAKE="/data/data/com.termux/files/usr/bin/make"
export CMAKE="/data/data/com.termux/files/usr/bin/cmake"
export PKG_CONFIG="/data/data/com.termux/files/usr/bin/pkg-config"

# Use the real Termux mysql_config, not the Braxon wrapper.
export MYSQL_CONFIG="/data/data/com.termux/files/usr/bin/mysql_config"
export DBD_MYSQL_CONFIG="/data/data/com.termux/files/usr/bin/mysql_config"
export MARIADB_CONFIG="/data/data/com.termux/files/usr/bin/mysql_config"

export BRAXON_MARIADB_LIBDIR="/data/data/com.termux/files/usr/lib/aarch64-linux-android/"
export BRAXON_MARIADB_FOUND_DIR="/data/data/com.termux/files/usr/lib/aarch64-linux-android"

export CMAKE_PREFIX_PATH="/data/data/com.termux/files/usr:/data/data/com.termux/files/usr/lib/cmake:/data/data/com.termux/files/usr/share/cmake${CMAKE_PREFIX_PATH:+:$CMAKE_PREFIX_PATH}"
export PKG_CONFIG_PATH="/data/data/com.termux/files/usr/lib/pkgconfig:/data/data/com.termux/files/usr/share/pkgconfig:/data/data/com.termux/files/usr/lib/aarch64-linux-android/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"

export CPATH="/data/data/com.termux/files/usr/include:/data/data/com.termux/files/usr/include/mariadb:/data/data/com.termux/files/usr/include/mariadb/mysql${CPATH:+:$CPATH}"
export LIBRARY_PATH="/data/data/com.termux/files/usr/lib/aarch64-linux-android/:/data/data/com.termux/files/usr/lib/aarch64-linux-android:/data/data/com.termux/files/usr/lib:/data/data/com.termux/files/usr/lib/aarch64-linux-android${LIBRARY_PATH:+:$LIBRARY_PATH}"
export LD_LIBRARY_PATH="/data/data/com.termux/files/usr/lib/aarch64-linux-android/:/data/data/com.termux/files/usr/lib/aarch64-linux-android:/data/data/com.termux/files/usr/lib:/data/data/com.termux/files/usr/lib/aarch64-linux-android${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

export CFLAGS="-I/data/data/com.termux/files/usr/include -I/data/data/com.termux/files/usr/include/mariadb -I/data/data/com.termux/files/usr/include/mariadb/mysql ${CFLAGS:-}"
export CPPFLAGS="-I/data/data/com.termux/files/usr/include -I/data/data/com.termux/files/usr/include/mariadb -I/data/data/com.termux/files/usr/include/mariadb/mysql ${CPPFLAGS:-}"
export LDFLAGS="-L/data/data/com.termux/files/usr/lib/aarch64-linux-android/ -L/data/data/com.termux/files/usr/lib/aarch64-linux-android -L/data/data/com.termux/files/usr/lib -L/data/data/com.termux/files/usr/lib/aarch64-linux-android -Wl,-rpath=/data/data/com.termux/files/usr/lib/aarch64-linux-android/ -Wl,-rpath=/data/data/com.termux/files/usr/lib/aarch64-linux-android ${LDFLAGS:-}"

echo "Native CMake/MariaDB env active"
echo "  CC=$CC"
echo "  CMAKE=$CMAKE"
echo "  MYSQL_CONFIG=$MYSQL_CONFIG"
echo "  LD_LIBRARY_PATH=$LD_LIBRARY_PATH"
