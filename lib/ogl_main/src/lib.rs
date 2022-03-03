use proc_macro::TokenStream;

use quote::quote;
mod options;

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
    let attr = syn::parse_macro_input!(attr as syn::AttributeArgs);
    let mut function = syn::parse_macro_input!(item as syn::ItemFn);

    let og_stmts = function.block.stmts.clone();
    function.block.stmts = Vec::new();

    options::parse_options(attr, &mut function);

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
        let gl = gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
        unsafe {
            gl.Viewport(0, 0, window_width, window_height);
            gl.ClearColor(bg_color[0], bg_color[1], bg_color[2], bg_color[3]);
        };
    ]);
    function.block.stmts.extend(og_stmts);
    let gen = quote! { #function };
    gen.into()
}
