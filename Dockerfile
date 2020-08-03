FROM rustlang/rust:nightly

WORKDIR /usr/src/app
COPY . .

EXPOSE 30081

ENV DATABASE_URL=postgresql://postgres:postgres@vejix.ce4pcrek9nhy.us-east-1.rds.amazonaws.com:5432/vejix_tasks

RUN cargo install diesel_cli --no-default-features --features postgres
RUN diesel setup
RUN diesel migration run

RUN cargo build --release
RUN cargo install --path .

CMD ["./target/release/tasks_service"]