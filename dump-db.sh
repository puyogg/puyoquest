NOW=$(date +%s)
docker compose exec discordbot-db pg_dump -U postgres discordbot-db > backup_${NOW}.sql