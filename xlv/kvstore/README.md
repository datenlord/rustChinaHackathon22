### 请求流程 
如图所示
![img_1.png](img_1.png)

### 开发过程
  由于一开始准备通过atomic实现cas insert数据的，但是后面因为rust语言不熟悉放弃了这个思路，改成通过request请求进入一个lockfree_queue，然后queue pull出来的时候通过单线程写入skiplist，类似简易的存算分离
  。思路是写queue时是多线程写入，存储时是单线程顺序写存储，顺序写存储的单线程直接cpu绑核，防止线程上下文切换开销。
  
#### 已完成
- 简易的skiplist（没有经过并发测试）
  - 无锁的queue（也没有经过并发测试，借助`crossbeam::epoch`完成的）
  - 实现了trait接口

#### 未完成
- 当kv数据封装成entry存储进入queue时没有写`serde`压缩
- 没有考虑好在queue中读取时如何range读取
- 从queue中消费出来写skiplist时，由于加了额外的trait bound导致写入报错 目前还没怎么解决
#### 总结
1. 虽然项目没有完成仅仅是个半成品 但是我俩还是在中学到很多东西，也了解到了rust的难度
2. 后续还是继续深入学习rust吧 这次开发 思路有，语言层面给我们拦住了，只能怪太菜了..
3. 开发人员：@uran0SH(主导项目) @azhsmesos