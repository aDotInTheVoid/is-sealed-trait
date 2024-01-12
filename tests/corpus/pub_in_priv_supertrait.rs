mod private {
    pub trait Super {}
}
pub trait Sealed: private::Super {}
