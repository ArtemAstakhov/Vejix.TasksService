export DATABASE_URL=postgresql://postgres:postgres@localhost:5432/local
export TOKEN_SECRET=secret

watch:
	diesel migration run && cargo watch -x "run"
