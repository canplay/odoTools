# Odo Tools

> - ## Website
>   - news √
>   - pearl shop
>   - center market
>   - register √
>   - login √
>   - download √
>   - community
>   - recharge
>   - server config
>   - client diff patch generate
>   - game event
>   - game broadcast
>   - game quest
>   - game items
>   - game equip
>   - game outfit
>   - game level
>   - game exp
>   - etc

> - ## Client Launcher
>   - bypass xc
>   - diff patch
>   - support version 2100
>   - support version 2260 √
>   - support version 2508

# Build

> - ## frontend
>   - Requirement:
>     - Nodejs (https://nodejs.org/)
>     - Yarn (https://classic.yarnpkg.com/lang/en/)
>     - Quasar (https://quasar.dev/)
>   - Debug commond:
>     - quasar dev
>   - Release commond:
>     - quasar build
>   - Output dir
>     - \<frontend dir\>/dist/spa
> - ## launcher & backend
>   - Requirement:
>     - Rust (https://www.rust-lang.org/)
>   - Debug commond:
>     - cargo run
>   - Release commond:
>     - cargo build --release
>   - Output dir
>     - \<your launcher dir or your backend dir\>/target/release

# Useage

> - ## frontend
>   - Put the files in the \<your frontend dir\>/dist/spa directory into the \<your odo server dir\>/web
> - ## backend
>   - Place backend.exe & config.json in the \<your odo server dir\>
> - ## launcher
>   - Place launcher.exe & service.ini in the \<your odo client dir\>
> - Firewall permits port 12347, modify the database port in \<your odo server dir\>/config.json

# NOTE

> - Firewall permits port 12347
> - modify the database port in \<your odo server dir\>/config.json
> - modify the game server address in \<your odo server dir\>/web/config.json
> - modify the game server ip in \<your odo client dir\>/service.ini
