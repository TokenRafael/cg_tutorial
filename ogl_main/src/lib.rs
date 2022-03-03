use proc_macro::TokenStream;

use quote::quote;
use regex::bytes::Regex;
use syn::{Lit, LitStr, Meta, MetaNameValue, NestedMeta};

macro_rules! quote_vec {
    ($($x:stmt;)*) => {
        vec![
            $(
                syn::parse_quote!($x),
            )*
        ]
    };
}

/// Generates the `main` function boilerplate code
/// for the OpenGL applications.
/// The `main` function is responsible for initializing the OpenGL context
/// and setting up the window.
///
#[proc_macro_attribute]
pub fn ogl_main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut title: String = "OpenGL Application".into();
    let mut width: i32 = 900;
    let mut height: i32 = 700;
    let mut bg_color: Vec<f32> = vec![0.0, 0.0, 0.0, 1.0];

    let attr = syn::parse_macro_input!(attr as syn::AttributeArgs);
    attr.iter().for_each(|a: &NestedMeta| {
        match a {
            NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                                                 path,
                                                 lit,
                                                 eq_token: _,
                                             })) => {
                match path.segments.first().unwrap().ident.to_string().as_str() {
                    "title" => {
                        title = match lit {
                            Lit::Str(value) => value.value(),
                            _ => panic!("Expected string literal for title"),
                        };
                    },

                    "window" => {
                        match lit {
                            Lit::Str(value) => {
                                let dimensions = value.value();

                                assert!(Regex::new(r"\d+x\d+").unwrap().is_match(dimensions.as_ref()), "Expected format: WIDTHxHEIGHT");

                                let mut v = dimensions.split('x');
                                width = v.next().unwrap().parse::<i32>().unwrap();
                                height = v.next().unwrap().parse::<i32>().unwrap();
                            },
                            _ => panic!("Expected string for window options"),
                        };
                    }

                    "bg_color" => {
                        match lit {
                            Lit::Str(value) => {
                                let color = value.value();

                                assert!(Regex::new(r"(\d+(\.\d+)? ){3}(\d+(\.\d+)?)").unwrap().is_match(color.as_ref()));

                                let mut v = color.split(' ');
                                let r = v.next().unwrap().parse::<f32>().unwrap();
                                let g = v.next().unwrap().parse::<f32>().unwrap();
                                let b = v.next().unwrap().parse::<f32>().unwrap();
                                let a = v.next().unwrap().parse::<f32>().unwrap();
                                bg_color = vec![r, g, b, a];
                            },
                            _ => panic!("Expected string for bg_color options"),
                        }
                    }

                    attr => panic!("Unknown attribute: {}", attr),
                }
            },
            _ => panic!("Could not undertand attribute"),
        }
    });

    let bg_color = quote! {
        [#(#bg_color),*]
    };

    let mut function = syn::parse_macro_input!(item as syn::ItemFn);
    let og_stmts = function.block.stmts.clone();
    function.block.stmts = Vec::new();

    function.block.stmts.push(syn::parse_quote!(let title = #title;));
    function.block.stmts.push(syn::parse_quote!(let window_width = #width;));
    function.block.stmts.push(syn::parse_quote!(let window_height = #height;));
    function.block.stmts.push(syn::parse_quote!(let bg_color = #bg_color;));

    function.block.stmts.extend(quote_vec![
        // Initialize SDL
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        // Setup GL attributes
        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);

        // Create a window
        let window = video_subsystem
            .window(title, window_width as u32, window_height as u32)
            .opengl() // Setup window to receive GL context
            .resizable()
            .build()
            .unwrap();

        // Create an GL context
        let gl_context = window.gl_create_context().unwrap();
        let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
        unsafe {
            gl::Viewport(0, 0, window_width, window_height);
            gl::ClearColor(bg_color[0], bg_color[1], bg_color[2], bg_color[3]);
        };
    ]);
    function.block.stmts.extend(og_stmts);
    let gen = quote! { #function };
    gen.into()
}
