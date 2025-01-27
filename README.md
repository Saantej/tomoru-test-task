
# Axum Server

Это минималистичный веб-сервер на базе [Axum](https://docs.rs/axum), который предоставляет простой API. Обновления информации о ip происходит паралельно кажду секунду.

## Запуск через Docker

1. **Склонируйте репозиторий:**
   ```bash
   git clone git@github.com:Saantej/tomoru-test-task.git
   cd tomoru-test-task
   ```

2. **Запустить проект Docker:**
   ```bash
   docker compose up --build
   ```


4. **Проверьте работу сервера:**
   Выполните запрос на эндпоинт `/ping` или откройте в браузере `http://localhost:3000/ping` :
   ```bash
   curl http://localhost:3000/ping
   ```


5. **Проверьте выывод в терминале:**
   ```bash
   axum_server-1  | IPS: 
   axum_server-1  | [("192.168.107.1", 4)]
   axum_server-1  | IPS: 
   axum_server-1  | [("192.168.107.1", 4)]
   axum_server-1  | IPS: 
   axum_server-1  | [("192.168.107.1", 4)]
   axum_server-1  | IPS: 
   axum_server-1  | [("192.168.107.1", 4)]
   ```

---
