
# Axum Server

Это минималистичный веб-сервер на базе [Axum](https://docs.rs/axum), который предоставляет простой API.

## Запуск через Docker

1. **Склонируйте репозиторий:**
   ```bash
   git clone git@github.com:Saantej/tomoru-test-task.git
   cd tomoru-test-task
   ```

2. **Запустить проек Docker:**
   ```bash
   docker compose up --build
   ```


4. **Проверьте работу сервера:**
   Выполните запрос на эндпоинт `/ping` или откройте в браузере `http://localhost:3000/ping` :
   ```bash
   curl http://localhost:3000/ping
   ```

---