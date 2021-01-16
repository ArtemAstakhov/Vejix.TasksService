FROM rustlang/rust:nightly as build

ENV USER=root
RUN cargo new --bin project
WORKDIR /project

COPY Cargo.toml Cargo.lock ./
COPY migrations ./migrations
COPY src ./src
COPY diesel.toml ./

RUN cargo build --release

FROM ubuntu

ENV TZ=Europe/Kiev
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN apt-get update && apt-get -y install postgresql

WORKDIR /project
COPY diesel.toml ./
COPY migrations ./migrations
COPY --from=build /project/target/release/tasks_service .

CMD ["./tasks_service"]