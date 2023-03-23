use sea_orm_migration::prelude::*;

use super::{
    moople_opt_id,
    moople_ty::{moople_id, moople_id_pkey},
};

#[derive(Debug, Clone)]
pub struct MoopleTableMeta {
    pub name: DynIden,
    pub key: DynIden,
}

impl MoopleTableMeta {
    pub fn new(name: impl IntoIden, key: impl IntoIden) -> Self {
        Self {
            name: name.into_iden(),
            key: key.into_iden(),
        }
    }

    pub fn create(&self, columns: impl IntoIterator<Item = ColumnDef>) -> TableCreateStatement {
        let mut tbl = Table::create()
            .table(self.name.clone())
            .col(&mut moople_id_pkey(self.key.clone()))
            .to_owned();

        for mut col in columns {
            tbl.col(&mut col);
        }

        tbl
    }

    pub fn create_with_ref<'a>(
        &self,
        columns: impl IntoIterator<Item = ColumnDef>,
        ref_tbls: impl IntoIterator<Item = &'a Ref>,
    ) -> TableCreateStatement {
        let mut tbl = self.create(columns);
        for r in ref_tbls {
            let (from_col, ref_tbl) = r.get_val();
            tbl.foreign_key(&mut self.create_foreign_key(ref_tbl, from_col.clone()));
            match r {
                Ref::Optional(_, _) => tbl.col(&mut moople_opt_id(from_col)),
                Ref::Ownership(_, _) => tbl.col(&mut moople_id(from_col)),
            };
        }

        tbl
    }

    fn foreign_key_name(&self, to_table: &Self) -> String {
        format!("fk_{}_{}", self.name.to_string(), to_table.name.to_string()).to_lowercase()
    }

    pub fn create_foreign_key(
        &self,
        to_table: &Self,
        from_col: impl IntoIden,
    ) -> ForeignKeyCreateStatement {
        let name = self.foreign_key_name(to_table);
        ForeignKey::create()
            .name(&name)
            .from_tbl(self.name.clone())
            .from_col(from_col)
            .to_tbl(to_table.name.clone())
            .to_col(to_table.key.clone())
            .to_owned()
    }

    pub fn drop_foreign_key(&self, to_table: &Self) -> ForeignKeyDropStatement {
        let name = self.foreign_key_name(to_table);
        ForeignKey::drop()
            .table(self.name.clone())
            .name(&name)
            .to_owned()
    }
}

#[derive(Debug)]
pub enum Ref {
    Ownership(DynIden, MoopleTableMeta),
    Optional(DynIden, MoopleTableMeta),
}

impl Ref {
    pub fn ownership(iden: impl IntoIden, table: &MoopleTbl) -> Self {
        Self::Ownership(iden.into_iden(), table.meta.clone())
    }

    pub fn opt(iden: impl IntoIden, table: &MoopleTbl) -> Self {
        Self::Optional(iden.into_iden(), table.meta.clone())
    }

    fn get_val(&self) -> (DynIden, &MoopleTableMeta) {
        match self {
            Ref::Optional(col, tbl) | Ref::Ownership(col, tbl) => (col.clone(), tbl),
        }
    }
}

#[derive(Debug)]
pub struct MoopleTbl {
    meta: MoopleTableMeta,
    columns: Vec<ColumnDef>,
    refs: Vec<Ref>,
}

impl MoopleTbl {
    pub fn new<T: IntoIden>(
        name: T,
        key: T,
        columns: impl IntoIterator<Item = ColumnDef>,
        refs: impl IntoIterator<Item = Ref>,
    ) -> Self {
        Self {
            meta: MoopleTableMeta::new(name, key),
            columns: columns.into_iter().collect(),
            refs: refs.into_iter().collect(),
        }
    }
    pub async fn drop_fk(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        for (_, tbl) in self.refs.iter().map(Ref::get_val) {
            if let Err(err) = manager
                .drop_foreign_key(self.meta.drop_foreign_key(tbl))
                .await
            {
                println!("Unable to delete fk: {:?}", err);
                
            }
        }
        Ok(())
    }

    pub async fn drop_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        if !manager.has_table(self.meta.name.to_string()).await? {
            return Ok(());
        }

        manager
            .drop_table(Table::drop().table(self.meta.name.clone()).to_owned())
            .await?;

        Ok(())
    }

    pub async fn create_table(&self, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
        manager
            .create_table(
                self.meta
                    .create_with_ref(self.columns.clone(), self.refs.iter()),
            )
            .await?;

        Ok(())
    }
}
