# rust_fullstack_example

An example using Rocket, Diesel, Tera and MySQL to create a simple chat app.

## Instructions
1. Clone the repo
2. Set rust to nightly:
```bash
rustup override set nightly
```
3. Change the link to the data base in `Rocket.toml` and in `.env`
4. Additionaly, if you want to run in release, change the address and port in `Rocket.toml`. 

## database setup
1. install docker
2. install mysql image :  https://dilsichandrasena.medium.com/how-to-deploy-and-use-a-mysql-docker-container-in-ubuntu-4ace7c893982

3. run these commands in mysql

```
    CREATE USER 'admin'@'172.17.0.1' IDENTIFIED BY 'admin';
    GRANT ALL PRIVILEGES ON *.* TO 'admin'@'172.17.0.1' WITH GRANT OPTION;

```