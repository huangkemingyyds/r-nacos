# 性能与容量说明

## 压测工具

主要使用goose压测。

参考项目中的子工程 [loadtest](https://github.com/heqingpan/rnacos/tree/master/loadtest)

## 性能压测结果

|模块|场景|单节点qps|集群qps|总结|
|--|--|--|--|--|
|配置中心|配置写入,单机模式|1.5万|1.5万||
|配置中心|配置写入,集群模式|1.8千|1.5千|接入raft后没有充分优化,待优化,理论上可接近单机模式|
|配置中心|配置查询|8万|n*8万|集群的查询总qps是节点的倍数|
|注册中心|服务实例注册,http协议|1.2万|1.0万|注册中心单机模式与集群模式写入的性能一致|
|注册中心|服务实例注册,grpc协议|1.2万|1.2万|grpc协议压测工具没有支持，目前没有实际压测，理论不会比http协议低|
|注册中心|服务实例心跳,http协议|1.2万|1.0万|心跳是按实例计算和服务实例注册一致共享qps|
|注册中心|服务实例心跳,grpc协议|8万以上|n*8万|心跳是按请求链接计算,且不过注册中心处理线程,每个节点只需管理当前节点的心跳，集群总心跳qps是节点的倍数|
|注册中心|查询服务实例|3万|n*3万|集群的查询总qps是节点的倍数|


**注：** 具体结果和压测环境有关


## 容量分析

### 配置中心

1. 配置中心的单机查询8万qps，很高，又支持水平扩容；集群基本没有查询瓶颈。
2. 配置中心所占用的内存和配置内存有关，在内存没有满前，基本没有瓶颈。
3. 配置中心集群写入时统一在主节点写入，写入可能有瓶颈；目前1.5千tps,后面优化后应该能到1万 tps以上。

### 注册中心

1. 注册中心的单机查询3万qps，比较高，又支持水平扩容；集群基本没有查询瓶颈。
2. 注册中心所占用的内存和配置内存有关，在内存没有满前，基本没有瓶颈。
3. 注册中心集群写入时每个节点都要写一遍，整体集群的写入性能tps和单机理论上相当。
4. http协议(v1.x版本)和grpc协议(v2.x)的心跳维护机制不同；http心跳是按实例计算和服务实例注册一致共享qps, grpc的心跳是按请求链接计算且不过注册中心处理线程。所有这类协议理论支持的容量差别很大。



#### 注册中心集群注册容量推理

1. http协议注册+心跳qps是1万，每个实例5秒钟一次心跳；理论上只能支持5万服务实例左右。
2. grpc协议，注册qps假设也是1万，心跳qps单实例8万，3节点集群总心跳24万；如果平均一个应用实例1小时重连一次；支持注册的服务实例总数为：60*60*10000 = 3600万，心跳支持的链接实例总数为：5*24万=120万个链接实例（和集群节点有关）。

结论:
如果使用v1.0x http协议，支持的实例在5万个左右。
如果使用v2.0x grpc协议，理论上基本没有瓶颈。
