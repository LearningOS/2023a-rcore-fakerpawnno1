# lab1 实验报告

# 1 实验说明
本实验主要实现了统计当前task运行期间的系统调用次数和系统调用时刻距离任务第一次被调度时刻的时长。其中第一个任务调用时间，在run_first_task中switch之前可以设置。switch之后就开始执行。
其他的任务在run_next_task中通过判断是否被启动来设置启动时间。未启动task在switch之后同样开始执行。

# 2 简答题
1. 问题1
   
    RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.0 

    ch2_bad_address   程序出错  PageFault in application, bad addr = 0x0, bad instruction = 0x804003c4, kernel killed it.

    ch2_bad_instructions  程序出错信息 [kernel] IllegalInstruction in application, kernel killed it.

    ch2_bad_register 程序出错信息 [kernel] IllegalInstruction in application, kernel killed it.

2. 问题2
   1. 刚进入__restore的时候，a0是内核trap上下文的地址。 __restore有两个场景，一个是每个应用第一次从内核态进入用户态，第二个是用户发出系统调用之后通过__restore返回到下一个指令地址。
   2. 处理了sstatus、sepc、sscratch 、sstatus记录了当前、sepc记录了跳转会用户态接下来程序指令地址、sscratch保存了用户栈sp。
   3. 此时x2试内核栈，在后面通过sscratch 恢复，x4不需要保存
   
   4. 交换了用户栈和内核栈，，现在sp 指向内核栈，sscratch 指向用户栈
   5. sret指令 此时的sepc已经是用户的下一条指令地址。
   6. 交换了用户栈和内核栈，，现在sp 指向用户栈，sscratch 指向内核栈
   7. U态进入S态是在call trap_handler 时

# 3荣誉准则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

    微信id：jxy362573 、ted、亲情珍贵

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

    https://cloud.tsinghua.edu.cn/f/17a7c9d9b57f4838ae5f/

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。




