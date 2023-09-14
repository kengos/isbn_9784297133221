FROM rust:1-slim-bookworm

RUN set -eux; \
  apt-get update; \
  apt-get install -y --no-install-recommends \
  curl \
  libssl-dev \
  pkg-config \
  locales \
  ; \
  rm -rf /var/lib/apt/lists/* ; \
  locale-gen ja_JP.UTF-8 ; \
  localedef -f UTF-8 -i ja_JP ja_JP.UTF-8 ;

ENV LANG=ja_JP.UTF-8 \
  LC_ALL=ja_JP.UTF-8 \
  TZ=Asia/Tokyo
RUN rustup component add rustfmt rust-src;
WORKDIR /app
EXPOSE 8888
