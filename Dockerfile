# Étape de construction
FROM rust:latest as builder

# Installer les dépendances système requises par Bevy
RUN apt-get update && apt-get install -y \
    libasound2-dev \
    libudev-dev \
    libx11-dev \
    libxcb-shm0-dev \
    libxcb-render0-dev \
    libxcb-xfixes0-dev \
    libxcb-randr0-dev \
    libxcb-composite0-dev \
    libxi-dev \
    libgl1-mesa-dev \
    libvulkan1 \
    vulkan-tools \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copier les fichiers de dépendances
COPY Cargo.toml Cargo.lock ./

# Créer un projet factice pour mettre en cache les dépendances
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "fn main() {}" > src/bin/client.rs && \
    cargo build --release && \
    rm -rf src

# Copier les sources réelles
COPY . .

# Construire le projet
RUN cargo build --release --bin server --bin client

# Étape d'exécution
FROM debian:bullseye-slim

# Installer les dépendances runtime
RUN apt-get update && apt-get install -y \
    libasound2 \
    libudev1 \
    libxcb1 \
    libxcb-render0 \
    libxcb-shm0 \
    libxcb-xfixes0 \
    libxcb-randr0 \
    libxcb-composite0 \
    libxi6 \
    libgl1-mesa-glx \
    libvulkan1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copier les binaires depuis le builder
COPY --from=builder /app/target/release/server /app/server
COPY --from=builder /app/target/release/client /app/client

# Exposer les ports (ajuster selon votre configuration)
EXPOSE 3536/udp 3537/udp

# Commande par défaut (modifier selon le besoin)
ENTRYPOINT ["/app/server"]