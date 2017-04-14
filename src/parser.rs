use loc::Loc;
use error::GoreErrorType as ET;
use error::{Result, err};
use token::TokenType as TT;
use token::Token;
use untyped_ast as ast;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            pos: 0
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn looking_at(&self, ty: TT) -> bool {
        !self.eof() && self.peek().ty == ty
    }

    fn looking_at_any(&self, tys: &[TT]) -> bool {
        for ty in tys {
            if self.looking_at(*ty) {
                return true;
            }
        }
        return false;
    }

    fn eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        self.peek_at(self.pos)
    }

    fn peek_prev(&self) -> &Token {
        self.peek_at(self.pos - 1)
    }

    fn peek_at(&self, offset: usize) -> &Token {
        if self.eof() {
            self.tokens.last().unwrap()
        } else {
            &self.tokens[offset]
        }
    }

    fn loc(&self) -> Loc {
        self.peek().loc.clone()
    }


    fn eat_or(&mut self, tok_ty: TT, err_ty: ET) -> Result<&Token> {
        if self.looking_at(tok_ty) {
            self.advance();
            return Ok(self.peek_prev());
        } else {
            return err(err_ty, self.loc());
        }
    }

    fn eat(&mut self, expected_tok: TT) -> Result<&Token> {
        if self.looking_at(expected_tok) {
            self.advance();
            return Ok(self.peek_prev());
        } else {
            let actual_tok = self.peek().ty;
            return err(
                ET::UnexpectedToken(actual_tok, vec![expected_tok]),
                self.loc()
            );
        }
    }

    pub fn parse(&mut self) -> Result<ast::Ast> {
        let loc = self.loc();

        self.eat_or(TT::Package, ET::MissingPackageDeclaration)?;
        let pname = self.eat_or(TT::Id, ET::MissingPackageName)
            .and_then(copy_lexeme)?;
        self.eat(TT::Semi)?;

        let mut decls = Vec::new();
        while self.looking_at_any(&[TT::Var, TT::Func, TT::Type]) {
            decls.extend(self.parse_toplevel_decl()?);
        }

        self.eat(TT::Eof)?;

        let ast = ast::Ast::new(pname, decls, loc);
        return Ok(ast);
    }

    fn parse_toplevel_decl(&mut self) -> Result<Vec<ast::TopLevelDecl>> {
        if self.looking_at(TT::Var) {
            let var_decls = self.parse_var_decl()?;
            let mut top_decls = Vec::new();
            for vd in var_decls {
                top_decls.push(ast::TopLevelDecl::VarDecl(vd));
            }
            self.eat(TT::Semi)?;
            return Ok(top_decls);
        }
        else if self.looking_at(TT::Type) {
            let ty_decls = self.parse_type_decl()?;
            let mut top_decls = Vec::new();
            for td in ty_decls {
                top_decls.push(ast::TopLevelDecl::TypeDecl(td));
            }
            self.eat(TT::Semi)?;
            return Ok(top_decls);
        }
        else if self.looking_at(TT::Func) {
            let func_decl = self.parse_func_decl()?;
            return Ok(vec![ast::TopLevelDecl::FuncDecl(func_decl)]);
        }
        else {
            return err(ET::ExpectedDeclaration, self.loc());
        }
    }

    // Too complex and convoluted
    fn parse_var_decl(&mut self) -> Result<Vec<ast::VarDecl>> {
        let var_loc = self.loc();
        self.advance(); // skip 'var' keyword

        if self.looking_at(TT::Id) {
            let (vnames, ty_opt, init_vec) = self.parse_var_spec(&var_loc)?;
            return Ok(construct_var_decls(vnames, ty_opt, init_vec, &var_loc));
        }
        else if self.looking_at(TT::LParen) {
            self.eat(TT::LParen)?;
            let mut decls = Vec::new();
            while self.looking_at(TT::Id) {
                let (vnames, ty_opt, init_vec) = self.parse_var_spec(&var_loc)?;
                decls.extend(
                    construct_var_decls(vnames, ty_opt, init_vec, &var_loc)
                );
                self.eat(TT::Semi)?;
            }
            self.eat(TT::RParen)?;
            if decls.is_empty() {
                return err(ET::InvalidVarDecl, var_loc);
            } else {
                return Ok(decls);
            }
        }
        else {
            return err(ET::InvalidVarDecl, self.loc());
        }
    }

    fn parse_var_spec(&mut self, loc: &Loc) ->
        Result<(Vec<ast::Id>, Option<ast::Ty>, Vec<ast::Expr>)> {
        let vnames = self.parse_id_list()?;
        let ty_opt =
            if self.looking_at_any(&[TT::Id, TT::LBracket, TT::Struct]) {
                Some(self.parse_ty()?)
            } else {
                None
            };
        let init_vec =
            if self.looking_at(TT::Assign) {
                self.advance();
                self.parse_expr_list()?
            } else {
                Vec::new()
            };

        match (ty_opt, init_vec.len()) {
            (None, 0) => {
                return err(ET::InvalidVarDecl, loc.clone());
            }
            (None, n) if n != vnames.len() => {
                return err(ET::VarExprLengthMismatch, loc.clone())
            }
            (ty_opt, n) => {
                if n != 0 && n != vnames.len() {
                    return err(ET::VarExprLengthMismatch, loc.clone())
                } else {
                    return Ok((vnames, ty_opt, init_vec));
                }
            }
        }
    }

    fn parse_type_decl(&mut self) -> Result<Vec<ast::TypeDecl>> {
        let ty_loc = self.loc();
        self.advance();
        if self.looking_at(TT::Id) {
            let ty_decl = self.parse_one_type_decl()?;
            return Ok(vec![ty_decl]);
        }
        else if self.looking_at(TT::LParen) {
            self.eat(TT::LParen);
            let mut ty_decls = Vec::new();
            while self.looking_at(TT::Id) {
                ty_decls.push(self.parse_one_type_decl()?);
                self.eat(TT::Semi);
            }
            self.eat(TT::RParen);
            if ty_decls.is_empty() {
                return err(ET::InvalidTypeDecl, ty_loc);
            } else {
                return Ok(ty_decls);
            }
        }
        else {
            return err(ET::InvalidTypeDecl, ty_loc);
        }
    }

    fn parse_one_type_decl(&mut self) -> Result<ast::TypeDecl> {
        let loc = self.loc();
        self.eat(TT::Id);
        let id = copy_lexeme(self.peek_prev())?;
        let ty = self.parse_ty().or_else(|gore_err|
            err(ET::InvalidTypeDecl, gore_err.loc)
        )?;
        return Ok(ast::TypeDecl::new(id, ty, loc));
    }

    fn parse_func_decl(&mut self) -> Result<ast::FuncDecl> {
        let loc = self.loc();
        self.eat(TT::Func)?;
        let func_name = self.parse_id()?;
        self.eat_or(TT::LParen, ET::ExpectedParamList)?;
        self.eat(TT::RParen)?;
        self.eat(TT::LBrace)?;
        self.eat(TT::RBrace)?;
        self.eat(TT::Semi)?;
        return Ok(ast::FuncDecl::new(
            func_name,
            vec![],
            ast::Ty::Void,
            vec![],
            loc
        ));
    }

    fn parse_id(&mut self) -> Result<ast::Id> {
        self.eat(TT::Id).and_then(copy_lexeme)
    }

    fn parse_id_list(&mut self) -> Result<Vec<ast::Id>> {
        let mut ids = Vec::new();
        ids.push(self.parse_id()?);
        while self.looking_at(TT::Comma) {
            self.advance();
            ids.push(self.parse_id()?);
        }
        return Ok(ids);
    }

    fn parse_ty(&mut self) -> Result<ast::Ty> {
        if self.looking_at(TT::Id) {
            let tyname = self.eat(TT::Id)
                .and_then(copy_lexeme)?;
            return Ok(ast::Ty::Name(tyname));
        } else if self.looking_at(TT::LBracket) {
            self.advance();
            if self.looking_at(TT::Int) || self.looking_at(TT::IntHex) {
                let size = usize_lexeme(self.peek())?;
                self.advance();
                let _ = self.eat(TT::RBracket);
                let sub_ty = self.parse_ty()?;
                return Ok(ast::Ty::Array(size, Box::new(sub_ty)));
            } else {
                let _ = self.eat(TT::RBracket);
                let sub_ty = self.parse_ty()?;
                return Ok(ast::Ty::Slice(Box::new(sub_ty)));
            }
        }
        else {
            return err(ET::Internal("todo: list valid tokens".to_string()), self.loc());
        }
    }

    fn parse_expr(&mut self) -> Result<ast::Expr> {
        let loc = self.loc();
        if self.looking_at(TT::Id) {
            let id = self.parse_id()?;
            return Ok(ast::Expr::new(Box::new(ast::ExprKind::Id(id)), loc));
        } else if self.looking_at(TT::Int) {
            let int_val = self.eat(TT::Int)
                .and_then(i64_lexeme)?;
            return Ok(ast::Expr::new(Box::new(ast::ExprKind::Int(int_val)), loc));
        } else {
            return err(ET::ExpectedExpression, self.loc());
        }
    }

    fn parse_expr_list(&mut self) -> Result<Vec<ast::Expr>> {
        let mut exprs = Vec::new();
        exprs.push(self.parse_expr()?);
        while self.looking_at(TT::Comma) {
            self.advance();
            exprs.push(self.parse_expr()?);
        }
        return Ok(exprs);
    }
}


fn copy_lexeme(t: &Token) -> Result<String> {
    match t.lexeme {
        None => err(ET::MissingLexeme, t.loc.clone()),
        Some(ref s) => Ok(s.clone())
    }
}

fn usize_lexeme(t: &Token) -> Result<usize> {
    let lexeme = copy_lexeme(t)?;
    match lexeme.parse::<usize>() {
        Ok(n) => Ok(n),
        Err(_) => err(ET::MissingLexeme, t.loc.clone()),
    }
}

fn i64_lexeme(t: &Token) -> Result<i64> {
    let lexeme = copy_lexeme(t)?;
    match lexeme.parse::<i64>() {
        Ok(n) => Ok(n),
        Err(_) => err(ET::MissingLexeme, t.loc.clone()),
    }
}


fn construct_var_decls(mut vnames: Vec<ast::Id>,
                       ty_opt: Option<ast::Ty>,
                       mut init_vec: Vec<ast::Expr>,
                       var_loc: &Loc)
                       -> Vec<ast::VarDecl> {
    // pre-cond: init_vec.len() == 0 || init_vec.len() == vnames.len()
    let mut decls = Vec::new();
    while let Some(vname) = vnames.pop() {
        let init_exp = init_vec.pop();
        decls.push(ast::VarDecl::new(vname,
                                     ty_opt.clone(),
                                     init_exp,
                                     var_loc.clone()));
    }
    decls.reverse();
    return decls;
}
