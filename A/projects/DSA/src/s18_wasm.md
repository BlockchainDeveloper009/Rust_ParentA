1. install wasm-pack
2. cargo install cargo-generate
3. cargo generate --git https://github.com/rustwasm/wasm-pack-template
4. change to project directory(wasm-game-of-life> npm init wasm-app www) & run.. wasm-pack build
5. wasm-game-of-life> npm init wasm-app www (should create a folder 'www' )
6. cd into 'www' folder and run: npm install
7. open package.config and add dependency which is 'pkg' folder
   ```
     "homepage": "https://github.com/rustwasm/create-wasm-app#readme",
  "dependencies": {
    "wasm-game-of-life": "file:../pkg"
  },
  ```
8. now, start the project by running below command from path 'www>'
npm run start

```
www> $env:NODE_OPTIONS="--openssl-legacy-provider"
PS C:\source\repos\Rust_ParentA\A\projects\wasm-game-of-life\www> npm run start

> create-wasm-app@0.1.0 start
> webpack-dev-server
```
every time a change is made in rust code,

then do
 from rust project
$cargo build
$wasm-pack build --target web


then go into www
f-life\www> npm install
f-life\www> npm run start
