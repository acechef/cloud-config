# cloud-config

创建链的配置文件。

### 依赖

* rust: 1.54.0

### 安装

```
cargo install --path .
```

### 用法

```
$ cloud-config help
cloud-config 6.3.0

Yieazy

USAGE:
    cloud-config <SUBCOMMAND>

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    append    append config
    create    create config
    delete    delete config
    help      Print this message or the help of the given subcommand(s)
```

### 初始化链

```
$ config-config create -h
cloud-config-create 

create config

USAGE:
    cloud-config create [OPTIONS] --consensus <CONSENSUS>

FLAGS:
    -h, --help    Print help information
        --use_num    use serial number instead of node address

OPTIONS:
        --chain-name <CHAIN_NAME>
            set chain name [default: test-chain]

        --config-dir <CONFIG_DIR>
            set config file directory, default means current directory

        --config-name <CONFIG_NAME>
            set config file name [default: config.toml]

        --consensus <CONSENSUS>
            Set consensus micro-service

        --controller <CONTROLLER>
            Set controller micro-service [default: controller]

        --executor <EXECUTOR>
            Set executor micro-service [default: executor_evm]

        --grpc-ports <GRPC_PORTS>
            grpc port list, input "p1,p2,p3,p4", use default grpc port count from 50000 + 1000 * i
            use default must set peer_count or p2p_ports [default: default]

        --kms <KMS>
            Set kms micro-service [default: kms_sm]

        --kms-password <KMS_PASSWORD>
            kms db password [default: 123456]

        --network <NETWORK>
            Set network micro-service [default: network_p2p]

        --p2p-ports <P2P_PORTS>
            p2p port list, input "ip1:port1,ip2:port2,ip3:port3,ip4:port4", use default port count
            from 127.0.0.1:40000 + 1 * i, use default must set peer_count or grpc_ports [default:
            default]

        --package-limit <PACKAGE_LIMIT>
            set one block contains tx limit, default 30000 [default: 30000]

        --peers-count <PEERS_COUNT>
            set initial node number, default "none" mean not use this must set grpc_ports or
            p2p_ports, if set peers_count, grpc_ports and p2p_ports, base on grpc_ports > p2p_ports
            > peers_count

        --storage <STORAGE>
            Set storage micro-service [default: storage_rocksdb]

        --version <VERSION>
            set version [default: 0]
```

#### 例子

```
$ cloud-config create --peers-count 4 --consensus consensus_raft --config-dir /tmp/test
```

#### 生成的文件

```
$ cd /tmp/test 
$ ls
test-chain                                           test-chain-57b98686b6636877a04710dc57127526feac76e7  test-chain-b4d2011d32ff5484b18dcb237e0dbf504b11c97e
test-chain-3e3acf2feb25ac611db9348244de132d01327dab  test-chain-94cc5493111435bcfb0a03eb39921ad0f2e379f8

$ tree .
.
├── test-chain
│   ├── config.toml
│   └── kms.db
├── test-chain-3e3acf2feb25ac611db9348244de132d01327dab
│   ├── config.toml
│   ├── consensus-log4rs.yaml
│   ├── controller-log4rs.yaml
│   ├── executor-log4rs.yaml
│   ├── kms.db
│   ├── kms-log4rs.yaml
│   ├── network-log4rs.yaml
│   └── storage-log4rs.yaml
├── test-chain-57b98686b6636877a04710dc57127526feac76e7
│   ├── config.toml
│   ├── consensus-log4rs.yaml
│   ├── controller-log4rs.yaml
│   ├── executor-log4rs.yaml
│   ├── kms.db
│   ├── kms-log4rs.yaml
│   ├── network-log4rs.yaml
│   └── storage-log4rs.yaml
├── test-chain-94cc5493111435bcfb0a03eb39921ad0f2e379f8
│   ├── config.toml
│   ├── consensus-log4rs.yaml
│   ├── controller-log4rs.yaml
│   ├── executor-log4rs.yaml
│   ├── kms.db
│   ├── kms-log4rs.yaml
│   ├── network-log4rs.yaml
│   └── storage-log4rs.yaml
└── test-chain-b4d2011d32ff5484b18dcb237e0dbf504b11c97e
    ├── config.toml
    ├── consensus-log4rs.yaml
    ├── controller-log4rs.yaml
    ├── executor-log4rs.yaml
    ├── kms.db
    ├── kms-log4rs.yaml
    ├── network-log4rs.yaml
    └── storage-log4rs.yaml

5 directories, 34 files
```

`test-chain-b4d2011d32ff5484b18dcb237e0dbf504b11c97e`：节点名称的构造为 `<chain-name>-<node-address>`

### 增加节点

```
$ cloud-config append -h
cloud-config-append 

append config

USAGE:
    cloud-config append [OPTIONS]

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
        --chain-name <CHAIN_NAME>
            set chain name [default: test-chain]

        --config-dir <CONFIG_DIR>
            set config file directory, default means current directory

        --config-name <CONFIG_NAME>
            set config file name [default: config.toml]

        --grpc-ports <GRPC_PORTS>
            grpc port list, input "p1,p2,p3,p4", use default grpc port count from 50000 + 1000 * i
            use default must set peer_count or p2p_ports [default: default]

        --kms-password <KMS_PASSWORD>
            kms db password [default: 123456]

        --network <NETWORK>
            Set network micro-service [default: network_p2p]

        --p2p-ports <P2P_PORTS>
            p2p port list, input "ip1:port1,ip2:port2,ip3:port3,ip4:port4", use default port count
            from 127.0.0.1:40000 + 1 * i, use default must set peer_count or grpc_ports [default:
            default]

        --package-limit <PACKAGE_LIMIT>
            set one block contains tx limit, default 30000 [default: 30000]

        --peers-count <PEERS_COUNT>
            set initial node number, default "none" mean not use this must set grpc_ports or
            p2p_ports, if set peers_count, grpc_ports and p2p_ports, base on grpc_ports > p2p_ports
            > peers_count
```

##### 例子

```
$ cloud-config append --peers-count 2 --config-dir /tmp/test

$ ls /tmp/test
test-chain-3e3acf2feb25ac611db9348244de132d01327dab  test-chain-57b98686b6636877a04710dc57127526feac76e7  test-chain-b4d2011d32ff5484b18dcb237e0dbf504b11c97e
test-chain  test-chain-4f9cec049857a119b51472dac520f4bde0bca4d0  test-chain-94cc5493111435bcfb0a03eb39921ad0f2e379f8  test-chain-e51ab390ec569bb5e9b0c96a84df40b9f6923af2
```

增加了2个新节点 `test-chain-4f9cec049857a119b51472dac520f4bde0bca4d0`，`test-chain-e51ab390ec569bb5e9b0c96a84df40b9f6923af2`

### 减少节点

```
$ cloud-config delete -h
cloud-config-delete 

delete config

USAGE:
    cloud-config delete [OPTIONS]

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
        --addresses <ADDRESSES>        delete node address. Such as
                                       0x16e80b488f6e423b9faff014d1883493c5043d29,0x5bc21f512f877f18840abe13de5816c1226c4710
                                       will node with the address [default: default]
        --chain-name <CHAIN_NAME>      set chain name [default: test-chain]
        --config-dir <CONFIG_DIR>      set config file directory, default means current directory
        --config-name <CONFIG_NAME>    set config file name [default: config.toml]
        --index <INDEX>                delete index. Such as 1,2 will delete first and second node
                                       [default: default]
```

#### 例子

```
$ cloud-config delete --addresses 0x4f9cec049857a119b51472dac520f4bde0bca4d0,0xe51ab390ec569bb5e9b0c96a84df40b9f6923af2 --config-dir /tmp/test

$ ls /tmp/test
test-chain                                           test-chain-57b98686b6636877a04710dc57127526feac76e7  test-chain-b4d2011d32ff5484b18dcb237e0dbf504b11c97e
test-chain-3e3acf2feb25ac611db9348244de132d01327dab  test-chain-94cc5493111435bcfb0a03eb39921ad0f2e379f8
```

节点`test-chain-4f9cec049857a119b51472dac520f4bde0bca4d0`，`test-chain-e51ab390ec569bb5e9b0c96a84df40b9f6923af2`被删除

### 
