# elem
stupid element game

(thank you carykh for elemental 3 and the idea)

## how to build

```sh
git clone https://github.com/x-osc/elem
cd elem
```
edit `compiler/elem.elem` file to add elements or whatever

```sh
cd compiler/
cargo run -- elem.elem
```
copy the generated categories, combinations, elements .json to `web/static/data`

```sh
cd web/
npm install
npm run dev
```
