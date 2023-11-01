// // pub use d::DExt;
// // pub use f::FExt;
// // pub use q::QExt;
//
// use crate::cpu::isa::ext::IsaExt;
//
// mod d;
// mod f;
// mod q;
//
// pub enum FloatExt {
//     F(FExt),
//     D(DExt),
//     Q(QExt),
// }

pub enum FloatExt {
    F(),
    D(),
    Q(),
}