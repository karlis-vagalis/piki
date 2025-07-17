env "local" {
    
    url = getenv("DATABASE_URL")

    dev = "docker://postgres/17/dev"
    src = "file://db/schema/schema.sql"

    migration {
        dir = "file://db/migrations"
    }
}