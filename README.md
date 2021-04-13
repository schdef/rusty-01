# vaulty

## docker

### build

- cargo build
- cargo run

### run

- build: docker build . -t vaulty
- run: docker run -p 8000:8000 -d vaulty-0

## use

### install and setup diesel

cargo install diesel_cli --no-default-features --features postgres

### run

- docker run --name some-postgres -p 5432:5432 -e POSTGRES_PASSWORD=mysecretpassword -d postgres
- diesel migration run
- echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env
- cargo run

- curl --location --request GET 'localhost:8000/my-context/context-key-id' \
  --header 'Content-Type: application/json'

# TOOD

- POST errorhandling VS. PUT overrride
- GET 404 handling
