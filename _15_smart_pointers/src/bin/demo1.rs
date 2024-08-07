/// 指针是一个持有内存地址的值，可以通过解引用来访问它指向的内存地址，
/// 理论上可以解引用到任意数据类型，
/// 引用是一个特殊的指针，它的解引用访问是受限的，
/// 只能解引用到它引用数据的类型，不能用用作 它用
///
/// 智能指针
///
/// 是一个表现行为很像指针的数据结构，但除了指向数据的指针外，它还有元数据以提供额外的处理能力
///
/// 这个定义有点模糊，我们对比其他的数据结构来明确一下
///
/// 很像之前的胖指针，智能指针一定是一个胖指针，
/// 但胖指针不一定是一个智能指针
///
/// 比如 &str 就只是一个胖指针，它有指向堆内存字符串的指针，
/// 同时还有关于字符串长度的元数据
///
/// 我们看看 智能指针 String 和 &str 的区别
///
/// 从图上可以看到，String 除了多了一个 capacity 字段，似乎也没什么特殊，
/// 但 String 对堆上的值有所有权，而 &str 是没有所有权的，
/// 这是 Rust 中智能指针和普通胖指针的区别
///
/// 那又有一个问题，智能指针，和结构体又有什么区别呢，
/// String 是用结构体定义的
/// 和普通结构体不同的是，String 实现了 Deref DerefMut ，
/// 这使得它在解引用的时候，会得到 &str
///
/// 另外，由于在堆上分配了数据，String 还需要为其分配的资源做相应的回收，
/// 而 String 内部使用了 Vec<u8> ，所以它可以依赖 Vec<T> 的能力来释放堆内存，
///
/// 所以，再清晰定义一下，在 Rust 中，凡是需要做资源回收的数据结构，
/// 且实现了 Deref/DerefMut/Drop 都是智能指针
///
/// 按照这个定义，除了 String，在之前课程中我们遇到了很多智能指针，
/// 比如用于在堆上分配内存的 Box<T> Vec<T>
/// 用于引用计数的 Rc<T> Arc<T>
/// 很多其他结构，PathBuf, Cow<'a, B> MutexGuard<T> RwLockReadGuard<T>
/// RlLockWriteGuard 等也是智能指针
///
/// 今天我们深入分析 三个使用智能指针的数据结构
/// 在堆上创建内存的 Box<T>
/// 提供写时克隆的 Cow<'a, B>
/// 以及用于数据加锁的 MutexGuard<T>
fn main() {}


pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    if m == 0 && n == 0 {
        return;
    }

    // for i in 0..n {
    //     nums1[(m + i) as usize] = nums2[i as usize];
    // }
    //
    // nums1.sort();

    let (mut p1, mut p2) = ((m - 1) as usize, (n - 1) as usize);
    let mut tail = (m + n - 1) as usize;
    let mut cur;

    while p1 >= 0 || p2 >= 0 {
        if p1 <= 0 {
            cur = nums2[p2];
            p2 -= 1;
        } else if p2 <= 0 {
            cur = nums1[p1];
            p1 -= 1;
        } else if nums1[p1] > nums2[p2] {
            cur = nums1[p1];
            p1 -= 1;
        } else {
            cur = nums2[p2];
            p2 -= 1;
        }

        nums1[tail] = cur;
        tail -= 1;
    }
}