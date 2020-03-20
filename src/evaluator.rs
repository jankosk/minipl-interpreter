use crate::ast::{BinaryOperator, Expression, Program, Statement, UnaryOperator};
use crate::utils::{EvalError, Type, Value};
use std::collections::HashMap;

type EvalResult<T> = Result<T, EvalError>;

pub struct Evaluator {
    global_scope: HashMap<String, (Type, Value)>,
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
            Statement::NewAssignment(id, type_def, exp) => {
                self.evaluate_new_assignment(id, type_def, exp)
            }
            Statement::Assignment(id, exp) => self.evaluate_assignment(id, exp),
            Statement::Print(exp) => self.evaluate_print(exp),
            _ => panic!("not supported!"),
        }
    }

    fn evaluate_new_assignment(
        &mut self,
        identifier: &String,
        type_def: &Type,
        exp: &Expression,
    ) -> EvalResult<()> {
        let val = self.evaluate_expression(exp)?;
        self.check_type_conformance(type_def, &val)?;
        self.global_scope
            .insert(identifier.clone(), (type_def.clone(), val));
        Ok(())
    }

    fn evaluate_assignment(&mut self, identifier: &String, exp: &Expression) -> EvalResult<()> {
        let tuple = match self.global_scope.get(identifier) {
            Some((type_def, _)) => {
                let val = self.evaluate_expression(exp)?;
                self.check_type_conformance(type_def, &val)?;
                (type_def.clone(), val.clone())
            }
            None => return Err(EvalError::VariableNotInitialized(identifier.clone())),
        };
        self.global_scope.insert(identifier.clone(), tuple);
        Ok(())
    }

    fn evaluate_print(&self, exp: &Expression) -> EvalResult<()> {
        let val = self.evaluate_expression(exp)?;
        println!("{}", val);
        Ok(())
    }

    fn evaluate_expression(&self, exp: &Expression) -> EvalResult<Value> {
        match exp {
            Expression::IntegerConstant(val) => Ok(Value::Integer(*val)),
            Expression::StringValue(string) => Ok(Value::String(string.clone())),
            Expression::Boolean(boolean) => Ok(Value::Bool(*boolean)),
            Expression::Binary(exp1, op, exp2) => self.evaluate_binary(exp1, op, exp2),
            Expression::Unary(op, exp) => self.evaluate_unary(op, exp),
            Expression::Identifier(id) => match self.global_scope.get(id) {
                Some((_, val)) => Ok(val.clone()),
                _ => Err(EvalError::VariableNotInitialized(id.clone())),
            },
        }
    }

    fn evaluate_binary(
        &self,
        left: &Box<Expression>,
        op: &BinaryOperator,
        right: &Box<Expression>,
    ) -> EvalResult<Value> {
        let left = self.evaluate_expression(&*left)?;
        let right = self.evaluate_expression(&*right)?;
        match (left, right) {
            (Value::Integer(val1), Value::Integer(val2)) => match op {
                BinaryOperator::Plus => Ok(Value::Integer(val1 + val2)),
                BinaryOperator::Minus => Ok(Value::Integer(val1 - val2)),
                BinaryOperator::Multiplication => Ok(Value::Integer(val1 * val2)),
                BinaryOperator::Division => Ok(Value::Integer(val1 / val2)),
                _ => Err(EvalError::UnsupportedOperation),
            },
            (Value::Bool(bool1), Value::Bool(bool2)) => match op {
                BinaryOperator::And => Ok(Value::Bool(bool1 && bool2)),
                _ => Err(EvalError::UnsupportedOperation),
            },
            (Value::String(str1), Value::String(str2)) => match op {
                BinaryOperator::Plus => Ok(Value::String(str1 + &str2)),
                _ => Err(EvalError::UnsupportedOperation),
            },
            _ => Err(EvalError::MismatchedTypes),
        }
    }

    fn evaluate_unary(&self, op: &UnaryOperator, exp: &Expression) -> EvalResult<Value> {
        let val = self.evaluate_expression(&*exp)?;
        match val {
            Value::Bool(boolean) => match op {
                UnaryOperator::Not => Ok(Value::Bool(!boolean)),
            },
            _ => Err(EvalError::MismatchedTypes),
        }
    }

    fn check_type_conformance(&self, type_def: &Type, val: &Value) -> EvalResult<()> {
        match val {
            Value::Bool(_) => match type_def {
                Type::Boolean => Ok(()),
                _ => Err(EvalError::MismatchedTypes),
            },
            Value::Integer(_) => match type_def {
                Type::Integer => Ok(()),
                _ => Err(EvalError::MismatchedTypes),
            },
            Value::String(_) => match type_def {
                Type::String => Ok(()),
                _ => Err(EvalError::MismatchedTypes),
            },
        }
    }
}
