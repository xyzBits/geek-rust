/// Add<Rhs> trait ，允许你重载加法运算符，
/// Rust 为所有的运行符都提供了 trait，
/// 你可以为自己的类型重载某些操作符，
///
/// 加 Add +  AddSign +=
/// 乘 Mul *  MulAssign *=
/// 减 Sub -  SubAssign -=
/// 除 Div /  DivAssign /=
/// 负 Neg -
/// 余 Rem %  RemAssign %=
///
/// 索引和范围
/// 只读索引  Index []
/// 可变索引 IndexMut []
/// 范围 RangeBounds (.., a.., ..b, a..b, ..=c, f..=g)
///
/// 位运算
/// 位与 BitAnd & BitAndAssign &=
/// 位或 BitOr |  BitOrAssign |=
/// 异或 BitXor ^ BitXorAssign ^=
/// 左移 Shl <<   ShlAssign <<=
/// 右移 Shr >>   ShrAssign >>=
/// 非 Not !
///
/// 解引用
/// 只读解引用 Deref *v
/// 可变解引用 DerefMut *v = ...
///
/// 其他
/// 析构 Drop
/// 闭包 Fn FnMut FnOnce
///
/// 对于普通的引用，解引用很直观，因为它只有一个指向值的地址，从这个地址
/// 可以获得所需要的值

fn main() {

    let mut x = 42;
    // 只有一个指向值的地址，从这个地址可以获取到所需要的值
    let y = &mut x;

    // 解引用，内部调用 DerefMut ，其实现就是 *self
    // *(y.deref()) = *y
    *y += 1;
}