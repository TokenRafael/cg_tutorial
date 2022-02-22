use proc_macro::TokenStream;
use quote::quote;

macro_rules! quote_vec {
    ($($x:stmt;)*) => {
        vec![
            $(
                syn::parse_quote!($x),
            )*
        ]
    };
}

#[proc_macro_attribute]
pub fn ogl_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = syn::parse_macro_input!(item as syn::ItemFn);
    let og_stmts = function.block.stmts.clone();
    function.block.stmts = quote_vec![
        // Initialize SDL
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        // Setup GL attributes
        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);

        // Create a window
        let window = video_subsystem
            .window("Game", 900, 700)
            .opengl() // Setup window to receive GL context
            .resizable()
            .build()
            .unwrap();

        // Create an GL context
        let gl_context = window.gl_create_context().unwrap();
        let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
        unsafe {
            gl::Viewport(0, 0, 900, 700);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        };
    ];
    function.block.stmts.extend(og_stmts);
    let gen = quote! { #function };
    gen.into()
}
