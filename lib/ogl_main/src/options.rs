use quote::quote;
use regex::bytes::Regex;
use syn::{AttributeArgs, ItemFn, Lit, Meta, MetaNameValue, NestedMeta};

pub fn parse_options(attr: AttributeArgs, function: &mut ItemFn) {
    let mut title: String = "OpenGL Application".into();
    let mut width: i32 = 900;
    let mut height: i32 = 700;
    let mut bg_color: Vec<f32> = vec![0.0, 0.0, 0.0, 1.0];

    attr.iter().for_each(|a: &NestedMeta| match a {
        NestedMeta::Meta(Meta::NameValue(MetaNameValue {
            path,
            lit,
            eq_token: _,
        })) => match path.segments.first().unwrap().ident.to_string().as_str() {
            "title" => {
                title = match lit {
                    Lit::Str(value) => value.value(),
                    _ => panic!("Expected string literal for title"),
                };
            }

            "window" => {
                match lit {
                    Lit::Str(value) => {
                        let dimensions = value.value();

                        assert!(
                            Regex::new(r"\d+x\d+")
                                .unwrap()
                                .is_match(dimensions.as_ref()),
                            "Expected format: WIDTHxHEIGHT"
                        );

                        let mut v = dimensions.split('x');
                        width = v.next().unwrap().parse::<i32>().unwrap();
                        height = v.next().unwrap().parse::<i32>().unwrap();
                    }
                    _ => panic!("Expected string for window options"),
                };
            }

            "bg_color" => match lit {
                Lit::Str(value) => {
                    let color = value.value();

                    assert!(Regex::new(r"(\d+(\.\d+)? ){3}(\d+(\.\d+)?)")
                        .unwrap()
                        .is_match(color.as_ref()));

                    let mut v = color.split(' ');
                    let r = v.next().unwrap().parse::<f32>().unwrap();
                    let g = v.next().unwrap().parse::<f32>().unwrap();
                    let b = v.next().unwrap().parse::<f32>().unwrap();
                    let a = v.next().unwrap().parse::<f32>().unwrap();
                    bg_color = vec![r, g, b, a];
                }
                _ => panic!("Expected string for bg_color options"),
            },

            attr => panic!("Unknown attribute: {}", attr),
        },
        _ => panic!("Could not undertand attribute"),
    });

    let bg_color = quote! {
        [#(#bg_color),*]
    };

    function.block.stmts.push(syn::parse_quote!(let title = #title;));
    function.block.stmts.push(syn::parse_quote!(let window_width = #width;));
    function.block.stmts.push(syn::parse_quote!(let window_height = #height;));
    function.block.stmts.push(syn::parse_quote!(let bg_color = #bg_color;));
}
