//声明macro_derive包是过程宏，需要依赖syn和quote包，还需要在[lib]中将过程宏开关开启，具体看../Cargo.toml
use proc_macro::TokenStream;
use quote::quote;

//自定义派生(derive)定义
//#[proc_macro_derive(PrintFn)]，当使用#[derive(PrintFn)]注解后，macro_derive函数将会被调用，特征名：PrintFn
#[proc_macro_derive(PrintFn)]
pub fn macro_derive(input: TokenStream) -> TokenStream {
    //syn解析TokenStream，将字符串形式的Rust代码解析为一个AST树的数据结构
    /* DeriveInput结构体示例(结构体更多字段描述可查阅syn中DeriveInput的文档)：
    DeriveInput {
        // --snip--

        ident: Ident {
            ident: "Pancakes",
            span: #0 bytes(95..103)
        },
        data: Struct(
            DataStruct {
                struct_token: Struct,
                fields: Unit,
                semi_token: Some(
                    Semi
                )
            }
        )
    }
     */
    let ast = syn::parse(input).unwrap();
    //操作这个数据结构，并通过quote包转换回Rust代码
    impl_macro(&ast)
}

fn impl_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    //quote!内编写Rust代码，访问Ident结构体实例，#name即quote!会获取name变量值
    let gen = quote! {
        impl PrintFn for #name{
            fn print_name(){
                println!("Hello, Macro! My name is {}!",stringify!(#name));
            }
        }
    };
    //into方法转换为TokenStream
    gen.into()
}

//类属性宏定义
//#[proc_macro_attribute]，可通过#[show_streams]或#[show_streams(...)]调用，其中...是任意标记树
//第一个参数是属性名称后带分隔符的标记树，如果只有属性名称，则参数值为空
//第二个参数是添加了该宏属性的条目
#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

//类函数宏定义
//#[proc_macro]make_answer!()调用
#[proc_macro]
pub fn make_answer(input: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 111 }".parse().unwrap()
}
