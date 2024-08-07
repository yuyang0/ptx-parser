pub(super) mod body;
pub(crate) mod parse;

#[derive(Debug, PartialEq)]
pub struct Function<'a> {
    pub signature: FunctionSignature<'a>,
    pub body: Option<body::FunctionBody<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionSignature<'a> {
    pub visible: bool,
    pub entry: bool,
    pub return_value: Option<ReturnValue<'a>>,
    pub name: &'a str,
    pub parameters: Option<Parameters<'a>>,
}

#[derive(Debug, PartialEq)]
pub(super) struct ReturnValue<'a> {
    raw_string: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct Parameter<'a> {
    pub name: &'a str,
    pub ty: &'a str,
    pub size: usize,
    raw_string: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct Parameters<'a> {
    pub params: Vec<Parameter<'a>>,
    raw_string: &'a str,
}

impl Parameters<'_> {
    fn parse(&mut self) {
        for p_raw_str in self.raw_string.split('\n') {
            let p_raw_str = p_raw_str.trim();
            let p_raw_str = p_raw_str.trim_end_matches(',');
            let parts = p_raw_str.split(' ').collect::<Vec<_>>();
            if parts.len() != 3 {
                continue;
            }
            let ty = parts[1];
            let name = parts[2];
            let size = match ty {
                ".s8" => 1,
                ".s16" => 2,
                ".s32" => 4,
                ".s64" => 8,
                ".u8" => 1,
                ".u16" => 2,
                ".u32" => 4,
                ".u64" => 8,
                ".f16" => 2,
                ".f16x2" => 4,
                ".f32" => 4,
                ".f64" => 8,
                ".b8" => 1,
                ".b16" => 2,
                ".b32" => 4,
                ".b64" => 8,
                ".b128" => 16,
                //TODO: more types
                _ => panic!("unknown type: {}", ty),
            };
            self.params.push(Parameter {
                name: name,
                ty: ty,
                size: size,
                raw_string: p_raw_str,
            });
        }
    }
}
#[cfg(test)]
mod test_parse_function_signature {

    use crate::parser::function::{parse::parse_function_signature, Parameter};

    use super::{FunctionSignature, Parameters, ReturnValue};

    #[test]
    fn visible_entry_name() {
        let input = ".visible .entry _Z6kernelPiS_i";
        let signature = parse_function_signature(input);
        assert_eq!(
            signature,
            Ok((
                "",
                FunctionSignature {
                    visible: true,
                    entry: true,
                    return_value: None,
                    name: "_Z6kernelPiS_i",
                    parameters: None,
                }
            ))
        )
    }

    #[test]
    fn func_no_return_no_parameters() {
        let input = ".func _Z6kernelPiS_i";
        let signature = parse_function_signature(input);
        assert_eq!(
            signature,
            Ok((
                "",
                FunctionSignature {
                    visible: false,
                    entry: false,
                    return_value: None,
                    name: "_Z6kernelPiS_i",
                    parameters: None,
                }
            ))
        )
    }

    #[test]
    fn func_no_return_trivial_parameters() {
        let input = ".func _ZN4core9panicking(hi)";
        let signature = parse_function_signature(input);
        assert_eq!(
            signature,
            Ok((
                "",
                FunctionSignature {
                    visible: false,
                    entry: false,
                    return_value: None,
                    name: "_ZN4core9panicking",
                    parameters: Some(Parameters {
                        raw_string: "hi",
                        params: vec![],
                    }),
                }
            ))
        )
    }

    #[test]
    fn func_no_return_some_parameters() {
        let input = ".func _ZN4core9panicking
(
	.param .b64 _ZN4core9panicking_param_0,
	.param .b64 _ZN4core9panicking_param_1,
	.param .b64 _ZN4core9panicking_param_2
)";
        let signature = parse_function_signature(input);
        assert_eq!(
            signature,
            Ok((
                "",
                FunctionSignature {
                    visible: false,
                    entry: false,
                    return_value: None,
                    name: "_ZN4core9panicking",
                    parameters: Some(Parameters {
                        raw_string: "
	.param .b64 _ZN4core9panicking_param_0,
	.param .b64 _ZN4core9panicking_param_1,
	.param .b64 _ZN4core9panicking_param_2
",
                        params: vec![
                            Parameter {
                                name: "_ZN4core9panicking_param_0",
                                ty: ".b64",
                                size: 8,
                                raw_string: ".param .b64 _ZN4core9panicking_param_0",
                            },
                            Parameter {
                                name: "_ZN4core9panicking_param_1",
                                ty: ".b64",
                                size: 8,
                                raw_string: ".param .b64 _ZN4core9panicking_param_1",
                            },
                            Parameter {
                                name: "_ZN4core9panicking_param_2",
                                ty: ".b64",
                                size: 8,
                                raw_string: ".param .b64 _ZN4core9panicking_param_2",
                            },
                        ],
                    }),
                }
            ))
        )
    }

    #[test]
    fn func_return_and_parameters() {
        let input = ".func  (.param .b64 func_retval0) _foo(
	.param .b64 _foo_param_0,
	.param .b64 _foo_param_1
)";
        let signature = parse_function_signature(input);
        assert_eq!(
            signature,
            Ok((
                "",
                FunctionSignature {
                    visible: false,
                    entry: false,
                    return_value: Some(ReturnValue {
                        raw_string: ".param .b64 func_retval0"
                    }),
                    name: "_foo",
                    parameters: Some(Parameters {
                        raw_string: "
	.param .b64 _foo_param_0,
	.param .b64 _foo_param_1
",
                        params: vec![
                            Parameter {
                                name: "_foo_param_0",
                                ty: ".b64",
                                size: 8,
                                raw_string: ".param .b64 _foo_param_0",
                            },
                            Parameter {
                                name: "_foo_param_1",
                                ty: ".b64",
                                size: 8,
                                raw_string: ".param .b64 _foo_param_1",
                            },
                        ]
                    })
                }
            ))
        )
    }
}

#[cfg(test)]
mod test_parse_function_body {

    use crate::parser::function::{body::FunctionBody, parse::parse_function_body};

    #[test]
    fn empty() {
        let input = ";";
        let body = parse_function_body(input);
        assert!(body.is_err())
    }

    #[test]
    fn non_empty() {
        let input = "{.reg .b32 %r<3>}";
        let body = parse_function_body(input);
        assert_eq!(
            body,
            Ok((
                "",
                FunctionBody {
                    body: Some(".reg .b32 %r<3>")
                }
            ))
        )
    }
}

#[cfg(test)]
mod test_parse_function {
    use crate::parser::function::{
        body::FunctionBody, parse::parse_function, Function, FunctionSignature,
    };

    #[test]
    fn no_return_no_parameters_no_body() {
        let input = ".func _Z6kernelPiS_i;";
        let function = parse_function(input);
        assert_eq!(
            function,
            Ok((
                "",
                Function {
                    signature: FunctionSignature {
                        visible: false,
                        entry: false,
                        return_value: None,
                        name: "_Z6kernelPiS_i",
                        parameters: None,
                    },
                    body: None,
                }
            ))
        )
    }

    #[test]
    fn no_return_no_parameters_with_body() {
        let input = ".func _Z6kernelPiS_i { \n foo \n bar }";
        let function = parse_function(input);
        assert_eq!(
            function,
            Ok((
                "",
                Function {
                    signature: FunctionSignature {
                        visible: false,
                        entry: false,
                        return_value: None,
                        name: "_Z6kernelPiS_i",
                        parameters: None,
                    },
                    body: Some(FunctionBody {
                        body: Some(" \n foo \n bar ")
                    }),
                }
            ))
        )
    }
}
