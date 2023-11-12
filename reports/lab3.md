# lab3 实验报告

# 1 实验说明
本实验主要实现了将fork和exec两个系统调用合并为1个系统调用spawn，核心在于新建进程空间，去掉fork中父进程的进程空间的复制，以及一个简单的stride进程调度算法。

# 2 简答题
1. 问题1
   不是，因为P2执行过后260溢出变为4，所以还是P2被执行。
2. 问题2
   1. 假设PASS_MAX为当前所有进程里最大的步进值，则对每次Stride调度器的调度步骤中，其最大的步进值STRIDE_MAX和最小的步进值STRIDE_MIN之差：STRIDE_MAX – STRIDE_MIN <= PASS_MAX，所有进程的最小priority为2，即最大的PASS_MAX为BIG_STRIDE/2，也即STRIDE_MAX – STRIDE_MIN <=BIG_STRIDE/2
3. ```Rust
   use core::cmp::Ordering;

   struct Stride(u64);

   impl PartialOrd for Stride {
      fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
         // ...
         let sub = other.0.wrapping_sub(self.0);
            if sub <= BIG_STRIDE {
                Ordering::Less
            } else {
                Ordering::Greater
            }
      }
   }

   impl PartialEq for Stride {
      fn eq(&self, other: &Self) -> bool {
         false
      }
   }
    


# 3荣誉准则
1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：


2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：


3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。




