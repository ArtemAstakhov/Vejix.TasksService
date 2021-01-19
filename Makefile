export DATABASE_URL=postgresql://postgres:postgres@localhost:5432/kaizen_dev
export TOKEN_SECRET=secret

watch:
	cargo watch -x "run"

diesel-setup:
	diesel migration run
