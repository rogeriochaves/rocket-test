# The product development platform

## How to run locally

Copy the .env.sample to .env

```
docker-compose up --build --renew-anon-volumes
```

## How to run locally manually

It's more work but use less resources because of running outside docker

Copy the .env.sample to .env and source its variables (dotenv)

Database:

```
docker-compose up tpdp_postgres
```

Core:

```
cargo install cargo-watch
cd core
cargo build
export DATABASE_URL=postgres://postgres:password@localhost/tpdp
diesel setup
diesel migration run
cargo watch -w src -x run
```

Site:

```
cd site
npm install
npm start
```

After first setup next time you can simply run

```
./start-dev.sh
```

## How to deploy

```
eval $(docker-machine env social-scaleway)
docker-compose -f docker-compose.prod.yml up -d --build
```
