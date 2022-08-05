```
proxychains4 bash <(curl -sSf https://sugar.metaplex.com/install.sh)

cargo install sugar-cli --locked

git clone https://github.com/metaplex-foundation/sugar.git
cd sugar
cargo install --locked --path ./

cd sugar

cp -rf ../../simple_nft2/metaplex/assets ./

sugar launch

proxychains4 sugar mint -n 9

```
