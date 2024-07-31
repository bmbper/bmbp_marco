extern crate core;

use proc_macro::TokenStream;
mod bean;
mod rdbc_bean;
mod rdbc_tree_bean;
mod tree_bean;

///    bean
///    ## example
///    ```rust
///     #[bean]
///     pub struct Demo{
///         name:String
///     }
///    ````
///    expand to:
///
///     ```rust
///         #[derive(Default, Debug, Clone, Serialize, Deserialize)]
///         #[serde(rename_all = "camelCase")]
///         #[serde(default)]
///         pub struct Demo {
///             name: String,
///         }
///         impl Demo {
///             pub fn new() -> Self {
///                 Self::default()
///             }
///             pub fn set_name(&mut self, value: String) -> &mut Self {
///                 self.name = value;
///                 self
///             }
///             pub fn get_name(&self) -> &String {
///                 &self.name
///             }
///             pub fn get_mut_name(&mut self) -> &mut String {
///                 &mut self.name
///            }
///         }
///
///     ```
///
#[proc_macro_attribute]
pub fn bean(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    bean::marco_bean(bean_meta_token, bean_struct_token)
}
///    tree_bean(organ)
///    增加指定的字段：
///     `organ_code`、`organ_name`、`organ_code_path`、`organ_code_name`、`organ_parent_code`、`organ_tree_grade`、`organ_leaf`
///     `organ_type`、`organ_children`
///    ## example
///    ```rust
///     #[tree_bean(organ)]
///     pub struct Demo{
///         name:String
///     }
///    ````
///    expand to:
///
///     ```rust
///
///
/// #[derive(Default, Debug, Clone, Serialize, Deserialize)]
/// #[serde(rename_all = "camelCase")]
/// #[serde(default)]
/// pub struct Demo {
///    organ_code: Option<String>,
///    organ_parent_code: Option<String>,
///    organ_name: Option<String>,
///    organ_code_path: Option<String>,
///    organ_name_path: Option<String>,
///    organ_tree_grade: Option<u32>,
///    organ_leaf: Option<String>,
///    organ_type: Option<String>,
///    organ_children: Option<Vec<Demo>>,
/// }
/// impl Demo {
///    pub fn new() -> Self {
///        Self::default()
///    }
///    pub fn set_organ_code(&mut self, value: Option<String>) -> &mut Self {
///        self.organ_code = value;
///        self
///    }
///    pub fn get_organ_code(&self) -> &Option<String> {
///        &self.organ_code
///    }
///    pub fn get_mut_organ_code(&mut self) -> &mut Option<String> {
///        &mut self.organ_code
///    }
///    pub fn set_organ_parent_code(&mut self, value: Option<String>) -> &mut Self {
///        self.organ_parent_code = value;
///        self
///    }
///    pub fn get_organ_parent_code(&self) -> &Option<String> {
///        &self.organ_parent_code
///    }
///    pub fn get_mut_organ_parent_code(&mut self) -> &mut Option<String> {
///        &mut self.organ_parent_code
///    }
///    pub fn set_organ_name(&mut self, value: Option<String>) -> &mut Self {
///        self.organ_name = value;
///        self
///    }
///    pub fn get_organ_name(&self) -> &Option<String> {
///        &self.organ_name
///    }
///    pub fn get_mut_organ_name(&mut self) -> &mut Option<String> {
///        &mut self.organ_name
///    }
///    pub fn set_organ_code_path(&mut self, value: Option<String>) -> &mut Self {
///        self.organ_code_path = value;
///        self
///    }
///    pub fn get_organ_code_path(&self) -> &Option<String> {
///        &self.organ_code_path
///    }
///    pub fn get_mut_organ_code_path(&mut self) -> &mut Option<String> {
///        &mut self.organ_code_path
///    }
///    pub fn set_organ_name_path(&mut self, value: Option<String>) -> &mut Self {
///        self.organ_name_path = value;
///        self
///    }
///    pub fn get_organ_name_path(&self) -> &Option<String> {
///        &self.organ_name_path
///    }
///    pub fn get_mut_organ_name_path(&mut self) -> &mut Option<String> {
///        &mut self.organ_name_path
///    }
///    pub fn set_organ_tree_grade(&mut self, value: Option<u32>) -> &mut Self {
///        self.organ_tree_grade = value;
///        self
///    }
///    pub fn get_organ_tree_grade(&self) -> &Option<u32> {
///        &self.organ_tree_grade
///    }
///    pub fn get_mut_organ_tree_grade(&mut self) -> &mut Option<u32> {
///        &mut self.organ_tree_grade
///    }
///    pub fn set_organ_leaf(&mut self, value: Option<String>) -> &mut Self {
///        self.organ_leaf = value;
///        self
///    }
///    pub fn get_organ_leaf(&self) -> &Option<String> {
///        &self.organ_leaf
///    }
///    pub fn get_mut_organ_leaf(&mut self) -> &mut Option<String> {
///        &mut self.organ_leaf
///    }
///    pub fn set_organ_type(&mut self, value: Option<String>) -> &mut Self {
///        self.organ_type = value;
///        self
///    }
///    pub fn get_organ_type(&self) -> &Option<String> {
///        &self.organ_type
///    }
///    pub fn get_mut_organ_type(&mut self) -> &mut Option<String> {
///        &mut self.organ_type
///    }
///    pub fn set_organ_children(&mut self, value: Option<Vec<Demo>>) -> &mut Self {
///        self.organ_children = value;
///        self
///    }
///    pub fn get_organ_children(&self) -> &Option<Vec<Demo>> {
///        &self.organ_children
///    }
///    pub fn get_mut_organ_children(&mut self) -> &mut Option<Vec<Demo>> {
///        &mut self.organ_children
///    }
/// }
///     ```
///
#[proc_macro_attribute]
pub fn tree_bean(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    let token = tree_bean::marco_tree_bean(bean_meta_token, bean_struct_token);
    println!("===>{}", token.to_string());
    token
}
/// rdbc_bean 增加基础字段 并设置get set 方法
#[proc_macro_attribute]
pub fn rdbc_bean(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    rdbc_bean::marco_rdbc_bean(bean_meta_token, bean_struct_token)
}
/// rdbc_bean 增加基础字段 并设置get set 方法
#[proc_macro_attribute]
pub fn rdbc_tree_bean(bean_meta_token: TokenStream, bean_struct_token: TokenStream) -> TokenStream {
    rdbc_tree_bean::marco_rdbc_tree_bean(bean_meta_token, bean_struct_token)
}
