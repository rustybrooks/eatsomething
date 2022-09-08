use std::env;

use deadpool_postgres::tokio_postgres::types::ToSql;
use deadpool_postgres::tokio_postgres::{NoTls, Row};
use deadpool_postgres::{tokio_postgres, Manager, ManagerConfig, Object, Pool, RecyclingMethod};

pub async fn create_pool() -> Pool {
    let mut pg_config = tokio_postgres::Config::new();
    pg_config.user(env::var("PG_USER").unwrap().as_str());
    pg_config.password(env::var("PG_PASS").unwrap().as_str());
    pg_config.port(
        env::var("PG_PORT")
            .unwrap()
            .as_str()
            .parse::<u16>()
            .unwrap(),
    );
    pg_config.host(env::var("PG_HOST").unwrap().as_str());
    pg_config.dbname("eatsomething");
    println!("{:?}", pg_config);
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    let pool = Pool::builder(mgr).max_size(5).build().unwrap();
    for i in 1..10 {
        let client = pool.get().await.unwrap();
        let stmt = client.prepare_cached("SELECT 1 + $1").await.unwrap();
        let rows = client.query(&stmt, &[&i]).await.unwrap();
        let value: i32 = rows[0].get(0);
        assert_eq!(value, i + 1);
    }

    return pool;
}

pub struct Sql {
    pool: Pool,
}

impl Sql {
    pub async fn new() -> Sql {
        Sql {
            pool: create_pool().await,
        }
    }

    async fn client(&self) -> Object {
        return self.pool.get().await.unwrap();
    }

    pub async fn execute(&self, query: &str) -> Vec<Row> {
        let client = self.client().await;
        let stmt = client.prepare_cached(query).await.unwrap();
        let rows = client.query(&stmt, &[]).await.unwrap();
        return rows;
    }

    pub async fn select(&self, query: &str) -> Vec<Row> {
        let client = self.client().await;
        let stmt = client.prepare_cached(query).await.unwrap();
        let rows = client.query(&stmt, &[]).await.unwrap();
        return rows;
    }

    pub async fn select_one(&self, query: &str) -> Result<Row, &str> {
        let mut rows = self.select(query).await;
        return if rows.len() == 1 {
            return match rows.pop() {
                Some(foo) => Ok(foo),
                None => Err("foo"),
            };
        } else {
            Err("expected one row, got something else")
        };
    }

    /*
        use itertools::Itertools; // For tuples() and format_with()

    let params: Vec<_> = members
        .iter()
        .flat_map(|row| [&row.id as &(dyn ToSql + Sync), &row.userid, &row.usertype])
        .collect();
    let query = format!(
        "insert into tablename(id, userid, usertype) values {}",
        (0..params.len())
            .tuples()
            .format_with(", ", |(i, j, k), f| {
                f(&format_args!("(${i}, ${j}, ${k})"))
            }),
    );
         */
    pub async fn insert<T>(&self, table: &str, parameters: Struct<T>) {
        let client = self.client().await;
        let query = format!("insert into {table}() values ()").to_string();
        let mut query_params = Vec::<&(dyn ToSql + Sync)>::new();
        client.execute(&query, &query_params[..]);
    }
}

pub struct MigrationStatement {
    statement: String,
    message: String,
}

pub struct Migration {
    // registry: { [id: string]: Migration } = {};
    version: u32,
    message: String,
    statements: Vec<MigrationStatement>,
    logs: Vec<String>,
}

struct MigrationInsert {
    version_pre: i32,
    version_post: i32,
    migration_datetime: String,
}

impl Migration {
    fn log(&mut self, msg: string) {
        self.logs.push(msg);
    }

    async fn migrate(&mut self, sql: &Sql, initial: bool) {
        sql.execute(
            "
            create table if not exists migrations (
                migration_id serial primary key,
                migration_datetime timestamp,
                version_pre int,
                version_post int
            )
        ",
        )
        .await;
        let res = sql
            .select_one("select max(version_post) as version from migrations")
            .await;
        let version: i32 = res.unwrap().get("version");
        println!("{}", version);

        let todo = self
            .statements
            .into_iter()
            .filter(|m| initial || m.version > version)
            .collect();

        self.log(format!("Version = {version} todo={todo} initial={initial}"));

        let mut version_post = version;

        for migration in todo.iter() {
            for statement in migration.statements.iter() {
                sql.execute(statement).await;
            }
            if migration.version > version_post {
                version_post = migration.version;
            }
        }

        sql.insert(
            "migrations",
            MigrationInsert {
                migration_datetime: format!(""),
                version_pre: version,
                version_post: versionPost,
            },
        );
    }

    fn _add_statement(&mut self, statement: MigrationStatement) {
        self.statements.push(statement)
    }
}
