//! This module contains types to manage the different kinds of secret keys.

/// This type is a marker for keys using binary elements as scalar.
// Todo: Naming
pub struct BinaryKeyKind;
/// This type is a marker for keys using ternary elements as scalar.
// Todo: Naming
pub struct TernaryKeyKind;
/// This type is a marker for keys using normaly sampled elements as scalar.
// Todo: Naming
pub struct GaussianKeyKind;
/// This type is a marker for keys using uniformly sampled elements as scalar.
// Todo: Naming
pub struct UniformKeyKind;

/// In concrete, secret keys can be based on different kinds of scalar values (put aside the
/// data type eventually used to store it in memory). This trait is implemented by marker types,
/// which are used to specify in the type system, what kind of keys we are currently using.
// Todo: Naming
pub trait KeyKind: seal::SealedKeyKind + Sync {}

impl KeyKind for BinaryKeyKind {}
impl KeyKind for TernaryKeyKind {}
impl KeyKind for GaussianKeyKind {}
impl KeyKind for UniformKeyKind {}

mod seal {
    pub trait SealedKeyKind {}
    impl SealedKeyKind for super::BinaryKeyKind {}
    impl SealedKeyKind for super::TernaryKeyKind {}
    impl SealedKeyKind for super::GaussianKeyKind {}
    impl SealedKeyKind for super::UniformKeyKind {}
}
