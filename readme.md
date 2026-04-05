Install it once:


cargo install sqlx-cli --no-default-features --features postgres
Run migrations manually:


sqlx migrate run --database-url "postgresql://username:password@host/schema"
Or if DATABASE_URL is set in your .env:


sqlx migrate run
Other useful commands:


# check which migrations have been applied
sqlx migrate info

# revert the last migration
sqlx migrate revert

# add a new migration file (auto-generates the filename)
sqlx migrate add create_projects
# → creates migrations/0002_create_projects.sql
The sqlx migrate add command is especially handy — it generates the next numbered file for you so you don't have to track the sequence manually.