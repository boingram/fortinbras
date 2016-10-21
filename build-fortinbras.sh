#/bin/bash

cd fortinbras-ui 
npm run build
cd ..
mkdir -p target/ui/
cp -r fortinbras-ui/dist/ target/ui/
cargo run
