use super::raft_rs::config::PyRaftConfig;
use pyo3::prelude::*;
use raftify::Config;

#[derive(Clone)]
#[pyclass(name = "Config")]
pub struct PyConfig {
    #[pyo3(get, set)]
    pub raft_config: PyRaftConfig,

    #[pyo3(get, set)]
    pub log_dir: String,
    #[pyo3(get, set)]
    pub compacted_log_dir: String,
    #[pyo3(get, set)]
    pub compacted_log_size_threshold: i32,
    #[pyo3(get, set)]
    pub snapshot_interval: f32,
    #[pyo3(get, set)]
    pub tick_interval: f32,
    #[pyo3(get, set)]
    pub lmdb_map_size: i32,
    #[pyo3(get, set)]
    pub cluster_id: String,
    #[pyo3(get, set)]
    pub terminate_on_remove: bool,
    #[pyo3(get, set)]
    pub conf_change_request_timeout: f32,
}

#[pymethods]
impl PyConfig {
    #[new]
    pub fn new(
        raft_config: PyRaftConfig,
        log_dir: Option<String>,
        compacted_log_dir: Option<String>,
        compacted_log_size_threshold: Option<i32>,
        snapshot_interval: Option<f32>,
        tick_interval: Option<f32>,
        lmdb_map_size: Option<i32>,
        cluster_id: Option<String>,
        terminate_on_remove: Option<bool>,
        conf_change_request_timeout: Option<f32>,
    ) -> Self {
        let cfg = Config::default();

        let log_dir = log_dir.unwrap_or(cfg.log_dir);
        let compacted_log_dir = compacted_log_dir.unwrap_or(cfg.compacted_log_dir);
        let compacted_log_size_threshold =
            compacted_log_size_threshold.unwrap_or(cfg.compacted_log_size_threshold);
        let snapshot_interval = snapshot_interval.unwrap_or(cfg.snapshot_interval);
        let tick_interval = tick_interval.unwrap_or(cfg.tick_interval);
        let lmdb_map_size = lmdb_map_size.unwrap_or(cfg.lmdb_map_size);
        let cluster_id = cluster_id.unwrap_or(cfg.cluster_id);
        let terminate_on_remove = terminate_on_remove.unwrap_or(cfg.terminate_on_remove);
        let conf_change_request_timeout =
            conf_change_request_timeout.unwrap_or(cfg.conf_change_request_timeout);

        Self {
            raft_config,
            log_dir,
            compacted_log_dir,
            compacted_log_size_threshold,
            snapshot_interval,
            tick_interval,
            lmdb_map_size,
            cluster_id,
            terminate_on_remove,
            conf_change_request_timeout,
        }
    }
}

impl From<PyConfig> for Config {
    fn from(config: PyConfig) -> Self {
        Self {
            log_dir: config.log_dir,
            compacted_log_dir: config.compacted_log_dir,
            compacted_log_size_threshold: config.compacted_log_size_threshold,
            snapshot_interval: config.snapshot_interval,
            tick_interval: config.tick_interval,
            lmdb_map_size: config.lmdb_map_size,
            cluster_id: config.cluster_id,
            terminate_on_remove: config.terminate_on_remove,
            conf_change_request_timeout: config.conf_change_request_timeout,
            raft_config: config.raft_config.inner,
        }
    }
}
