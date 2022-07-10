$DB_USER = "postgres"
$DB_PASSWORD = "password"
$DB_NAME = "newsletter"
$DB_PORT = 5432

$PortMap = "${DB_PORT}:5432"

$Cwd = Get-Item . | Select Name

if ($Cwd -eq "scripts") {
    Set-Location ../
}

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

Start-Sleep 5

Write-Host -ForegroundColor Green "Postgres is up and running on port ${DB_PORT} - running migrations..."

$Env:DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}"

$Output = (sqlx database create) | Out-String
Write-Host -ForegroundColor Red $Output

$Output = (sqlx migrate run) | Out-String
Write-Host -ForegroundColor Red $Output

Write-Host -ForegroundColor Green "Postgres has been migrated, ready to go!"