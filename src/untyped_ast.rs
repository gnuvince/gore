use loc::Loc;

pub type Id = String;

#[derive(Debug)]
pub struct Ast {
    package_name: Id,
    declarations: Vec<TopLevelDecl>,
    loc: Loc
}

impl Ast {
    pub fn new(name: Id, decls: Vec<TopLevelDecl>, loc: Loc) -> Ast {
        Ast {
            package_name: name,
            declarations: decls,
            loc: loc
        }
    }
}

#[derive(Debug)]
pub enum TopLevelDecl {
    FunDecl(FunDecl),
    TypeDecl(TypeDecl),
    VarDecl(VarDecl)
}

#[derive(Debug)]
pub struct FunDecl {
    name: Id,
    params: Vec<Param>,
    return_ty: Ty,
    body: Vec<Stmt>,
    loc: Loc
}

#[derive(Debug)]
pub struct TypeDecl {
    name: Id,
    ty: Ty,
    loc: Loc
}

#[derive(Debug)]
pub struct VarDecl {
    name: Id,
    ty: Option<Ty>,
    init: Option<Expr>,
    loc: Loc
}

impl VarDecl {
    pub fn new(name: Id, ty: Option<Ty>, init: Option<Expr>, loc: Loc) -> VarDecl {
        VarDecl { name: name, ty: ty, init: init, loc: loc }
    }
}

#[derive(Debug)]
pub struct Stmt {
    kind: Box<StmtKind>,
    loc: Loc
}

#[derive(Debug)]
pub enum StmtKind {
    Empty,
    Break,
    Continue,
    Return { expr: Option<Expr> },
    Print { expr: Vec<Expr>, newline: bool },
    VarDecl { decl: VarDecl },
    TypeDecl { decl: TypeDecl },
    ShortDecl { ids: Vec<Id>, exprs: Vec<Expr> },
    Assign { lvalues: Vec<LValue>, exprs: Vec<Expr> },
    AssignOp { lvalue: LValue, expr: Expr, op: BinOp },
    Incr { lvalue: LValue },
    Decr { lvalue: LValue },
    Call { id: Id, args: Vec<Expr> },
    Block { stmts: Vec<Stmt> },

    If(IfStmt),

    ForInfinite { stmts: Vec<Stmt> },
    ForWhile { expr: Expr, stmts: Vec<Stmt> },
    For {
        init: Option<Stmt>,
        cond: Option<Expr>,
        update: Option<Stmt>,
        stmts: Vec<Stmt>
    },

    Switch {
        init: Option<Stmt>,
        expr: Option<Expr>,
        cases: Vec<(Vec<Expr>, Vec<Stmt>)>,
        default: Vec<Stmt>
    }
}

#[derive(Debug)]
pub enum IfStmt {
    If {
        init: Option<Stmt>,
        cond: Expr,
        stmts: Vec<Stmt>
    },
    IfElse {
        init: Option<Stmt>,
        cond: Expr,
        then_stmts: Vec<Stmt>,
        else_stmts: Vec<Stmt>
    },
    IfElseIf {
        init: Option<Stmt>,
        cond: Expr,
        then_stmts: Vec<Stmt>,
        else_if: Box<IfStmt>
    }
}


#[derive(Debug)]
pub enum LValue {
    Id(Id),
    Blank,
    ArrayAccess { lvalue: Box<LValue>, expr: Expr },
    StructAccess { lvalue: Box<LValue>, field: Id }
}


#[derive(Debug, Clone)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod,
    BitAnd, BitOr,
    And, Or,
    Lt, Le, Gt, Ge, Eq, Ne,
    LShift, RShift
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not, Bitnot, BitClear, Negate
}


#[derive(Debug, Clone)]
pub struct Expr {
    expr_kind: Box<ExprKind>,
    loc: Loc
}

impl Expr {
    pub fn new(kind: Box<ExprKind>, loc: Loc) -> Expr {
        Expr {
            expr_kind: kind,
            loc: loc
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    // Base expressions
    Id(Id),
    Int(i64),
    Float64(f64),
    String(String),
    Rune(char),

    // Compound expressions
    UnaryOp { op: UnaryOp, expr: Box<Expr> },
    BinaryOp { op: BinOp, expr1: Box<Expr>, expr2: Box<Expr> },
    Append { slice: Box<Expr>, element: Box<Expr> },
    Call { func: Id, args: Vec<Expr> }, // either func-call or cast
    Cast { ty: Ty, expr: Box<Expr> }, // cast
    IndexAccess { array: Box<Expr>, index: Box<Expr> }, // slice or array access
    FieldAccess { record: Box<Expr>, field: Id }
}


#[derive(Debug, Clone)]
pub enum Ty {
    Name(Id),
    Slice(Box<Ty>),
    Array(usize, Box<Ty>),
    Struct(Vec<Param>),
    Func(Vec<Ty>, Box<Ty>),

    Void, // Not an actual Go type, used for func return types
}

#[derive(Debug, Clone)]
pub struct Param {
    name: Id,
    ty: Box<Ty>
}
