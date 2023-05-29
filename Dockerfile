# Build stage
FROM rust:latest AS build
WORKDIR /app
COPY . .
RUN cargo build --release

# Final stage
FROM mariadb:latest
ENV MARIADB_ROOT_PASSWORD=secret
EXPOSE 8080 3306
COPY --from=build /app/target/release/dbwebconnect /app/dbwebconnect
COPY /tmp /app/tmp
ENTRYPOINT [ "/app/dbwebconnect", "0.0.0.0" ]
