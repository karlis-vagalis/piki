env "local" {
    
    url = getenv("DATABASE_URL")

    dev = "docker://postgres/18beta1/dev"
    src = "file://db/schema/schema.sql"

    migration {
        dir = "file://db/migrations"
    }
}