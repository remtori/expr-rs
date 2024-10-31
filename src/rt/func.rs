use std::{marker::PhantomData, u32};

use super::Value;

pub struct ExternalFunction<In, F> {
    func: F,
    _marker: PhantomData<In>,
}

pub trait AnyExternalFunction {
    fn arg_count(&self) -> u32;
    fn call(&mut self, args: &[Value]) -> Value;
}

impl<F: FnMut(&[Value]) -> Value> AnyExternalFunction for ExternalFunction<&[Value], F> {
    fn arg_count(&self) -> u32 {
        u32::MAX
    }

    fn call(&mut self, args: &[Value]) -> Value {
        (self.func)(args)
    }
}

macro_rules! impl_ext_func {
    ($($params:ident),*) => {
        #[allow(unused_variables)]
        #[allow(non_snake_case)]
        impl<F: FnMut($($params),*) -> Value, $($params: From<Value> + 'static),*> AnyExternalFunction for ExternalFunction<($($params ,)*), F> {
            #[inline(always)]
            fn arg_count(&self) -> u32 {
                let count = 0;
                $(
                    let $params = count + 1;
                    let count = $params;
                )*

                count
            }

            fn call(&mut self, args: &[Value]) -> Value {
                $(
                    let $params = args[0].into();
                    let args = &args[1..];
                )*

                (self.func)($($params),*)
            }
        }
    };
}

pub trait IntoExtFunc<In> {
    type Func: AnyExternalFunction;

    fn into_ext(self) -> Self::Func;
}

impl<'a, F: FnMut(&[Value]) -> Value> IntoExtFunc<&'a [Value]> for F {
    type Func = ExternalFunction<&'a [Value], F>;

    fn into_ext(self) -> Self::Func {
        ExternalFunction {
            func: self,
            _marker: PhantomData,
        }
    }
}

macro_rules! impl_into_ext_func {
    ($($params:ident),*) => {
        #[allow(unused_parens)]
        impl<F: FnMut($($params),*) -> Value, $($params: From<Value> + 'static),*> IntoExtFunc<($($params),*)> for F {
            type Func = ExternalFunction<($($params ,)*), F>;

            fn into_ext(self) -> Self::Func {
                ExternalFunction {
                    func: self,
                    _marker: PhantomData,
                }
            }
        }
    };
}

macro_rules! call_12_times {
    ($target:ident) => {
        $target!();
        $target!(T1);
        $target!(T1, T2);
        $target!(T1, T2, T3);
        $target!(T1, T2, T3, T4);
        $target!(T1, T2, T3, T4, T5);
        $target!(T1, T2, T3, T4, T5, T6);
        $target!(T1, T2, T3, T4, T5, T6, T7);
        $target!(T1, T2, T3, T4, T5, T6, T7, T8);
        $target!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
        $target!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
        $target!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
        $target!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    };
}

call_12_times!(impl_ext_func);
call_12_times!(impl_into_ext_func);
