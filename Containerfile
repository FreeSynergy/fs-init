FROM fedora:41

WORKDIR /app

# Copy pre-built binary (built by CI)
COPY target/release/fs-init /usr/local/bin/fs-init

# Runtime user
RUN useradd -r -s /sbin/nologin fsinit

USER fsinit

ENTRYPOINT ["/usr/local/bin/fs-init"]

LABEL org.opencontainers.image.source="https://github.com/FreeSynergy/fs-init"
LABEL org.opencontainers.image.description="FreeSynergy Init — bootstraps a node"
