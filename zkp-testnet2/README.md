## ZKP
```bash
# 零知识证明计算测试 CPU模式
cargo run --release --example posw

unbuffer cargo run --release --example posw | tee -a local.log
```