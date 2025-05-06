[working-directory: 'packages/server']
dev:
    cargo run

[working-directory: 'packages']
migrate-up:
    sea-orm-cli migrate up

[working-directory: 'packages']
migrate-down:
    sea-orm-cli migrate down

[working-directory: 'packages']
migrate-fresh:
    sea-orm-cli migrate fresh

[working-directory: 'packages/entity']
generate-entity:
    sea-orm-cli generate entity --lib --output-dir src
    
