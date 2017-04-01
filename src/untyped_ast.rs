use loc::Loc;

type Id = String;


#[derive(Debug)]
struct Ast {
    package_name: Id,
    declarations: Vec<TopLevelDecl>,
    loc: Loc
}

#[derive(Debug)]
enum TopLevelDecl {
    FunDecl(FunDecl),
    TypeDecl(TypeDecl),
    VarDecl(VarDecl)
}

#[derive(Debug)]
struct FunDecl {
    name: Id,
    params: Vec<Param>,
    return_ty: Ty,
    body: Vec<Stmt>,
    loc: Loc
}

#[derive(Debug)]
struct TypeDecl {
    name: Id,
    ty: Ty,
    loc: Loc
}

#[derive(Debug)]
struct VarDecl {
    name: Id,
    ty: Ty,
    init: Option<Expr>,
    loc: Loc
}

#[derive(Debug)]
struct Stmt {
    kind: Box<StmtKind>,
    loc: Loc
}

#[derive(Debug)]
enum StmtKind {
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
enum IfStmt {
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
enum LValue {
    Id(Id),
    Blank,
    ArrayAccess { lvalue: Box<LValue>, expr: Expr },
    StructAccess { lvalue: Box<LValue>, field: Id }
}


#[derive(Debug)]
enum BinOp {
    Add, Sub, Mul, Div, Mod,
    BitAnd, BitOr,
    And, Or,
    Lt, Le, Gt, Ge, Eq, Ne,
    LShift, RShift
}

#[derive(Debug)]
enum UnaryOp {
    Not, Bitnot, BitClear, Negate
}


#[derive(Debug)]
struct Expr {
    expr_kind: Box<ExprKind>,
    loc: Loc
}

#[derive(Debug)]
enum ExprKind {
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


#[derive(Debug)]
enum Ty {
    Int,
    Float64,
    Bool,
    String,
    Rune,

    Id(Id),
    Slice(Box<Ty>),
    Array(usize, Box<Ty>),
    Struct(Vec<Param>),
    Func(Vec<Ty>, Box<Ty>),

    Void, // Not an actual Go type, used for func return types
    TyAlias(Id, Box<Ty>)
}

#[derive(Debug)]
struct Param {
    name: Id,
    ty: Box<Ty>
}
