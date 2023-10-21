# Get rust image 
FROM rustlang/rust:nightly-bullseye as builder

# Install cargo-binstall
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Download tailwind
RUN WGET https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
RUN cp tailwindcss-linux-x64 /usr/local/cargo/bin/tailwind

# Install binaries with cargo binstall
RUN cargo binstall trunk -y
RUN cargo binstall sqlx -y

# Build backend
RUN cd ./backend & \
  cargo build --release &\
  sqlx migrate run --database-url $DATABASE_URL &\
  cd ..

# Build frontend with trunk and tailwind
RUN cd ./frontend & \
  tailwind -i input.css -o style/tailwind.css &\
  trunk build --release &\
  cd ..

# - Copy backend and frontend to container
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Get alpine image
# Set environment variables
# Run container
FROM rustlang/rust:nightly-bullseye as runner