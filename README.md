# rustChinaHackathon22
***Rust China Hackathon 2022 线上活动***

## 大赛主题

首届 Rust China Hackathon Online 来啦！本届 Hackathon 主题为「Rust for Fun」，期待与你一起用 Rust 释放创新的更多可能性。

本届  Hackathon 将面向更广泛人群，分为 「社区组」与 「企业组」两大赛道。无论你是应用开发者、游戏开发者、云原生开发者，还是嵌入式开发者，都可以找到适合自己的方向，一起“玩转” Rust。

本届 Hackathon 报名通道于 2022 年 11 月 15 日正式开启，选手们可以自行组队参赛，通过初赛甄选后，将在线上完成 路演与决赛答辩，优胜队伍将获得奖金、企业直聘通道等支持。
**报名入口：https://shimo.im/forms/NJkbE55P5WFLz7qR/fill**


## 参赛福利
- 线上同好技术交流
- 评委大咖资深点评
- 参赛者专属大礼包
- 参赛团队特别采访

## 赛题说明

### 赛题讲解

**腾讯会议**：
会议主题：【DatenLord】2022 Rust Hackathon 企业组题目宣讲

会议时间：**2022/12/4 16:00-17:00** (GMT+08:00) 中国标准时间 - 北京

点击链接入会，或添加至会议列表：
https://meeting.tencent.com/dm/y3paDmGk4U6B

#会议号：685-203-430

### 赛题信息 

**主题**：Concurrent Indexing

**背景**：Xline是达坦科技推出的分布式元数据KV存储器。在使用中，此系统需要处理来自客户端的高并发请求，数目有时甚至可以扩展到数百至数千个。为了在内存中定位到相应KV数据，我们维护了一个索引，而此索引也成为了整个系统的瓶颈之一。在这样的情况下，如何提高索引在处理并发请求时的性能就成为了一个问题。

**挑战**：在本次Hackathon中，你将需要通过创建这样一个并发索引来解决此问题，此索引应该满足以下要求：

- 客户端将并发地发送请求，但并不会发送冲突的请求。
- 无冲突的请求应当被并发执行，以提高生产量。

```Rust
/// Operations of Index
pub(crate) trait IndexOperate<K: Ord, V> {
    /// Get a range of keys in [key, range_end]
    fn get(&self, key: &K, range_end: &K) -> Vec<&V>;
    /// delete a range of keys in [key, range_end]     
    fn delete(&self, key: &K, range_end: &K) -> Vec<V>;
    /// insert of update a key     
    fn insert_or_update(&self, key: K, value: V) -> Option<V>;
}
```

你的实现应遵从上述 Trait 并满足上述要求。我们将使用一定基准测试来进行评估，并根据其结果评分。在基准测试中，我们将发送大量并发请求，因此你也可以创建自己的性能测试来帮助进行优化。
## 奖金安排
- 一等奖： 1组，1000 美金+ 社区限量大礼包 + 社区访谈 
- 二等奖： 1组，700 美金 + 社区限量大礼包
- 三等奖：1组，500 美金 + 社区限量大礼包
- 最佳参与奖：5组 ，社区限量大礼包

## 赛程安排
- 报名+组队：11.15～12.12 
- ***DatenLord 赛题深度讲解： 12.4 下午16:00-17:00***
- 组委会整理报名资料：12.12-12.14
- 开发：12.15～12.18 
- 作品提交：12.19 
- 作品初评：12.20～12.25 （评委会 + 社区投票）
- 大赛线上路演： 12.29（线上直播 ： 14:00 ～ 18:00）+ 观众投票 + 社区媒体
- 作品颁奖： 12.30
 
## 评审标准
由企业组+社区 共同组成的评审团进行评审，包含如下几个维度：
1. 代码完成度：5分
代码质量
效果设计
2. 性能 5分


## 参赛标准
参赛对象:在职人员、学生均可

## 组队规模
要求组队参加，每个团队 1～5人，每人只限参加一个团队。
对每个参赛人员进行审核。

## 作品提交
请按照规定时间提交至此 github 目录下

## 其他事项
1. 第一行代码 commit 的时间不得早于12月14日23:59，否则即视为违规，取消参赛资料
2. 每个项目一个独立目录提交，文件名为团队拼音

如有疑问，可邮件联系ruopeng.zhou@datenlord.com
