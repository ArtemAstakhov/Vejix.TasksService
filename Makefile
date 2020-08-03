export DATABASE_URL=postgresql://postgres:postgres@vejix.ce4pcrek9nhy.us-east-1.rds.amazonaws.com:5432/vejix_tasks

watch:
	diesel migration run && cargo watch -x "run"

setup:
	diesel setup
