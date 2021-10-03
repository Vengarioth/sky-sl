#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    /// The equals operator, compares both operands for equality, e.g. `a == b`
    Equals,

    /// The not equals operator, compares both operands for inequality, e.g. `a != b`
    NotEquals,

    /// The greater than operator, compares both operands for the first being greater, e.g. `a > b`
    GreaterThan,

    /// The greater or equal than operator, e.g. ` a >= b`
    GreatherOrEqualThan,

    /// The less than operator, e.g. `a < b`
    LessThan,

    /// The less or equal than operator, e.g. `a <= b`
    LessOrEqualThan,

    /// The add operator, e.g. `a + b`
    Add,

    /// The subtract operator, e.g. `a - b`
    Subtract,

    /// The multiply operator, e.g. `a * b`
    Multiply,

    /// The divide operator, e.g. `a / b`
    Divide,

    /// The remainder operator, e.g. `a % b`
    Remainder,

    /// The bitwise and operator, e.g. `a & b`
    BitwiseAnd,

    /// The bitwise or operator, e.g. `a | b`
    BitwiseOr,

    /// The bitwise exclusive or (XOR) operator, e.g. `a ^ b`
    BitwiseXOr,

    /// The left shift operator, e.g. `a << b`
    LeftShift,

    /// The Right shift operator, e.g. `a << b`
    RightShift,

    /// The exponent operator, e.g. `a ** b`
    Exponent,
}

impl BinaryOperator {
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::Equals => 1,
            BinaryOperator::NotEquals => 1,
            BinaryOperator::GreaterThan => 1,
            BinaryOperator::GreatherOrEqualThan => 1,
            BinaryOperator::LessThan => 1,
            BinaryOperator::LessOrEqualThan => 1,
            BinaryOperator::Add => 4,
            BinaryOperator::Subtract => 4,
            BinaryOperator::Multiply => 5,
            BinaryOperator::Divide => 5,
            BinaryOperator::Remainder => 0,
            BinaryOperator::BitwiseAnd => 2,
            BinaryOperator::BitwiseOr => 2,
            BinaryOperator::BitwiseXOr => 2,
            BinaryOperator::LeftShift => 3,
            BinaryOperator::RightShift => 3,
            BinaryOperator::Exponent => 6,
        }
    }

    pub fn associativity(&self) -> Associativity {
        match self {
            BinaryOperator::Equals => Associativity::Left,
            BinaryOperator::NotEquals => Associativity::Left,
            BinaryOperator::GreaterThan => Associativity::Left,
            BinaryOperator::GreatherOrEqualThan => Associativity::Left,
            BinaryOperator::LessThan => Associativity::Left,
            BinaryOperator::LessOrEqualThan => Associativity::Left,
            BinaryOperator::Add => Associativity::Left,
            BinaryOperator::Subtract => Associativity::Left,
            BinaryOperator::Multiply => Associativity::Left,
            BinaryOperator::Divide => Associativity::Left,
            BinaryOperator::Remainder => Associativity::Left,
            BinaryOperator::BitwiseAnd => Associativity::Left,
            BinaryOperator::BitwiseOr => Associativity::Left,
            BinaryOperator::BitwiseXOr => Associativity::Left,
            BinaryOperator::LeftShift => Associativity::Right,
            BinaryOperator::RightShift => Associativity::Right,
            BinaryOperator::Exponent => Associativity::Right,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    /// The boolean not operator, e.g. `!a`
    BooleanNot,

    /// The bitwise not operator, e.g. `~a`
    BitwiseNot,

    /// The keep sign operator, e.g. `+a`
    KeepSign,

    /// The flip sign operator, e.g. `-a`
    FlipSign,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Associativity {
    Left,
    Right,
}

impl Associativity {
    pub fn is_left(self) -> bool {
        match self {
            Associativity::Left => true,
            Associativity::Right => false,
        }
    }
}
