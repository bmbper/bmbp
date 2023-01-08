#[derive(Clone)]
pub enum DdlSQL {
    DropDataBase(DropDataBaseSQL),
    CreateDataBase(CreateDataBaseSQL),
    DropSchemaBase(DropSchemaSQL),
    CreateSchemaBase(CreateSchemaSQL),
    DropTable(DropTableSQL),
    CreateTable(CreateTableSQL),
    DropView(DropViewSQL),
    CreateView(CreateViewSQL),
    DropIndex(DropIndexSQL),
    CreateIndex(CreateIndexSQL),
    DropConstriant(DropConstriantSQL),
    CreateConstriant(CreateConstriantSQL),
    DropColumn(DropColumnSQL),
    CreateColumn(CreateColumnSQL),
    DropComment(DropCommentSQL),
    CreateComment(CreateCommentSQL),
    AlterTable(AlterTableSQL),
    AlterColumn(AlterColumnSQL),
}
#[derive(Clone)]
pub struct CreateDataBaseSQL {}
#[derive(Clone)]
pub struct DropDataBaseSQL {}
#[derive(Clone)]
pub struct CreateSchemaSQL {}
#[derive(Clone)]
pub struct DropSchemaSQL {}
#[derive(Clone)]
pub struct CreateTableSQL {}
#[derive(Clone)]
pub struct DropTableSQL {}
#[derive(Clone)]
pub struct CreateViewSQL {}
#[derive(Clone)]
pub struct DropViewSQL {}
#[derive(Clone)]
pub struct CreateIndexSQL {}
#[derive(Clone)]
pub struct DropIndexSQL {}
#[derive(Clone)]
pub struct CreateCommentSQL {}
#[derive(Clone)]
pub struct DropCommentSQL {}
#[derive(Clone)]
pub struct CreateConstriantSQL {}
#[derive(Clone)]
pub struct DropConstriantSQL {}
#[derive(Clone)]
pub struct CreateColumnSQL {}
#[derive(Clone)]
pub struct DropColumnSQL {}
#[derive(Clone)]
pub struct AlterTableSQL {}
#[derive(Clone)]
pub struct AlterColumnSQL {}
