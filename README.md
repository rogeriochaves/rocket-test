# The product development platform

## How to run locally

Database:

```
docker-compose up
```

Core:

```
cd core
cargo build
export DATABASE_URL=postgres://postgres:password@localhost/tpdp
diesel setup
diesel migration run
cargo run
```

Site:

```
cd site
npm install
npm start
```
