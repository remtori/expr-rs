use std::borrow::Cow;

use super::{AnyExternalFunction, IntoExtFunc, Value};

mod builtin;

pub struct Registry {
    vars: Vec<(Cow<'static, [u8]>, Value)>,
    fns: Vec<(Cow<'static, [u8]>, Box<dyn AnyExternalFunction>)>,
}

impl Default for Registry {
    fn default() -> Self {
        let mut registry = Self::empty();
        registry
            .add_var(b"PI", std::f64::consts::PI)
            .add_fn(b"pow", builtin::pow)
            .add_fn(b"sin", builtin::sin)
            .add_fn(b"cos", builtin::cos)
            .add_fn(b"tan", builtin::tan)
            .add_fn(b"asin", builtin::asin)
            .add_fn(b"acos", builtin::acos)
            .add_fn(b"atan", builtin::atan)
            .add_fn(b"sinh", builtin::sinh)
            .add_fn(b"cosh", builtin::cosh)
            .add_fn(b"tanh", builtin::tanh)
            .add_fn(b"asinh", builtin::asinh)
            .add_fn(b"acosh", builtin::acosh)
            .add_fn(b"atanh", builtin::atanh)
            .add_fn(b"exp", builtin::exp)
            .add_fn(b"ln", builtin::ln)
            .add_fn(b"log10", builtin::log10)
            .add_fn(b"log2", builtin::log2)
            .add_fn(b"sqrt", builtin::sqrt)
            .add_fn(b"cbrt", builtin::cbrt)
            .add_fn(b"max", builtin::max)
            .add_fn(b"min", builtin::min)
            .add_fn(b"sum", builtin::sum);

        registry
    }
}

impl Registry {
    pub fn empty() -> Self {
        Self {
            vars: Vec::new(),
            fns: Vec::new(),
        }
    }

    pub fn add_var(
        &mut self,
        name: impl Into<Cow<'static, [u8]>>,
        value: impl Into<Value>,
    ) -> &mut Self {
        self.vars.push((name.into(), value.into()));
        self
    }

    pub fn add_fn<In: 'static, F: IntoExtFunc<In> + 'static>(
        &mut self,
        name: impl Into<Cow<'static, [u8]>>,
        func: F,
    ) -> &mut Self {
        self.fns.push((name.into(), Box::new(func.into_ext())));
        self
    }

    pub(crate) fn var_ident(&self, ident: &[u8]) -> Option<u32> {
        self.vars
            .iter()
            .enumerate()
            .find(|(_, var)| var.0.as_ref() == ident.as_ref())
            .map(|(i, _)| i as u32)
    }

    pub(crate) fn fn_ident(&self, ident: &[u8]) -> Option<(u32, u32)> {
        self.fns
            .iter()
            .enumerate()
            .find(|(_, var)| var.0.as_ref() == ident.as_ref())
            .map(|(i, (_, func))| (i as u32, func.arg_count()))
    }

    pub(crate) fn var(&self, ident: u32) -> Value {
        self.vars[ident as usize].1
    }

    pub(crate) fn call(&mut self, ident: u32, args: &[Value]) -> Value {
        let ident = ident as usize;
        let func = self.fns[ident].1.as_mut();
        func.call(args)
    }
}
