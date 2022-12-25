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

pub struct CreateDataBaseSQL {}

pub struct DropDataBaseSQL {}

pub struct CreateSchemaSQL {}

pub struct DropSchemaSQL {}

pub struct CreateTableSQL {}

pub struct DropTableSQL {}

pub struct CreateViewSQL {}

pub struct DropViewSQL {}

pub struct CreateIndexSQL {}

pub struct DropIndexSQL {}

pub struct CreateCommentSQL {}

pub struct DropCommentSQL {}

pub struct CreateConstriantSQL {}
pub struct DropConstriantSQL {}

pub struct CreateColumnSQL {}
pub struct DropColumnSQL {}

pub struct AlterTableSQL {}
pub struct AlterColumnSQL {}
