use crate::core::dto::ColumnValue;

pub trait Query {
    fn build(&self) -> String;
}

pub struct DataQueryBuilder {
    table: String,
    columns: Vec<String>,
}

impl DataQueryBuilder {
    pub fn new(table: &str, columns: &Vec<String>) -> Self {
        Self {
            table: table.to_string(),
            columns: columns.clone(),
        }
    }
}

impl Query for DataQueryBuilder {
    fn build(&self) -> String {
        let mut query: String = String::from("select ");

        for column in &self.columns {
            query.push_str(&format!("[{}], ", column));
        }

        // Remove last ,
        query.remove(query.len() - 2);
        query.push_str(&format!("from {}", self.table));

        return query;
    }
}

pub struct DataInsertQueryBuilder {
    table: String,
    columns: Vec<String>,
    data: Vec<ColumnValue>,
}

impl DataInsertQueryBuilder {
    pub fn new(table: &str, columns: &Vec<String>, data: Vec<ColumnValue>) -> Self {
        Self {
            table: table.to_string(),
            columns: columns.clone(),
            data,
        }
    }
}

impl Query for DataInsertQueryBuilder {
    fn build(&self) -> String {
        let mut query: String = format!("insert into {}(", self.table);

        for column in &self.columns {
            query.push_str(&format!("[{}], ", column));
        }

        // Remove last , and space
        query.remove(query.len() - 1);
        query.remove(query.len() - 1);
        query.push_str(") values (");

        // Add data
        for data in &self.data {
            query.push_str(&format!("{}, ", data));
        }

        query.remove(query.len() - 2);
        query.push_str(");");

        query
    }
}
