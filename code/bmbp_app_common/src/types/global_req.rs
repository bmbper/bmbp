use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct PageParams<T>
where
    T: Serialize + Clone + Default + Send + Sync,
{
    page_no: usize,
    page_size: usize,
    #[serde(flatten)]
    params: Option<T>,
}

#[allow(unused)]
impl<T> PageParams<T>
where
    T: Serialize + Clone + Default + Send + Sync,
{
    pub fn get_page_no(&self) -> &usize {
        &self.page_no
    }
    pub fn get_page_size(&self) -> &usize {
        &self.page_size
    }
    pub fn get_params(&self) -> Option<&T> {
        self.params.as_ref()
    }
    pub fn get_mut_params(&mut self) -> Option<&mut T> {
        self.params.as_mut()
    }
    pub fn set_page_no(&mut self, page_no: usize) -> &mut Self {
        self.page_no = page_no;
        self
    }
    pub fn set_page_size(&mut self, page_size: usize) -> &mut Self {
        self.page_size = page_size;
        self
    }
    pub fn set_params(&mut self, params: T) -> &mut Self {
        self.params = Some(params);
        self
    }
}
