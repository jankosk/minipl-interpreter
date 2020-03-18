use crate::ast::{BinaryOperator, Expression, Program, Statement};
use crate::utils::{EvalError, Type};
use std::collections::HashMap;

type EvalResult<T> = Result<T, EvalError>;

pub struct Evaluator {
    global_scope: HashMap<String, Type>,
    program: Program,
}

impl Evaluator {
    pub fn new(program: Program) -> Self {
        Evaluator {
            global_scope: HashMap::new(),
            program,
        }
    }

    pub fn evaluate_program(&mut self) -> EvalResult<()> {
        for statement in self.program.statements.clone() {
            self.evaluate_statement(&statement)?;
        }
        Ok(())
    }

    fn evaluate_statement(&mut self, statement: &Statement) -> EvalResult<()> {
        match statement {
            Statement::Assignment(id, exp) => self.evaluate_assignment(id, exp),
            Statement::Print(exp) => self.evaluate_print(exp),
        }
    }

    fn evaluate_assignment(&mut self, identifier: &String, exp: &Expression) -> EvalResult<()> {
        let val = self.evaluate_expression(exp)?;
        self.global_scope.insert(identifier.clone(), val);
        Ok(())
    }

    fn evaluate_print(&self, exp: &Expression) -> EvalResult<()> {
        let val = self.evaluate_expression(exp)?;
        println!("{}", val);
        Ok(())
    }

    fn evaluate_expression(&self, exp: &Expression) -> EvalResult<Type> {
        match exp {
            Expression::IntegerConstant(val) => Ok(Type::Integer(*val)),
            Expression::Binary(exp1, op, exp2) => self.evaluate_binary(exp1, op, exp2),
            Expression::Identifier(id) => {
                let val = self.global_scope.get(id);
                match val {
                    Some(val) => Ok(val.clone()),
                    _ => Err(EvalError::VariableNotInitialized(id.clone())),
                }
            }
            _ => Err(EvalError::SyntaxError),
        }
    }

    fn evaluate_binary(
        &self,
        left: &Box<Expression>,
        op: &BinaryOperator,
        right: &Box<Expression>,
    ) -> EvalResult<Type> {
        let left = self.evaluate_expression(&*left)?;
        let right = self.evaluate_expression(&*right)?;
        match (left, right) {
            (Type::Integer(val1), Type::Integer(val2)) => match op {
                BinaryOperator::Plus => Ok(Type::Integer(val1 + val2)),
                BinaryOperator::Minus => Ok(Type::Integer(val1 - val2)),
                _ => Err(EvalError::UnsupportedOperation),
            },
            _ => Err(EvalError::MismatchedTypes),
        }
    }
}
