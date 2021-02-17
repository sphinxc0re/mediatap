FROM rustembedded/cross:armv5te-unknown-linux-gnueabi-0.2.1

RUN apt-get update && \
  apt-get install --assume-yes wget unzip

RUN wget -O sqlite3-src.zip https://sqlite.org/2021/sqlite-amalgamation-3340100.zip && \
  unzip sqlite3-src.zip

WORKDIR /sqlite-amalgamation-3340100

RUN $CC_armv5te_unknown_linux_gnueabi -o libsqlite3.so -shared sqlite3.c -lpthread -ldl && \
  mv libsqlite3.so /usr/lib/libsqlite3.so
