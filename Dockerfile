# Build-Stage mit vollständigem Rust-Image
FROM rust AS builder

# Setze das Arbeitsverzeichnis
WORKDIR /usr/src/app

# Kopiere Cargo.toml und Cargo.lock für das Caching der Abhängigkeiten
COPY Cargo.toml Cargo.lock ./

# Temporäre Cache-Lösung: führe Dummy-Build durch, um Abhängigkeiten zu cachen
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Kopiere den gesamten Code in das Image
COPY . .

# Baue das Release-Binary
RUN cargo build --release

# Runtime-Stage mit einem neueren schlanken Ubuntu-Image
FROM ubuntu

# Installiere benötigte Bibliotheken für das Projekt
RUN apt-get update && apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Setze das Arbeitsverzeichnis für das Runtime-Image
WORKDIR /usr/local/bin

# Kopiere das Release-Binary vom Builder
COPY --from=builder /usr/src/app/target/release/monitor .

# Definiere den Startbefehl
CMD ["./monitor"]
