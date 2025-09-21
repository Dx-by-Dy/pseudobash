```bash
echo "PSEUDOBASH_PATH=$PWD/utils/release" > .env
```

```bash
cd utils && find . -name "Cargo.toml" -exec dirname {} \; | xargs -I {} sh -c 'cd {} && cargo build -r --target-dir ../'; cd ../
```

```bash
cargo build -r --target-dir .
```
