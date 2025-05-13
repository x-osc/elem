cd ./compiler/
cargo run -- elem.elem
cd ../
cp ./compiler/categories.json ./web/static/data/categories.json
cp ./compiler/elements.json ./web/static/data/elements.json
cp ./compiler/combinations.json ./web/static/data/combinations.json
echo copied files to ./web/static/data/
