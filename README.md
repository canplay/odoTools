# Odo Server Toolbox ( WIP )

> - ## Website
>    - news
>    - pearl shop
>    - center market
>    - register (done)
>    - login
>    - download
>    - community

> - ## Client Launcher
>    - xc path
>    - bsd update
>    - support version 2100
>    - support version 2260
>    - support version 2508

> - ## Server Manager
>    - config
>    - event
>    - client patch generate
>    - etc

> - ## Gm Tool
>    - broadcast
>    - quest
>    - items
>    - equip
>    - outfit
>    - level
>    - exp
>    - etc

# Build
> - ## frontend
>    - Requirement:
>        - Nodejs (https://nodejs.org/)
>        - Yarn (https://classic.yarnpkg.com/lang/en/)
>        - Quasar (https://quasar.dev/)
>    - Debug commond:
>        - quasar dev
>    - Release commond:
>        - quasar build
>    - Output dir
>        - \<frontend dir\>/dist/spa
> - ## launcher & backend
>    - Requirement:
>        - Rust (https://www.rust-lang.org/)
>    - Debug commond:
>        - cargo run
>    - Release commond:
>        - cargo build --release
>    - Output dir
>        - \<your launcher dir or your backend dir\>/target/release

# Useage
> - ## frontend
>    - Put the files in the \<your frontend dir\>/dist/spa directory into the \<your odo server dir\>/web
> - ## backend
>    - Place backend.exe & config.json & log4rs.yaml in the \<your odo server dir\>
> - ## launcher
>    - Place launcher.exe in the \<your odo client dir\>
> - Firewall permits port 12347, modify the database port in \<your odo server dir\>/config.json
> - Start backend.exe and wait for the display ' ===== all success ===== '
> - Open website http://localhost:12347 to register an account
> - Login to the game using launcher.exe
