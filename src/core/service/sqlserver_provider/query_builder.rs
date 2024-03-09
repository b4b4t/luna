use crate::core::dto::ColumnValue;

pub trait Query {
    fn build(&self) -> String;
}

pub struct DataQueryBuilder {
    table: String,
    columns: Vec<String>,
    primary_key: Vec<String>,
    skip: Option<u64>,
    take: Option<u64>,
    predicate: Option<String>,
}

impl DataQueryBuilder {
    pub fn new(
        table: &str,
        columns: &Vec<String>,
        primary_key: Vec<String>,
        skip: Option<u64>,
        take: Option<u64>,
        predicate: Option<&String>,
    ) -> Self {
        Self {
            table: table.to_string(),
            columns: columns.clone(),
            primary_key,
            skip,
            take,
            predicate: match predicate {
                Some(p) => Some(p.clone()),
                None => None,
            },
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

        if self.predicate.is_some() {
            query.push_str(&format!(" where {}", self.predicate.as_ref().unwrap()));
        }

        if self.take.is_some() && self.skip.is_some() {
            // Order by
            query.push_str(" order by ");
            for primary_key in &self.primary_key {
                query.push_str(&format!(" {},", primary_key));
            }
            query.remove(query.len() - 1);

            // Offset and fetch next rows
            query.push_str(&format!(
                " offset {} rows fetch next {} rows only",
                self.skip.as_ref().unwrap(),
                self.take.as_ref().unwrap()
            ));
        }

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
