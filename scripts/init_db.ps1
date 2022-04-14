$DB_USER = "postgres"
$DB_PASSWORD = "password"
$DB_NAME = "newsletter"
$DB_PORT = 5432

$PortMap = "${DB_PORT}:5432"

if (-not (Get-Command sqlx -ErrorAction SilentlyContinue)) {
    Write-Host -ForegroundColor Red "Error: sqlx is not installed."
    Write-Host -ForegroundColor Red "Use:"
    Write-Host -ForegroundColor Red "`tcargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
    Write-Host -ForegroundColor Red "to install it."
    Exit 1
}

if (-not (Get-Command docker -ErrorAction SilentlyContinue)) {
    Write-Host -ForegroundColor Red "Error: docker is not installed."
    Write-Host -ForegroundColor Red "Visit:"
    Write-Host -ForegroundColor Red "`thttps://www.docker.com/get-started/" 
    Write-Host -ForegroundColor Red "for installation instructions."
    Exit 1
}

docker run -e POSTGRES_USER=${DB_USER} -e POSTGRES_PASSWORD=${DB_PASSWORD} -e POSTGRES_DB=${DB_NAME} -p $PortMap -d postgres

while (-not (Test-NetConnection -ComputerName localhost -Port 5432 -InformationLevel Quiet)) {
    Write-Host -ForegroundColor Yellow "Postgres is still unavailable..."   
    Start-Sleep 1
}


Write-Host -ForegroundColor Green "Postgres is up and running on port ${DB_PORT} - running migrations..."   

sqlx database create
sqlx migrate run

Write-Host -ForegroundColor Green "Postgres has been migrated, ready to go!"