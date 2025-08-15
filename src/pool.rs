use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::{Config, NoTls};

pub enum PoolOrConfig {
    Pool(Pool),
    Config(Config),
    Manager(Manager),
    DatabaseUrl(String),
}


/// Creates a [`deadpool_postgres::Pool`] from different possible input types.
///
/// # Parameters
///
/// - `pool_or_config`: A [`PoolOrConfig`] enum specifying how to construct the pool:
///   - [`PoolOrConfig::Pool`] — Returns the given pool directly.
///   - [`PoolOrConfig::Config`] — Creates a pool from an existing [`tokio_postgres::Config`] object.
///   - [`PoolOrConfig::Manager`] — Creates a pool from an existing [`deadpool_postgres::Manager`].
///   - [`PoolOrConfig::DatabaseUrl`] — Parses a PostgreSQL connection string into a config and creates a pool.
///
/// # Behavior
///
/// - Uses a maximum pool size of **16** connections (change `MAX_POOL_SIZE` to customize).
/// - Uses [`RecyclingMethod::Fast`] for quick connection reuse without validation.
/// - Any connection errors during pool creation will cause the process to panic (`unwrap()`).
///
/// # Examples
///
/// Creating a pool from a database URL:
/// ```no_run
/// use deadpool_postgres::{Pool};
/// use tokio_postgres::Config;
/// use mycrate::{create_pool, PoolOrConfig};
///
/// let pool: Pool = create_pool(PoolOrConfig::DatabaseUrl(
///     "postgres://user:password@localhost:5432/mydb".into()
/// ));
/// ```
///
/// Creating a pool from an existing `tokio_postgres::Config`:
/// ```no_run
/// use tokio_postgres::Config;
/// use mycrate::{create_pool, PoolOrConfig};
///
/// let mut config = Config::new();
/// config.host("localhost")
///       .user("postgres")
///       .password("password")
///       .dbname("mydb");
///
/// let pool = create_pool(PoolOrConfig::Config(config));
/// ```
///
/// # Panics
///
/// This function will panic if:
/// - The database URL cannot be parsed.
/// - The pool builder fails during creation.
///
/// # See also
/// - [`deadpool_postgres::Pool`]
/// - [`deadpool_postgres::Manager`]
/// - [`tokio_postgres::Config`]
///
pub fn create_pool(pool_or_config: PoolOrConfig) -> Pool {
    const MAX_POOL_SIZE: usize = 16;

    fn build_pool(manager: Manager) -> Pool {
        Pool::builder(manager)
            .max_size(MAX_POOL_SIZE)
            .build()
            .unwrap()
    }

    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };

    match pool_or_config {
        PoolOrConfig::Pool(pool) => pool,
        PoolOrConfig::Config(config) => build_pool(Manager::from_config(config, NoTls, mgr_config)),
        PoolOrConfig::Manager(manager) => build_pool(manager),
        PoolOrConfig::DatabaseUrl(url) => {
            let pg_config: Config = url.parse().unwrap();
            build_pool(Manager::from_config(pg_config, NoTls, mgr_config))
        }
    }
}