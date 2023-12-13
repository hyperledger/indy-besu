# WASM FFI for Indy Besu VDR

### How to build with wasm-pack
Install wasm-pack from https://rustwasm.github.io/wasm-pack/installer/ and then

#### Build NodeJS bindings
```
wasm-pack build --target=nodejs
```

#### Build WASM (consumed in browser without bundler usage) bindings
```
wasm-pack build --target=web
```

#### Build bindings to use with webpack
```
wasm-pack build
```

### Run NodeJS demo
```
wasm-pack build --target=nodejs
cd demo/node
yarn install
yarn start
```