# Sync canister data to offchain storage

编译之前先把源码的私钥地址设置好

## 1. Token canister

DFCT, WICPT的历史交易数据

轮询目前token canister的总交易数量，如果有新的交易，根据index将新的交易记录同步下来存到sqlite



## 2. Token registry

用户发行代币的信息

轮询目前代币总数量， 如果有新的代币发出来，将其信息同步到本地sqlite



## 3. DSwap

dswap所有的操作记录

轮询目前的操作数量，如果有新的操作，将其信息同步到本地sqlite



用Agent-rs库，实现中可以尽量更通用化，基本的模式就是：通过一个接口获取操作数，与本地已经同步的数据对比，如果有新的数据，就逐一将新的数据同步下来。最理想的是做成可配置的，比如我在配置文件里面指明要同步哪个canister，用哪个candid接口获取最新操作数量，用哪个candid接口获取指定的操作记录，配置完成之后直接启动即可开始同步数据。（目前各个canister接口不统一，暂时难以做到这么通用，但是同类型的canister做到通用化是可以的，比如针对token canister的同步，可以做到可配置，因为token canister的接口都是一样的）

## 第一次启动
```sh
cargo build --release

./target/release/tt
```

## 后续启动
需要根据之前允许的结果，比如
```sh
···
from 142_507 to 142_524
12
from 142_525 to 142_536
18
from 142_537 to 142_554
```
那么启动命令为：
```sh
cargo build --release

./target/release/tt 142555
59
from 142_555 to 142_613
14
from 142_614 to 142_627
18
from 142_628 to 142_645
```