# Load the environment variables from the .env file
set -a && source ./.env && set +a

# Create the database and run the migrations
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_USER}
sqlx database create
sqlx migrate run
