#![allow(unused_imports)]
#![allow(unused_variables)]
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;
use util::tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::fmt::Debug;
    use std::ops::{Add, Div, Mul, Sub};
    use std::str::FromStr;
    use util::tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub fn parse_Expr<
        'input,
        F,
    >(
        input: &'input str,
    ) -> Result<F, __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match try!(__state0(input, None, &mut __tokens, __lookahead)) {
            (_, Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (_, None, __Nonterminal::____Expr(__nt)) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<F> {
        Expr(F),
        Factor(F),
        Term(F),
        ____Expr(F),
    }

    // State 0
    //   Expr = (*) Expr "+" Factor [EOF]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [EOF]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [EOF]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) r#"[0-9]+"# [EOF]
    //   Term = (*) r#"[0-9]+"# ["*"]
    //   Term = (*) r#"[0-9]+"# ["+"]
    //   Term = (*) r#"[0-9]+"# ["-"]
    //   Term = (*) r#"[0-9]+"# ["/"]
    //   __Expr = (*) Expr [EOF]
    //
    //   "(" -> Shift(S4)
    //   r#"[0-9]+"# -> Shift(S5)
    //
    //   Expr -> S1
    //   Factor -> S2
    //   Term -> S3
    pub fn __state0<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym0));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym0 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
    }

    // State 1
    //   Expr = Expr (*) "+" Factor [EOF]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [EOF]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   __Expr = Expr (*) [EOF]
    //
    //   EOF -> Reduce(__Expr = Expr => Call(ActionFn(0));)
    //   "+" -> Shift(S6)
    //   "-" -> Shift(S7)
    //
    pub fn __state1<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state6(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state7(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action0(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::____Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 2
    //   Expr = Factor (*) [EOF]
    //   Expr = Factor (*) ["+"]
    //   Expr = Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   EOF -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "*" -> Shift(S8)
    //   "+" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "-" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "/" -> Shift(S9)
    //
    pub fn __state2<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            None |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 3
    //   Factor = Term (*) [EOF]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   EOF -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "*" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "+" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "-" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "/" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //
    pub fn __state3<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 4
    //   Expr = (*) Expr "+" Factor [")"]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [")"]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [")"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = "(" (*) Expr ")" [EOF]
    //   Term = "(" (*) Expr ")" ["*"]
    //   Term = "(" (*) Expr ")" ["+"]
    //   Term = "(" (*) Expr ")" ["-"]
    //   Term = "(" (*) Expr ")" ["/"]
    //   Term = (*) r#"[0-9]+"# [")"]
    //   Term = (*) r#"[0-9]+"# ["*"]
    //   Term = (*) r#"[0-9]+"# ["+"]
    //   Term = (*) r#"[0-9]+"# ["-"]
    //   Term = (*) r#"[0-9]+"# ["/"]
    //
    //   "(" -> Shift(S13)
    //   r#"[0-9]+"# -> Shift(S14)
    //
    //   Expr -> S10
    //   Factor -> S11
    //   Term -> S12
    pub fn __state4<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 5
    //   Term = r#"[0-9]+"# (*) [EOF]
    //   Term = r#"[0-9]+"# (*) ["*"]
    //   Term = r#"[0-9]+"# (*) ["+"]
    //   Term = r#"[0-9]+"# (*) ["-"]
    //   Term = r#"[0-9]+"# (*) ["/"]
    //
    //   EOF -> Reduce(Term = r#"[0-9]+"# => Call(ActionFn(7));)
    //   "*" -> Reduce(Term = r#"[0-9]+"# => Call(ActionFn(7));)
    //   "+" -> Reduce(Term = r#"[0-9]+"# => Call(ActionFn(7));)
    //   "-" -> Reduce(Term = r#"[0-9]+"# => Call(ActionFn(7));)
    //   "/" -> Reduce(Term = r#"[0-9]+"# => Call(ActionFn(7));)
    //
    pub fn __state5<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 6
    //   Expr = Expr "+" (*) Factor [EOF]
    //   Expr = Expr "+" (*) Factor ["+"]
    //   Expr = Expr "+" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) r#"[0-9]+"# [EOF]
    //   Term = (*) r#"[0-9]+"# ["*"]
    //   Term = (*) r#"[0-9]+"# ["+"]
    //   Term = (*) r#"[0-9]+"# ["-"]
    //   Term = (*) r#"[0-9]+"# ["/"]
    //
    //   "(" -> Shift(S4)
    //   r#"[0-9]+"# -> Shift(S5)
    //
    //   Factor -> S15
    //   Term -> S3
    pub fn __state6<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state15(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 7
    //   Expr = Expr "-" (*) Factor [EOF]
    //   Expr = Expr "-" (*) Factor ["+"]
    //   Expr = Expr "-" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [EOF]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [EOF]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [EOF]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) r#"[0-9]+"# [EOF]
    //   Term = (*) r#"[0-9]+"# ["*"]
    //   Term = (*) r#"[0-9]+"# ["+"]
    //   Term = (*) r#"[0-9]+"# ["-"]
    //   Term = (*) r#"[0-9]+"# ["/"]
    //
    //   "(" -> Shift(S4)
    //   r#"[0-9]+"# -> Shift(S5)
    //
    //   Factor -> S16
    //   Term -> S3
    pub fn __state7<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state16(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state3(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 8
    //   Factor = Factor "*" (*) Term [EOF]
    //   Factor = Factor "*" (*) Term ["*"]
    //   Factor = Factor "*" (*) Term ["+"]
    //   Factor = Factor "*" (*) Term ["-"]
    //   Factor = Factor "*" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) r#"[0-9]+"# [EOF]
    //   Term = (*) r#"[0-9]+"# ["*"]
    //   Term = (*) r#"[0-9]+"# ["+"]
    //   Term = (*) r#"[0-9]+"# ["-"]
    //   Term = (*) r#"[0-9]+"# ["/"]
    //
    //   "(" -> Shift(S4)
    //   r#"[0-9]+"# -> Shift(S5)
    //
    //   Term -> S17
    pub fn __state8<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state17(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 9
    //   Factor = Factor "/" (*) Term [EOF]
    //   Factor = Factor "/" (*) Term ["*"]
    //   Factor = Factor "/" (*) Term ["+"]
    //   Factor = Factor "/" (*) Term ["-"]
    //   Factor = Factor "/" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [EOF]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) r#"[0-9]+"# [EOF]
    //   Term = (*) r#"[0-9]+"# ["*"]
    //   Term = (*) r#"[0-9]+"# ["+"]
    //   Term = (*) r#"[0-9]+"# ["-"]
    //   Term = (*) r#"[0-9]+"# ["/"]
    //
    //   "(" -> Shift(S4)
    //   r#"[0-9]+"# -> Shift(S5)
    //
    //   Term -> S18
    pub fn __state9<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state4(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state5(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state18(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 10
    //   Expr = Expr (*) "+" Factor [")"]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [")"]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   Term = "(" Expr (*) ")" [EOF]
    //   Term = "(" Expr (*) ")" ["*"]
    //   Term = "(" Expr (*) ")" ["+"]
    //   Term = "(" Expr (*) ")" ["-"]
    //   Term = "(" Expr (*) ")" ["/"]
    //
    //   ")" -> Shift(S19)
    //   "+" -> Shift(S20)
    //   "-" -> Shift(S21)
    //
    pub fn __state10<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<F>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state19(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 11
    //   Expr = Factor (*) [")"]
    //   Expr = Factor (*) ["+"]
    //   Expr = Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   ")" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "*" -> Shift(S22)
    //   "+" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "-" -> Reduce(Expr = Factor => Call(ActionFn(3));)
    //   "/" -> Shift(S23)
    //
    pub fn __state11<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state23(input, __lookbehind, __tokens, __sym0, __sym1));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action3(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 12
    //   Factor = Term (*) [")"]
    //   Factor = Term (*) ["*"]
    //   Factor = Term (*) ["+"]
    //   Factor = Term (*) ["-"]
    //   Factor = Term (*) ["/"]
    //
    //   ")" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "*" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "+" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "-" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //   "/" -> Reduce(Factor = Term => Call(ActionFn(6));)
    //
    pub fn __state12<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action6(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 13
    //   Expr = (*) Expr "+" Factor [")"]
    //   Expr = (*) Expr "+" Factor ["+"]
    //   Expr = (*) Expr "+" Factor ["-"]
    //   Expr = (*) Expr "-" Factor [")"]
    //   Expr = (*) Expr "-" Factor ["+"]
    //   Expr = (*) Expr "-" Factor ["-"]
    //   Expr = (*) Factor [")"]
    //   Expr = (*) Factor ["+"]
    //   Expr = (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = "(" (*) Expr ")" [")"]
    //   Term = "(" (*) Expr ")" ["*"]
    //   Term = "(" (*) Expr ")" ["+"]
    //   Term = "(" (*) Expr ")" ["-"]
    //   Term = "(" (*) Expr ")" ["/"]
    //   Term = (*) r#"[0-9]+"# [")"]
    //   Term = (*) r#"[0-9]+"# ["*"]
    //   Term = (*) r#"[0-9]+"# ["+"]
    //   Term = (*) r#"[0-9]+"# ["-"]
    //   Term = (*) r#"[0-9]+"# ["/"]
    //
    //   "(" -> Shift(S13)
    //   r#"[0-9]+"# -> Shift(S14)
    //
    //   Expr -> S24
    //   Factor -> S11
    //   Term -> S12
    pub fn __state13<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym1));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym1 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Expr(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state24(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Factor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 14
    //   Term = r#"[0-9]+"# (*) [")"]
    //   Term = r#"[0-9]+"# (*) ["*"]
    //   Term = r#"[0-9]+"# (*) ["+"]
    //   Term = r#"[0-9]+"# (*) ["-"]
    //   Term = r#"[0-9]+"# (*) ["/"]
    //
    //   ")" -> Reduce(Term = r#"[0-9]+"# => Call(ActionFn(7));)
    //   "*" -> Reduce(Term = r#"[0-9]+"# => Call(ActionFn(7));)
    //   "+" -> Reduce(Term = r#"[0-9]+"# => Call(ActionFn(7));)
    //   "-" -> Reduce(Term = r#"[0-9]+"# => Call(ActionFn(7));)
    //   "/" -> Reduce(Term = r#"[0-9]+"# => Call(ActionFn(7));)
    //
    pub fn __state14<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __nt = super::__action7(input, __sym0);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 15
    //   Expr = Expr "+" Factor (*) [EOF]
    //   Expr = Expr "+" Factor (*) ["+"]
    //   Expr = Expr "+" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   EOF -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "*" -> Shift(S8)
    //   "+" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "-" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "/" -> Shift(S9)
    //
    pub fn __state15<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<F>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            None |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 16
    //   Expr = Expr "-" Factor (*) [EOF]
    //   Expr = Expr "-" Factor (*) ["+"]
    //   Expr = Expr "-" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [EOF]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [EOF]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   EOF -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "*" -> Shift(S8)
    //   "+" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "-" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "/" -> Shift(S9)
    //
    pub fn __state16<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<F>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state8(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state9(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            None |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 17
    //   Factor = Factor "*" Term (*) [EOF]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   EOF -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "*" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "+" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "-" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "/" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //
    pub fn __state17<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<F>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 18
    //   Factor = Factor "/" Term (*) [EOF]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   EOF -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "*" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "+" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "-" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "/" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //
    pub fn __state18<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<F>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 19
    //   Term = "(" Expr ")" (*) [EOF]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   EOF -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "*" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "+" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "-" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "/" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //
    pub fn __state19<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<F>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            None |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 20
    //   Expr = Expr "+" (*) Factor [")"]
    //   Expr = Expr "+" (*) Factor ["+"]
    //   Expr = Expr "+" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) r#"[0-9]+"# [")"]
    //   Term = (*) r#"[0-9]+"# ["*"]
    //   Term = (*) r#"[0-9]+"# ["+"]
    //   Term = (*) r#"[0-9]+"# ["-"]
    //   Term = (*) r#"[0-9]+"# ["/"]
    //
    //   "(" -> Shift(S13)
    //   r#"[0-9]+"# -> Shift(S14)
    //
    //   Factor -> S25
    //   Term -> S12
    pub fn __state20<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state25(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 21
    //   Expr = Expr "-" (*) Factor [")"]
    //   Expr = Expr "-" (*) Factor ["+"]
    //   Expr = Expr "-" (*) Factor ["-"]
    //   Factor = (*) Factor "*" Term [")"]
    //   Factor = (*) Factor "*" Term ["*"]
    //   Factor = (*) Factor "*" Term ["+"]
    //   Factor = (*) Factor "*" Term ["-"]
    //   Factor = (*) Factor "*" Term ["/"]
    //   Factor = (*) Factor "/" Term [")"]
    //   Factor = (*) Factor "/" Term ["*"]
    //   Factor = (*) Factor "/" Term ["+"]
    //   Factor = (*) Factor "/" Term ["-"]
    //   Factor = (*) Factor "/" Term ["/"]
    //   Factor = (*) Term [")"]
    //   Factor = (*) Term ["*"]
    //   Factor = (*) Term ["+"]
    //   Factor = (*) Term ["-"]
    //   Factor = (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) r#"[0-9]+"# [")"]
    //   Term = (*) r#"[0-9]+"# ["*"]
    //   Term = (*) r#"[0-9]+"# ["+"]
    //   Term = (*) r#"[0-9]+"# ["-"]
    //   Term = (*) r#"[0-9]+"# ["/"]
    //
    //   "(" -> Shift(S13)
    //   r#"[0-9]+"# -> Shift(S14)
    //
    //   Factor -> S26
    //   Term -> S12
    pub fn __state21<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Factor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state26(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(input, __lookbehind, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 22
    //   Factor = Factor "*" (*) Term [")"]
    //   Factor = Factor "*" (*) Term ["*"]
    //   Factor = Factor "*" (*) Term ["+"]
    //   Factor = Factor "*" (*) Term ["-"]
    //   Factor = Factor "*" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) r#"[0-9]+"# [")"]
    //   Term = (*) r#"[0-9]+"# ["*"]
    //   Term = (*) r#"[0-9]+"# ["+"]
    //   Term = (*) r#"[0-9]+"# ["-"]
    //   Term = (*) r#"[0-9]+"# ["/"]
    //
    //   "(" -> Shift(S13)
    //   r#"[0-9]+"# -> Shift(S14)
    //
    //   Term -> S27
    pub fn __state22<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state27(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 23
    //   Factor = Factor "/" (*) Term [")"]
    //   Factor = Factor "/" (*) Term ["*"]
    //   Factor = Factor "/" (*) Term ["+"]
    //   Factor = Factor "/" (*) Term ["-"]
    //   Factor = Factor "/" (*) Term ["/"]
    //   Term = (*) "(" Expr ")" [")"]
    //   Term = (*) "(" Expr ")" ["*"]
    //   Term = (*) "(" Expr ")" ["+"]
    //   Term = (*) "(" Expr ")" ["-"]
    //   Term = (*) "(" Expr ")" ["/"]
    //   Term = (*) r#"[0-9]+"# [")"]
    //   Term = (*) r#"[0-9]+"# ["*"]
    //   Term = (*) r#"[0-9]+"# ["+"]
    //   Term = (*) r#"[0-9]+"# ["-"]
    //   Term = (*) r#"[0-9]+"# ["/"]
    //
    //   "(" -> Shift(S13)
    //   r#"[0-9]+"# -> Shift(S14)
    //
    //   Term -> S28
    pub fn __state23<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (0, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state13(input, __lookbehind, __tokens, __sym2));
            }
            Some((_, (6, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state14(input, __lookbehind, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookbehind, __lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Term(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state28(input, __lookbehind, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookbehind, __lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    // State 24
    //   Expr = Expr (*) "+" Factor [")"]
    //   Expr = Expr (*) "+" Factor ["+"]
    //   Expr = Expr (*) "+" Factor ["-"]
    //   Expr = Expr (*) "-" Factor [")"]
    //   Expr = Expr (*) "-" Factor ["+"]
    //   Expr = Expr (*) "-" Factor ["-"]
    //   Term = "(" Expr (*) ")" [")"]
    //   Term = "(" Expr (*) ")" ["*"]
    //   Term = "(" Expr (*) ")" ["+"]
    //   Term = "(" Expr (*) ")" ["-"]
    //   Term = "(" Expr (*) ")" ["/"]
    //
    //   ")" -> Shift(S29)
    //   "+" -> Shift(S20)
    //   "-" -> Shift(S21)
    //
    pub fn __state24<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<F>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (1, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state29(input, __lookbehind, __tokens, __sym0, __sym1, __sym2));
            }
            Some((_, (3, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state20(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            Some((_, (4, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym2 = &mut Some((__tok0));
                __result = try!(__state21(input, __lookbehind, __tokens, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 25
    //   Expr = Expr "+" Factor (*) [")"]
    //   Expr = Expr "+" Factor (*) ["+"]
    //   Expr = Expr "+" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   ")" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "*" -> Shift(S22)
    //   "+" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "-" -> Reduce(Expr = Expr, "+", Factor => Call(ActionFn(2));)
    //   "/" -> Shift(S23)
    //
    pub fn __state25<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<F>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state23(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action2(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 26
    //   Expr = Expr "-" Factor (*) [")"]
    //   Expr = Expr "-" Factor (*) ["+"]
    //   Expr = Expr "-" Factor (*) ["-"]
    //   Factor = Factor (*) "*" Term [")"]
    //   Factor = Factor (*) "*" Term ["*"]
    //   Factor = Factor (*) "*" Term ["+"]
    //   Factor = Factor (*) "*" Term ["-"]
    //   Factor = Factor (*) "*" Term ["/"]
    //   Factor = Factor (*) "/" Term [")"]
    //   Factor = Factor (*) "/" Term ["*"]
    //   Factor = Factor (*) "/" Term ["+"]
    //   Factor = Factor (*) "/" Term ["-"]
    //   Factor = Factor (*) "/" Term ["/"]
    //
    //   ")" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "*" -> Shift(S22)
    //   "+" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "-" -> Reduce(Expr = Expr, "-", Factor => Call(ActionFn(1));)
    //   "/" -> Shift(S23)
    //
    pub fn __state26<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<F>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (2, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state22(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (5, __tok0), __loc)) => {
                let mut __lookbehind = Some(__loc);
                let mut __sym3 = &mut Some((__tok0));
                __result = try!(__state23(input, __lookbehind, __tokens, __sym2, __sym3));
            }
            Some((_, (1, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action1(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Expr(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    // State 27
    //   Factor = Factor "*" Term (*) [")"]
    //   Factor = Factor "*" Term (*) ["*"]
    //   Factor = Factor "*" Term (*) ["+"]
    //   Factor = Factor "*" Term (*) ["-"]
    //   Factor = Factor "*" Term (*) ["/"]
    //
    //   ")" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "*" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "+" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "-" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //   "/" -> Reduce(Factor = Factor, "*", Term => Call(ActionFn(4));)
    //
    pub fn __state27<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<F>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action4(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 28
    //   Factor = Factor "/" Term (*) [")"]
    //   Factor = Factor "/" Term (*) ["*"]
    //   Factor = Factor "/" Term (*) ["+"]
    //   Factor = Factor "/" Term (*) ["-"]
    //   Factor = Factor "/" Term (*) ["/"]
    //
    //   ")" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "*" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "+" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "-" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //   "/" -> Reduce(Factor = Factor, "/", Term => Call(ActionFn(5));)
    //
    pub fn __state28<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, (usize, &'input str), usize)>,
        __sym0: &mut Option<F>,
        __sym1: &mut Option<&'input str>,
        __sym2: &mut Option<F>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action5(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Factor(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    // State 29
    //   Term = "(" Expr ")" (*) [")"]
    //   Term = "(" Expr ")" (*) ["*"]
    //   Term = "(" Expr ")" (*) ["+"]
    //   Term = "(" Expr ")" (*) ["-"]
    //   Term = "(" Expr ")" (*) ["/"]
    //
    //   ")" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "*" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "+" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "-" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //   "/" -> Reduce(Term = "(", Expr, ")" => Call(ActionFn(8));)
    //
    pub fn __state29<
        'input,
        F,
        __TOKENS: Iterator<Item=Result<(usize, (usize, &'input str), usize),__ParseError<usize,(usize, &'input str),()>>>,
    >(
        input: &'input str,
        __lookbehind: Option<usize>,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<&'input str>,
        __sym1: &mut Option<F>,
        __sym2: &mut Option<&'input str>,
    ) -> Result<(Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>), __ParseError<usize,(usize, &'input str),()>> where
      F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
    {
        let mut __result: (Option<usize>, Option<(usize, (usize, &'input str), usize)>, __Nonterminal<F>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(e),
        };
        match __lookahead {
            Some((_, (1, _), _)) |
            Some((_, (2, _), _)) |
            Some((_, (3, _), _)) |
            Some((_, (4, _), _)) |
            Some((_, (5, _), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __nt = super::__action8(input, __sym0, __sym1, __sym2);
                return Ok((__lookbehind, __lookahead, __Nonterminal::Term(__nt)));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Expr::parse_Expr;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
    }

    fn __tokenize(text: &str) -> Option<(usize, usize)> {
        let mut __chars = text.char_indices();
        let mut __current_match: Option<(usize, usize)> = None;
        let mut __current_state: usize = 0;
        loop {
            match __current_state {
                0 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '(' => {
                            __current_match = Some((0, __index + 1));
                            __current_state = 1;
                            continue;
                        }
                        ')' => {
                            __current_match = Some((1, __index + 1));
                            __current_state = 2;
                            continue;
                        }
                        '*' => {
                            __current_match = Some((2, __index + 1));
                            __current_state = 3;
                            continue;
                        }
                        '+' => {
                            __current_match = Some((3, __index + 1));
                            __current_state = 4;
                            continue;
                        }
                        '-' => {
                            __current_match = Some((4, __index + 1));
                            __current_state = 5;
                            continue;
                        }
                        '/' => {
                            __current_match = Some((5, __index + 1));
                            __current_state = 6;
                            continue;
                        }
                        '0' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                1 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                2 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                3 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                4 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                5 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                6 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                7 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        '0' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '1' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '2' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '3' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '4' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '5' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '6' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '7' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '8' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        '9' => {
                            __current_match = Some((6, __index + 1));
                            __current_state = 7;
                            continue;
                        }
                        _ => {
                            return __current_match;
                        }
                    }
                }
                8 => {
                    let (__index, __ch) = match __chars.next() { Some(p) => p, None => return __current_match };
                    match __ch {
                        _ => {
                            return __current_match;
                        }
                    }
                }
                _ => { panic!("invalid state {}", __current_state); }
            }
        }
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            __Matcher { text: s, consumed: 0 }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                match __tokenize(__text) {
                    Some((__index, __length)) => {
                        let __result = &__text[..__length];
                        let __remaining = &__text[__length..];
                        let __end_offset = __start_offset + __length;
                        self.text = __remaining;
                        self.consumed = __end_offset;
                        Some(Ok((__start_offset, (__index, __result), __end_offset)))
                    }
                    None => {
                        Some(Err(__ParseError::InvalidToken { location: __start_offset }))
                    }
                }
            }
        }
    }
}

pub fn __action0<
    'input,
    F,
>(
    input: &'input str,
    __0: F,
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
{
    (__0)
}

pub fn __action1<
    'input,
    F,
>(
    input: &'input str,
    l: F,
    _: &'input str,
    r: F,
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
{
    l - r
}

pub fn __action2<
    'input,
    F,
>(
    input: &'input str,
    l: F,
    _: &'input str,
    r: F,
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
{
    l + r
}

pub fn __action3<
    'input,
    F,
>(
    input: &'input str,
    __0: F,
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
{
    (__0)
}

pub fn __action4<
    'input,
    F,
>(
    input: &'input str,
    l: F,
    _: &'input str,
    r: F,
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
{
    l * r
}

pub fn __action5<
    'input,
    F,
>(
    input: &'input str,
    l: F,
    _: &'input str,
    r: F,
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
{
    l / r
}

pub fn __action6<
    'input,
    F,
>(
    input: &'input str,
    __0: F,
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
{
    (__0)
}

pub fn __action7<
    'input,
    F,
>(
    input: &'input str,
    n: &'input str,
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
{
    match n.parse() { Ok(v) => v, Err(_) => panic!("can't parse") }
}

pub fn __action8<
    'input,
    F,
>(
    input: &'input str,
    _: &'input str,
    __0: F,
    _: &'input str,
) -> F where
  F: Debug + FromStr + Sub<Output=F> + Add<Output=F> + Mul<Output=F> + Div<Output=F>,
{
    (__0)
}

pub trait __ToTriple<'input, F, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, F, > __ToTriple<'input, F, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, F, > __ToTriple<'input, F, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
