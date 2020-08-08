FROM rustlang/rust:nightly as build

ENV USER=root
RUN cargo new --bin project
WORKDIR /project

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release
RUN rm -rf src

COPY migrations ./migrations
COPY src ./src
COPY diesel.toml Rocket.toml ./

ENV DATABASE_URL=postgresql://postgres:postgres@vejix.ce4pcrek9nhy.us-east-1.rds.amazonaws.com:5432/vejix_tasks

RUN cargo install diesel_cli --no-default-features --features postgres
RUN diesel setup
RUN diesel migration run

RUN rm ./target/release/deps/tasks_service*
RUN cargo build --release

FROM ubuntu

ENV TZ=Europe/Kiev
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN apt-get update && apt-get -y install postgresql

WORKDIR /project
COPY diesel.toml Rocket.toml ./
COPY --from=build /project/target/release/tasks_service .

EXPOSE 30081

ENV DATABASE_URL=postgresql://postgres:postgres@vejix.ce4pcrek9nhy.us-east-1.rds.amazonaws.com:5432/vejix_tasks

CMD ["./tasks_service"]