#[allow(dead_code)]
#[derive(Clone, Debug)]
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
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CreateDataBaseSQL {}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DropDataBaseSQL {}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CreateSchemaSQL {}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DropSchemaSQL {}
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CreateTableSQL {}
#[derive(Clone, Debug)]
pub struct DropTableSQL {}
#[derive(Clone, Debug)]
pub struct CreateViewSQL {}
#[derive(Clone, Debug)]
pub struct DropViewSQL {}
#[derive(Clone, Debug)]
pub struct CreateIndexSQL {}
#[derive(Clone, Debug)]
pub struct DropIndexSQL {}
#[derive(Clone, Debug)]
pub struct CreateCommentSQL {}
#[derive(Clone, Debug)]
pub struct DropCommentSQL {}
#[derive(Clone, Debug)]
pub struct CreateConstriantSQL {}
#[derive(Clone, Debug)]
pub struct DropConstriantSQL {}
#[derive(Clone, Debug)]
pub struct CreateColumnSQL {}
#[derive(Clone, Debug)]
pub struct DropColumnSQL {}
#[derive(Clone, Debug)]
pub struct AlterTableSQL {}
#[derive(Clone, Debug)]
pub struct AlterColumnSQL {}
