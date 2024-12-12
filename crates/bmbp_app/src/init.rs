use bmbp_orm::{init_bmbp_orm, RdbcDataSource, RdbcDbType, RdbcOrm, BMBP_ORM};
use bmbp_vars::{set_ctx_var, BMBP_APP_HOME_URL, BMBP_APP_LOGIN_NAME, BMBP_APP_WHITE_LIST};
use salvo::Router;

pub fn init_app_router() -> Router {
    let mut router = Router::new()
        .push(bmbp_ui_lib::build_router())
        .push(bmbp_ui_theme::build_router())
        .push(bmbp_framework::build_router())
        .push(bmbp_rbac::build_router())
        .push(bmbp_home::build_router());
    let auth_router = bmbp_app_auth::build_router();
    router = auth_router.push(router);
    router
}

pub fn init_template() {
    bmbp_framework::build_template();
    bmbp_rbac::build_template();
}

pub fn init_white_list() {
    set_ctx_var(BMBP_APP_WHITE_LIST, "/static/**,/auth/**");
    set_ctx_var(BMBP_APP_LOGIN_NAME, "Bmbp应用开发脚手架");
    set_ctx_var(BMBP_APP_HOME_URL, "/framework.view");
}

pub async fn initialize_orm() {
    let ds = RdbcDataSource {
        db_type: RdbcDbType::Postgres,
        host: "127.0.0.1".to_string(),
        port: 5432,
        user: "bmbp".to_string(),
        password: "zgk0130!".to_string(),
        db_name: "bmbp".to_string(),
        charset: "utf8".to_string(),
        pool_config: Default::default(),
    };
    let rs = init_bmbp_orm(ds).await;
    if rs.is_err() {
        panic!("{}", rs.err().unwrap().to_string())
    }
}
