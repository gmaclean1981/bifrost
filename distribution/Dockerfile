FROM ghcr.io/duvholt/bifrost@sha256:d074d883f61f94f32d494709aa34426f91c325f66dfc308c735ca2181cdbbc8c
RUN apt-get update && apt-get -y upgrade && rm -rf /var/lib/apt/lists/*
ADD https://github.com/gmaclean1981/bifrost/releases/download/dev-2026-10-28-maint.1/bifrost-dev-2026-10-28-maint.1-linux-amd64.tar.gz /tmp/bifrost-release.tar.gz
RUN printf '%s  %s\n' '5503aa77589382280e28be968a03a1d44a5363c39fe41aace0c880b296c7b1ce' '/tmp/bifrost-release.tar.gz' | sha256sum -c - \
    && mkdir -p /tmp/bifrost-release /usr/share/licenses/bifrost \
    && tar -xzf /tmp/bifrost-release.tar.gz -C /tmp/bifrost-release \
    && install -m 755 /tmp/bifrost-release/bifrost /app/bifrost \
    && cp -a /tmp/bifrost-release/licenses/. /usr/share/licenses/bifrost/ \
    && rm -rf /tmp/bifrost-release /tmp/bifrost-release.tar.gz
LABEL org.opencontainers.image.version="dev-2026-10-28-maint.1" \
      org.opencontainers.image.source="https://github.com/gmaclean1981/bifrost" \
      org.opencontainers.image.licenses="GPL-3.0-only"
WORKDIR /config/bifrost
CMD ["/app/bifrost"]
