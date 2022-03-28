# Odo Tools

> - ## Website
>   - news √
>   - pearl shop √
>   - center market
>   - register √
>   - login √
>   - download √
>   - community
>   - recharge
>   - server config √
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
>   - bypass xc √
>   - diff patch
>   - support version 2100 √
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
>     - '\<frontend dir\>/dist/spa'
> - ## launcher
>   - Nodejs (https://nodejs.org/)
>   - Quasar (https://quasar.dev/)
>   - Tauri (https://tauri.studio/)
>   - Debug commond:
>     - quasar dev
>     - yarn tauri dev OR cargo tauri dev IN THE '\<launcher dir\>/src-tauri'
>   - Release commond:
>     - quasar build
>     - yarn tauri build OR cargo tauri build IN THE '\<launcher dir\>/src-tauri'
>   - Output dir
>     - '\<launcher dir\>/src-tauri/target/debug' or '\<launcher dir\>/src-tauri/target/release'
> - ## backend
>   - Requirement:
>     - Rust (https://www.rust-lang.org/)
>   - Debug commond:
>     - cargo run
>   - Release commond:
>     - cargo build --release
>   - Output dir
>     - '\<your launcher dir or your backend dir\>/target/debug' or '\<your launcher dir or your backend dir\>/target/release'

# Useage

> - ## frontend
>   - Put the files in the '\<your frontend dir\>/dist/spa' directory into the '\<your odo server dir\>/web'
> - ## backend
>   - Place backend.exe in the '\<your odo server dir\>', import website.sql into the 'website' mongodb collection
> - ## launcher
>   - Place launcher.exe & Odo Launcher.exe.manifest & service.ini in the '\<your odo client dir\>'

# NOTE

> - Firewall permits port 12347
> - modify the database port in '\<your odo server dir\>/config.json'
> - modify the game server address in 'website' mongodb collection
