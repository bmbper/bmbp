use bmbp_app_common::{BmbpError, BmbpResp, PageVo};
use bmbp_rdbc_orm::{Delete, Insert, Query, RdbcFilter, RdbcORM, RdbcOrmRow, RdbcTable, Update};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct RdbcModel {
    name: Option<String>,
    age: Option<i32>,
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
}
impl RdbcModel {
    pub fn new() -> Self {
        Self::default()
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
    pub fn set_age(&mut self, value: Option<i32>) -> &mut Self {
        self.age = value;
        self
    }
    pub fn get_age(&self) -> &Option<i32> {
        &self.age
    }
    pub fn get_mut_age(&mut self) -> &mut Option<i32> {
        &mut self.age
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
}
impl RdbcModel {
    pub fn get_table_name() -> String {
        return "RDBC_MODEL".to_string();
    }
    pub fn get_table_primary_key() -> String {
        return "data_id".to_string();
    }
    pub fn get_table_columns() -> Vec<String> {
        return vec![
            "name".to_string(),
            "age".to_string(),
            "data_id".to_string(),
            "data_level".to_string(),
            "data_flag".to_string(),
            "data_status".to_string(),
            "data_sort".to_string(),
            "data_create_time".to_string(),
            "data_create_user".to_string(),
            "data_update_time".to_string(),
            "data_update_user".to_string(),
            "data_owner_org".to_string(),
            "data_sign".to_string(),
        ];
    }
}
impl RdbcModel {
    pub fn build_query_sql() -> Query {
        let mut query = Query::new();
        query.table(Self::get_table_name());
        query.select_vec(Self::get_table_columns());
        query.order_by("data_sort", true);
        query.order_by("data_update_time", false);
        query
    }
    pub fn build_info_sql(data_id: &Option<String>) -> Query {
        let mut query = Query::new();
        query.table(Self::get_table_name());
        query.select_vec(Self::get_table_columns());
        query.eq_(Self::get_table_primary_key(), data_id);
        query
    }
    pub fn build_remove_sql(data_id: &Option<String>) -> Delete {
        let mut delete = Delete::new();
        delete.table(Self::get_table_name());
        delete.eq_(Self::get_table_primary_key(), data_id);
        delete
    }
    pub fn build_enable_sql(data_id: &Option<String>) -> Update {
        let mut update = Update::new();
        update.table(Self::get_table_name());
        update.set("data_status", "1");
        update.eq_(Self::get_table_primary_key(), data_id);
        update
    }
    pub fn build_disable_sql(data_id: &Option<String>) -> Update {
        let mut update = Update::new();
        update.table(Self::get_table_name());
        update.set("data_status", "0");
        update.eq_(Self::get_table_primary_key(), data_id);
        update
    }
    pub fn build_update_status_sql(data_id: &Option<String>, status: String) -> Update {
        let mut update = Update::new();
        update.table(Self::get_table_name());
        update.set("data_status", status);
        update.eq_(Self::get_table_primary_key(), data_id);
        update
    }
    pub fn build_update_flag_sql(data_id: &Option<String>, flag: String) -> Update {
        let mut update = Update::new();
        update.table(Self::get_table_name());
        update.set("data_flag", flag);
        update.eq_(Self::get_table_primary_key(), data_id);
        update
    }
    pub fn build_insert_sql(&self) -> Insert {
        let mut insert = Insert::new();
        insert.table(Self::get_table_name());
        if let Some(value) = self.get_name() {
            insert.insert_column_value("name", value);
        }
        if let Some(value) = self.get_age() {
            insert.insert_column_value("age", value);
        }
        if let Some(value) = self.get_data_id() {
            insert.insert_column_value("data_id", value);
        }
        if let Some(value) = self.get_data_level() {
            insert.insert_column_value("data_level", value);
        }
        if let Some(value) = self.get_data_flag() {
            insert.insert_column_value("data_flag", value);
        }
        if let Some(value) = self.get_data_status() {
            insert.insert_column_value("data_status", value);
        }
        if let Some(value) = self.get_data_sort() {
            insert.insert_column_value("data_sort", value);
        }
        if let Some(value) = self.get_data_create_time() {
            insert.insert_column_value("data_create_time", value);
        }
        if let Some(value) = self.get_data_create_user() {
            insert.insert_column_value("data_create_user", value);
        }
        if let Some(value) = self.get_data_update_time() {
            insert.insert_column_value("data_update_time", value);
        }
        if let Some(value) = self.get_data_update_user() {
            insert.insert_column_value("data_update_user", value);
        }
        if let Some(value) = self.get_data_owner_org() {
            insert.insert_column_value("data_owner_org", value);
        }
        if let Some(value) = self.get_data_sign() {
            insert.insert_column_value("data_sign", value);
        }
        insert
    }
    pub fn build_update_sql(&self) -> Update {
        let mut update = Update::new();
        update.table(Self::get_table_name());
        if let Some(value) = self.get_name() {
            update.set("name", value);
        }
        if let Some(value) = self.get_age() {
            update.set("age", value);
        }
        if let Some(value) = self.get_data_id() {
            update.set("data_id", value);
        }
        if let Some(value) = self.get_data_level() {
            update.set("data_level", value);
        }
        if let Some(value) = self.get_data_flag() {
            update.set("data_flag", value);
        }
        if let Some(value) = self.get_data_status() {
            update.set("data_status", value);
        }
        if let Some(value) = self.get_data_sort() {
            update.set("data_sort", value);
        }
        if let Some(value) = self.get_data_create_time() {
            update.set("data_create_time", value);
        }
        if let Some(value) = self.get_data_create_user() {
            update.set("data_create_user", value);
        }
        if let Some(value) = self.get_data_update_time() {
            update.set("data_update_time", value);
        }
        if let Some(value) = self.get_data_update_user() {
            update.set("data_update_user", value);
        }
        if let Some(value) = self.get_data_owner_org() {
            update.set("data_owner_org", value);
        }
        if let Some(value) = self.get_data_sign() {
            update.set("data_sign", value);
        }
        update.eq_(Self::get_table_primary_key(), self.get_data_id());
        update
    }
}
impl From<RdbcOrmRow> for RdbcModel {
    fn from(row: RdbcOrmRow) -> Self {
        let mut model = RdbcModel::new();
        if let Some(data) = row.get_data().get("name") {
            model.set_name(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("age") {
            model.set_age(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_id") {
            model.set_data_id(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_level") {
            model.set_data_level(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_flag") {
            model.set_data_flag(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_status") {
            model.set_data_status(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_sort") {
            model.set_data_sort(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_create_time") {
            model.set_data_create_time(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_create_user") {
            model.set_data_create_user(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_update_time") {
            model.set_data_update_time(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_update_user") {
            model.set_data_update_user(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_owner_org") {
            model.set_data_owner_org(Some(data.into()));
        }
        if let Some(data) = row.get_data().get("data_sign") {
            model.set_data_sign(Some(data.into()));
        }
        model
    }
}
pub struct RdbcModelOrm;
impl RdbcModelOrm {
    pub async fn select_page_by_query(
        page_no: &usize,
        page_size: &usize,
        query: &Query,
    ) -> BmbpResp<PageVo<RdbcModel>> {
        match RdbcORM
            .await
            .select_page_by_query::<RdbcModel>(page_no.clone(), page_size.clone(), &query)
            .await
        {
            Ok(mut page) => {
                let mut page_vo = PageVo::new();
                page_vo.set_page_no(page.page_num().clone());
                page_vo.set_page_size(page.page_size().clone());
                page_vo.set_op_data(page.data_take());
                page_vo.set_row_total(page.total().clone());
                Ok(page_vo)
            }
            Err(e) => Err(BmbpError::service(e.get_msg().as_str())),
        }
    }
    pub async fn select_list_by_query(query: &Query) -> BmbpResp<Option<Vec<RdbcModel>>> {
        match RdbcORM
            .await
            .select_list_by_query::<RdbcModel>(&query)
            .await
        {
            Ok(data) => Ok(data),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
        }
    }
    pub async fn select_one_by_query(query: &Query) -> BmbpResp<Option<RdbcModel>> {
        match RdbcORM.await.select_one_by_query::<RdbcModel>(&query).await {
            Ok(data) => Ok(data),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
        }
    }
    pub async fn execute_insert(insert: &Insert) -> BmbpResp<usize> {
        match RdbcORM.await.execute_insert(insert).await {
            Ok(data) => Ok(data as usize),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
        }
    }
    pub async fn execute_update(update: &Update) -> BmbpResp<usize> {
        match RdbcORM.await.execute_update(update).await {
            Ok(data) => Ok(data as usize),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
        }
    }
    pub async fn execute_delete(delete: &Delete) -> BmbpResp<usize> {
        match RdbcORM.await.execute_delete(delete).await {
            Ok(data) => Ok(data as usize),
            Err(err) => Err(BmbpError::service(err.get_msg().as_str())),
        }
    }
}
impl RdbcModel {
    pub async fn find_one(&self) -> BmbpResp<Option<Self>> {
        Ok(Some(self.clone()))
    }
    pub async fn save(&self) -> BmbpResp<Option<Self>> {
        let model = self.find_one().await?;
        if model.is_some() {
            self.update().await?;
        } else {
            self.insert().await?;
        }
        self.find_one().await
    }
    pub async fn insert(&self) -> BmbpResp<usize> {
        let insert = self.build_insert_sql();
        Ok(0)
    }
    pub async fn update(&self) -> BmbpResp<usize> {
        let insert = self.build_update_sql();
        Ok(0)
    }
    pub async fn remove(&self) -> BmbpResp<usize> {
        Ok(0)
    }
    pub async fn remove_logic(&self) -> BmbpResp<usize> {
        Ok(0)
    }
    pub async fn enable(&self) -> BmbpResp<usize> {
        Ok(0)
    }
    pub async fn disable(&self) -> BmbpResp<usize> {
        Ok(0)
    }
    pub async fn find_page(
        page_no: Option<&usize>,
        page_size: Option<&usize>,
    ) -> BmbpResp<PageVo<Self>> {
        Ok(PageVo::new())
    }
    pub async fn find_page_by_query(
        page_no: Option<&usize>,
        page_size: Option<&usize>,
        query: Query,
    ) -> BmbpResp<PageVo<Self>> {
        Ok(PageVo::new())
    }
    pub async fn find_list() -> BmbpResp<Option<Vec<Self>>> {
        Ok(None)
    }
    pub async fn find_list_by_query(query: &Query) -> BmbpResp<Option<Vec<Self>>> {
        Ok(None)
    }
    pub async fn find_removed_page(
        page_no: Option<&usize>,
        page_size: Option<&usize>,
    ) -> BmbpResp<PageVo<Self>> {
        Ok(PageVo::new())
    }
    pub async fn find_removed_page_by_query(
        page_no: Option<&usize>,
        page_size: Option<&usize>,
    ) -> BmbpResp<PageVo<Self>> {
        Ok(PageVo::new())
    }
    pub async fn find_removed_list() -> BmbpResp<Option<Vec<Self>>> {
        Ok(None)
    }
    pub async fn find_removed_list_by_query(query: &Query) -> BmbpResp<Option<Vec<Self>>> {
        Ok(None)
    }
    pub async fn find_all_page(
        page_no: Option<&usize>,
        page_size: Option<&usize>,
    ) -> BmbpResp<PageVo<Self>> {
        Ok(PageVo::new())
    }
    pub async fn find_all_page_by_query(
        page_no: Option<&usize>,
        page_size: Option<&usize>,
        query: Query,
    ) -> BmbpResp<PageVo<Self>> {
        Ok(PageVo::new())
    }
    pub async fn find_all_list() -> BmbpResp<Option<Vec<Self>>> {
        Ok(None)
    }
    pub async fn find_all_list_by_query(query: &Query) -> BmbpResp<Option<Vec<Self>>> {
        Ok(None)
    }
    pub async fn find_list_by_id_slice(id: Option<&[String]>) -> BmbpResp<Option<Vec<Self>>> {
        Ok(None)
    }
    pub async fn find_by_id(id: Option<&String>) -> BmbpResp<Option<Self>> {
        Ok(None)
    }
    pub async fn remove_by_id(id: Option<&String>) -> BmbpResp<usize> {
        Ok(0)
    }
    pub async fn remove_by_id_slice(id: &[String]) -> BmbpResp<usize> {
        Ok(0)
    }
    pub async fn remove_logic_by_id(id: Option<&String>) -> BmbpResp<usize> {
        Ok(0)
    }
    pub async fn remove_logic_by_id_slice(id: &[String]) -> BmbpResp<usize> {
        Ok(0)
    }
    pub async fn enable_by_id(id: Option<String>) -> BmbpResp<usize> {
        Ok(0)
    }
    pub async fn enable_by_id_slice(id: &[String]) -> BmbpResp<usize> {
        Ok(0)
    }
    pub async fn disable_by_id(id: Option<String>) -> BmbpResp<usize> {
        Ok(0)
    }
    pub async fn disable_by_id_slice(id: &[String]) -> BmbpResp<usize> {
        Ok(0)
    }
}
impl RdbcModel {
    pub fn valid(&self) -> BmbpResp<()> {
        Ok(())
    }
}
