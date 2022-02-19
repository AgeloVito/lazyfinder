# lazyfinder
## 适合在攻防场景下遍历目标目录中包含指定关键字的文件，并从匹配到的文件中匹配特定字符串所在行
## 比系统原生的find+grep啥的快很多，而且可以精确到行

# usage
```
cargo build --release
./lazyfinder -d ~/  -p toml,yml,config -k password
```

# 效果

<img width="964" alt="image" src="https://user-images.githubusercontent.com/38530231/153534773-b4bbc7b3-8e62-4ab7-949b-b0bb38aac126.png">


