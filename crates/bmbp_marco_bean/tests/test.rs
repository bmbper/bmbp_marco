use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Demo {
    data_id: Option<String>,
    data_level: Option<String>,
    data_flag: Option<String>,
    data_status: Option<String>,
    data_sort: Option<i32>,
    data_create_time: Option<String>,
    data_create_user: Option<String>,
    data_update_time: Option<String>,
    data_update_user: Option<String>,
    data_owner_org: Option<String>,
    data_sign: Option<String>,
    org_code: Option<String>,
    org_parent_code: Option<String>,
    org_name: Option<String>,
    org_code_path: Option<String>,
    org_name_path: Option<String>,
    org_tree_grade: Option<u32>,
    org_leaf: Option<String>,
    org_type: Option<String>,
    org_children: Option<Vec<Demo>>,
    name: Option<String>,
}
impl Demo {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_data_id(&mut self, value: Option<String>) -> &mut Self {
        self.data_id = value;
        self
    }
    pub fn get_data_id(&self) -> &Option<String> {
        &self.data_id
    }
    pub fn get_mut_data_id(&mut self) -> &mut Option<String> {
        &mut self.data_id
    }
    pub fn set_data_level(&mut self, value: Option<String>) -> &mut Self {
        self.data_level = value;
        self
    }
    pub fn get_data_level(&self) -> &Option<String> {
        &self.data_level
    }
    pub fn get_mut_data_level(&mut self) -> &mut Option<String> {
        &mut self.data_level
    }
    pub fn set_data_flag(&mut self, value: Option<String>) -> &mut Self {
        self.data_flag = value;
        self
    }
    pub fn get_data_flag(&self) -> &Option<String> {
        &self.data_flag
    }
    pub fn get_mut_data_flag(&mut self) -> &mut Option<String> {
        &mut self.data_flag
    }
    pub fn set_data_status(&mut self, value: Option<String>) -> &mut Self {
        self.data_status = value;
        self
    }
    pub fn get_data_status(&self) -> &Option<String> {
        &self.data_status
    }
    pub fn get_mut_data_status(&mut self) -> &mut Option<String> {
        &mut self.data_status
    }
    pub fn set_data_sort(&mut self, value: Option<i32>) -> &mut Self {
        self.data_sort = value;
        self
    }
    pub fn get_data_sort(&self) -> &Option<i32> {
        &self.data_sort
    }
    pub fn get_mut_data_sort(&mut self) -> &mut Option<i32> {
        &mut self.data_sort
    }
    pub fn set_data_create_time(&mut self, value: Option<String>) -> &mut Self {
        self.data_create_time = value;
        self
    }
    pub fn get_data_create_time(&self) -> &Option<String> {
        &self.data_create_time
    }
    pub fn get_mut_data_create_time(&mut self) -> &mut Option<String> {
        &mut self.data_create_time
    }
    pub fn set_data_create_user(&mut self, value: Option<String>) -> &mut Self {
        self.data_create_user = value;
        self
    }
    pub fn get_data_create_user(&self) -> &Option<String> {
        &self.data_create_user
    }
    pub fn get_mut_data_create_user(&mut self) -> &mut Option<String> {
        &mut self.data_create_user
    }
    pub fn set_data_update_time(&mut self, value: Option<String>) -> &mut Self {
        self.data_update_time = value;
        self
    }
    pub fn get_data_update_time(&self) -> &Option<String> {
        &self.data_update_time
    }
    pub fn get_mut_data_update_time(&mut self) -> &mut Option<String> {
        &mut self.data_update_time
    }
    pub fn set_data_update_user(&mut self, value: Option<String>) -> &mut Self {
        self.data_update_user = value;
        self
    }
    pub fn get_data_update_user(&self) -> &Option<String> {
        &self.data_update_user
    }
    pub fn get_mut_data_update_user(&mut self) -> &mut Option<String> {
        &mut self.data_update_user
    }
    pub fn set_data_owner_org(&mut self, value: Option<String>) -> &mut Self {
        self.data_owner_org = value;
        self
    }
    pub fn get_data_owner_org(&self) -> &Option<String> {
        &self.data_owner_org
    }
    pub fn get_mut_data_owner_org(&mut self) -> &mut Option<String> {
        &mut self.data_owner_org
    }
    pub fn set_data_sign(&mut self, value: Option<String>) -> &mut Self {
        self.data_sign = value;
        self
    }
    pub fn get_data_sign(&self) -> &Option<String> {
        &self.data_sign
    }
    pub fn get_mut_data_sign(&mut self) -> &mut Option<String> {
        &mut self.data_sign
    }
    pub fn set_org_code(&mut self, value: Option<String>) -> &mut Self {
        self.org_code = value;
        self
    }
    pub fn get_org_code(&self) -> &Option<String> {
        &self.org_code
    }
    pub fn get_mut_org_code(&mut self) -> &mut Option<String> {
        &mut self.org_code
    }
    pub fn set_org_parent_code(&mut self, value: Option<String>) -> &mut Self {
        self.org_parent_code = value;
        self
    }
    pub fn get_org_parent_code(&self) -> &Option<String> {
        &self.org_parent_code
    }
    pub fn get_mut_org_parent_code(&mut self) -> &mut Option<String> {
        &mut self.org_parent_code
    }
    pub fn set_org_name(&mut self, value: Option<String>) -> &mut Self {
        self.org_name = value;
        self
    }
    pub fn get_org_name(&self) -> &Option<String> {
        &self.org_name
    }
    pub fn get_mut_org_name(&mut self) -> &mut Option<String> {
        &mut self.org_name
    }
    pub fn set_org_code_path(&mut self, value: Option<String>) -> &mut Self {
        self.org_code_path = value;
        self
    }
    pub fn get_org_code_path(&self) -> &Option<String> {
        &self.org_code_path
    }
    pub fn get_mut_org_code_path(&mut self) -> &mut Option<String> {
        &mut self.org_code_path
    }
    pub fn set_org_name_path(&mut self, value: Option<String>) -> &mut Self {
        self.org_name_path = value;
        self
    }
    pub fn get_org_name_path(&self) -> &Option<String> {
        &self.org_name_path
    }
    pub fn get_mut_org_name_path(&mut self) -> &mut Option<String> {
        &mut self.org_name_path
    }
    pub fn set_org_tree_grade(&mut self, value: Option<u32>) -> &mut Self {
        self.org_tree_grade = value;
        self
    }
    pub fn get_org_tree_grade(&self) -> &Option<u32> {
        &self.org_tree_grade
    }
    pub fn get_mut_org_tree_grade(&mut self) -> &mut Option<u32> {
        &mut self.org_tree_grade
    }
    pub fn set_org_leaf(&mut self, value: Option<String>) -> &mut Self {
        self.org_leaf = value;
        self
    }
    pub fn get_org_leaf(&self) -> &Option<String> {
        &self.org_leaf
    }
    pub fn get_mut_org_leaf(&mut self) -> &mut Option<String> {
        &mut self.org_leaf
    }
    pub fn set_org_type(&mut self, value: Option<String>) -> &mut Self {
        self.org_type = value;
        self
    }
    pub fn get_org_type(&self) -> &Option<String> {
        &self.org_type
    }
    pub fn get_mut_org_type(&mut self) -> &mut Option<String> {
        &mut self.org_type
    }
    pub fn set_org_children(&mut self, value: Option<Vec<Demo>>) -> &mut Self {
        self.org_children = value;
        self
    }
    pub fn get_org_children(&self) -> &Option<Vec<Demo>> {
        &self.org_children
    }
    pub fn get_mut_org_children(&mut self) -> &mut Option<Vec<Demo>> {
        &mut self.org_children
    }
    pub fn set_name(&mut self, value: Option<String>) -> &mut Self {
        self.name = value;
        self
    }
    pub fn get_name(&self) -> &Option<String> {
        &self.name
    }
    pub fn get_mut_name(&mut self) -> &mut Option<String> {
        &mut self.name
    }
}
