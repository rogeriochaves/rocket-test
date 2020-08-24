# The product development platform

## How to run locally

Copy the .env.sample to .env

```
docker-compose up --build
```

## How to run locally manually

Copy the .env.sample to .env and source its variables (dotenv)

Database:

```
docker-compose up tpdp_postgres
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

## How to deploy

```
eval $(docker-machine env social-scaleway)
docker-compose -f docker-compose.prod.yml up --build
```
