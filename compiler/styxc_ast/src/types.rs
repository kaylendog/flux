/// A trait implementable by typed objects allowing them
pub trait Typed {
	fn ty(&self) -> Type;
}

impl Typed for Expr {
    fn ty(&self) -> Type {
        use Expr::*;
		match self {
			
		}
    }
}