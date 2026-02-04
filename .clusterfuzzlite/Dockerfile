FROM gcr.io/oss-fuzz-base/base-builder-rust
RUN apt-get update && apt-get install -y make autoconf automake libtool
COPY . $SRC/language-interop-compiler
WORKDIR $SRC/language-interop-compiler
COPY .clusterfuzzlite/build.sh $SRC/
