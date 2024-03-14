# banksim

This is a bank simulator.
There is also [acqui](https://github.com/ghashy/acqui), written in Swift for macOS, which serves as an `banksim` management client.

> [!IMPORTANT]
> Currently, `banksim` supports a single-store account.

`banksim` was designed to be simple. It can create/delete accounts, open credits, create transactions, track balances, bank emission. With a simple internal design, it aims to offer real-life API interaction, just like in real acquiring services.

There are two storage backends supported:
- In-memory storage
- Postgres

> The primary purpose of `banksim` is for mocking and running backends that need to process payments in a test environment.

## Usage:

You can either build the Docker container yourself in this directory or use a pre-built image from Docker Hub:
```bash
docker pull ghashy/banksim
```

You need to pass a configuration file and a secret files as secrets. For example, using docker-compose:
```yaml
services:
  banksim:
    image: ghashy/banksim:0.1
    expose:
      - "15100"
    secrets:
      - terminal-password
      - example-config
    environment:
      TERMINAL_PASSWORD_FILE: /run/secrets/terminal-password
      BANKSIM_CONFIG_FILE: /run/secrets/example-config
      POSTGRES_PASSWORD_FILE: /run/secrets/postgres-password 
      DATA_BACKEND_TYPE: postgres
secrets:
  example-config:
    file: secrets/example_config.yaml
  terminal-password:
    file: secrets/terminal_password.txt
  postgres-password:
    file: secrets/postgres_password.txt
```

Configuration example:
```yaml
port: 15100
addr: localhost
bank_username: bank_user
terminal_settings:
  terminal_key: 3C43FD0A-50E5-435F-8969-D83BC07C4912
  success_url: "http://mydomain.com/success_path"
  fail_url: "http://mydomain.com/fail_path"
  success_add_card_url: "http://mydomain.com/add_card_success_path"
  fail_add_card_url: "http://mydomain.com/add_card_fail_path"
  notification_url: "http://mydomain.com/notification_path"
  send_notification_finish_authorize: true
  send_notification_completed: true
  send_notification_reversed: true
database_settings: # Optional
  username: postgres
  database_name: banksim
  host: banksim-pg-host
```

After running, use [acqui](https://github.com/ghashy/acqui) for bank management and [banksim-api](https://github.com/ghashy/airactions/tree/main/backends/banksim-api) for store-bank interaction.
