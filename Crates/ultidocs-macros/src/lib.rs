use proc_macro::{TokenStream};


// mod helpers;
// use helpers::{
//     parse_input,
//     generate_runtime_types,
//     generate_dir
// };
#[cfg(any(feature = "dir", feature = "web"))]
mod macros;

#[cfg(all(feature = "minify_web", feature = "format_web"))]
compile_error!("minify_web and format_web cannot both be enabled");

#[proc_macro]
pub fn include_dir(input: TokenStream) -> TokenStream {
    #[cfg(feature = "dir")]
    {
        let input_string = input.to_string();
        let out = macros::dir::perform_dir(input_string);
        return out.parse().unwrap();
    }

    #[cfg(not(feature = "dir"))]
    {
        return "compile_error!(\"dir feature is not enabled\");"
            .parse()
            .unwrap();
    }
}


#[proc_macro]
pub fn include_web(input: TokenStream) -> TokenStream {
    #[cfg(feature = "web")]
    {
        let input_string = input.to_string();
        let out = macros::web::perform_web(input_string);
        return out.parse().unwrap();
    }

    #[cfg(not(feature = "web"))]
    {
        return "compile_error!(\"web feature is not enabled\");"
            .parse()
            .unwrap();
    }
}