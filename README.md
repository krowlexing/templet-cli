# templet-cli

`cargo build`

`cargo run event -s "string content" --tag 1 -S sql.db -n name`

## Примеры

`templet-cli events -o 1 --host localhost:3000 -T token`

`templet-cli event -s "string content" --tag 1 --host localhost:3000 -T token`

`templet-cli query filename --tag 2 -h localhost:3000 -T token`

`templet-cli reply -s "reply string" -o 1 -S sqlite_file.db --name Name`

## Описание

Совершает запросы к серверу, если у запроса есть тело, то оно указывается в одном из следующих форматов:

- имя файла, содержимое которого отправляется как тело запроса
- `-s ""` строковой параметр, задающий тело запроса напрямую

В зависимости от указанных параметров запрос:

- локальную базу данных sqlite. Требуются `-S` имя файла бд и `--name` имя.
- отправляется по http к указанному серверу. Требуются `--host` хост и `-T` токен. 
    
    Токен можно получить при запросе /token?name=Name к серверу.