# Concurrent Indexing

为了方便性能对照，项目内有三个Index

- SimpleIndex. 加锁的`BTreeMap`。
- CSIndex. `crossbeam-skiplist`。
- SkipMapIndex. 于本次比赛实现的skiplist。

结果是……根本打不过:(