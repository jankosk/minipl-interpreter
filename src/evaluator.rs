use crate::ast::{BinaryOperator, Expression, Program, Statement, UnaryOperator};
use crate::utils::{EvalError, Type, Value};
use std::collections::HashMap;
use std::io::{stdout, stdin, Write};

type EvalResult<T> = Result<T, EvalError>;
type GlobalVar = (Type, Option<Value>);

pub struct Evaluator {
    global_scope: HashMap<String, GlobalVar>,
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
            self.evaluate_statement(statement)?;
        }
        Ok(())
    }

    fn evaluate_statement(&mut self, statement: Statement) -> EvalResult<()> {
        match statement {
            Statement::NewAssignment(id, type_def, exp) => {
                self.evaluate_new_assignment(id, type_def, exp)
            }
            Statement::VarInitialization(id, type_def) => self.evaluate_var_init(id, type_def),
            Statement::Assignment(id, exp) => self.evaluate_assignment(id, exp),
            Statement::Print(exp) => self.evaluate_print(exp),
            Statement::Assert(exp) => self.evaluate_assert(exp),
            Statement::Read(id) => self.evaluate_read(id),
            Statement::For(id, start, end, stmts) => self.evaluate_for(id, start, end, stmts),
        }
    }

    fn evaluate_new_assignment(
        &mut self,
        identifier: String,
        type_def: Type,
        exp: Expression,
    ) -> EvalResult<()> {
        let val = self.evaluate_expression(exp)?;
        self.check_type_conformance(&type_def, &val)?;
        self.add_new_variable(identifier, (type_def, Some(val)))?;
        Ok(())
    }

    fn evaluate_var_init(&mut self, id: String, type_def: Type) -> EvalResult<()> {
        self.global_scope.insert(id, (type_def, None));
        Ok(())
    }

    fn evaluate_assignment(&mut self, identifier: String, exp: Expression) -> EvalResult<()> {
        let (type_def, _) = self.find_assigned_variable(&identifier)?;
        let val = self.evaluate_expression(exp)?;
        self.check_type_conformance(&type_def, &val)?;
        self.global_scope.insert(identifier, (type_def, Some(val)));
        Ok(())
    }

    fn evaluate_print(&mut self, exp: Expression) -> EvalResult<()> {
        let val = self.evaluate_expression(exp)?;
        print!("{}", val);
        stdout().flush().unwrap();
        Ok(())
    }

    fn evaluate_read(&mut self, id: String) -> EvalResult<()> {
        let (type_def, _) = self.find_assigned_variable(&id)?;
        let input = self.get_input()?;
        match type_def {
            Type::String => {
                let val = Value::String(input);
                self.global_scope.insert(id, (type_def, Some(val)));
            }
            Type::Integer => {
                let parsed = input.trim().parse::<i32>();
                match parsed {
                    Ok(int) => {
                        let val = Value::Integer(int);
                        self.global_scope.insert(id, (type_def, Some(val)));
                    }
                    Err(_) => return Err(EvalError::MismatchedTypes),
                }
            }
            _ => return Err(EvalError::MismatchedTypes),
        };
        Ok(())
    }

    fn evaluate_assert(&mut self, exp: Expression) -> EvalResult<()> {
        let val = self.evaluate_expression(exp.clone())?;
        match val {
            Value::Bool(boolean) if !boolean => {
                println!("Assertion failed: {}", &exp);
                Ok(())
            }
            Value::Bool(_) => Ok(()),
            _ => return Err(EvalError::MismatchedTypes),
        }
    }

    fn evaluate_for(
        &mut self,
        id: String,
        exp1: Expression,
        exp2: Expression,
        stmts: Vec<Box<Statement>>,
    ) -> EvalResult<()> {
        let start = match self.evaluate_expression(exp1) {
            Ok(Value::Integer(int)) => int,
            _ => return Err(EvalError::MismatchedTypes),
        };
        let end = match self.evaluate_expression(exp2) {
            Ok(Value::Integer(int)) => int,
            _ => return Err(EvalError::MismatchedTypes),
        };
        let (type_def, _) = self.find_assigned_variable(&id)?;
        if type_def != Type::Integer {
            return Err(EvalError::MismatchedTypes);
        }
        for i in start..=end {
            let loop_val = Value::Integer(i);
            self.global_scope
                .insert(id.clone(), (Type::Integer, Some(loop_val)));
            for stmt in stmts.clone() {
                self.evaluate_statement(*stmt)?;
            }
        }
        Ok(())
    }

    fn evaluate_expression(&mut self, exp: Expression) -> EvalResult<Value> {
        match exp {
            Expression::IntegerConstant(val) => Ok(Value::Integer(val)),
            Expression::StringValue(string) => Ok(Value::String(string.clone())),
            Expression::Boolean(boolean) => Ok(Value::Bool(boolean)),
            Expression::Binary(exp1, op, exp2) => self.evaluate_binary(exp1, op, exp2),
            Expression::Unary(op, exp) => self.evaluate_unary(op, *exp),
            Expression::Identifier(id) => {
                let (_, opt) = self.find_assigned_variable(&id)?;
                match opt {
                    Some(val) => Ok(val.clone()),
                    None => Err(EvalError::VariableNotInitialized(id.clone())),
                }
            }
        }
    }

    fn evaluate_binary(
        &mut self,
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    ) -> EvalResult<Value> {
        let left = self.evaluate_expression(*left)?;
        let right = self.evaluate_expression(*right)?;
        match (left, right) {
            (Value::Integer(val1), Value::Integer(val2)) => match op {
                BinaryOperator::Plus => Ok(Value::Integer(val1 + val2)),
                BinaryOperator::Minus => Ok(Value::Integer(val1 - val2)),
                BinaryOperator::Multiplication => Ok(Value::Integer(val1 * val2)),
                BinaryOperator::Division => Ok(Value::Integer(val1 / val2)),
                BinaryOperator::Equals => Ok(Value::Bool(val1 == val2)),
                BinaryOperator::LessThan => Ok(Value::Bool(val1 < val2)),
                BinaryOperator::GreaterThan => Ok(Value::Bool(val1 > val2)),
                _ => Err(EvalError::UnsupportedOperation),
            },
            (Value::Bool(bool1), Value::Bool(bool2)) => match op {
                BinaryOperator::And => Ok(Value::Bool(bool1 && bool2)),
                BinaryOperator::Equals => Ok(Value::Bool(bool1 == bool2)),
                BinaryOperator::LessThan => Ok(Value::Bool(bool1 < bool2)),
                BinaryOperator::GreaterThan => Ok(Value::Bool(bool1 > bool2)),
                _ => Err(EvalError::UnsupportedOperation),
            },
            (Value::String(str1), Value::String(str2)) => match op {
                BinaryOperator::Plus => Ok(Value::String(str1 + &str2)),
                BinaryOperator::Equals => Ok(Value::Bool(str1 == str2)),
                BinaryOperator::LessThan => Ok(Value::Bool(str1 < str2)),
                BinaryOperator::GreaterThan => Ok(Value::Bool(str1 > str2)),
                _ => Err(EvalError::UnsupportedOperation),
            },
            _ => Err(EvalError::MismatchedTypes),
        }
    }

    fn evaluate_unary(&mut self, op: UnaryOperator, exp: Expression) -> EvalResult<Value> {
        let val = self.evaluate_expression(exp)?;
        match val {
            Value::Bool(boolean) => match op {
                UnaryOperator::Not => Ok(Value::Bool(!boolean)),
            },
            _ => Err(EvalError::MismatchedTypes),
        }
    }

    fn find_assigned_variable(&mut self, id: &String) -> EvalResult<GlobalVar> {
        match self.global_scope.get(id) {
            Some((type_def, val)) => Ok((type_def.clone(), val.clone())),
            None => Err(EvalError::VariableNotInitialized(id.clone())),
        }
    }

    fn add_new_variable(&mut self, id: String, var: GlobalVar) -> EvalResult<()> {
        let (type_def, val) = var;
        match self.global_scope.get(&id) {
            Some(_) => Err(EvalError::VariableAlreadyInitialized(id)),
            None => {
                self.global_scope.insert(id, (type_def, val));
                Ok(())
            }
        }
    }

    fn get_input(&mut self) -> EvalResult<String> {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => Ok(input),
            Err(err) => Err(EvalError::IOError(err.to_string())),
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
