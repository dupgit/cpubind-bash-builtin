use bash_builtins::variables::find_as_string;
use bash_builtins::{builtin_metadata, Args, Builtin, Result};
use core_affinity::get_core_ids;
use std::io::{self, BufWriter, Write};

builtin_metadata!(name = "cpubind", create = CpuBind::default);

#[derive(Default)]
struct CpuBind;

impl Builtin for CpuBind {
    fn call(&mut self, _args: &mut Args) -> Result<()> {
        let stdout_handle = io::stdout();
        let mut output = BufWriter::new(stdout_handle.lock());
        let core_ids = match get_core_ids() {
            Some(core_ids) => core_ids,
            None => Vec::new(),
        };

        let mut slurm_str: String = "".to_string();
        let job_id = find_as_string("SLURM_JOB_ID");
        if let Some(job_id_str) = job_id.as_ref().and_then(|v| v.to_str().ok()) {
            slurm_str = format!("{job_id_str}")
        }

        let mpi_rank = find_as_string("SLURM_PROCID");
        if let Some(mpi_rank_str) = mpi_rank.as_ref().and_then(|v| v.to_str().ok()) {
            slurm_str = format!("{slurm_str} - {mpi_rank_str}")
        }
        let local_id = find_as_string("SLURM_LOCALID");
        if let Some(local_id_str) = local_id.as_ref().and_then(|v| v.to_str().ok()) {
            slurm_str = format!("{slurm_str} - {local_id_str}")
        }

        let hostname = match hostname::get()?.into_string() {
            Ok(string) => string,
            Err(_) => "".to_string(),
        };

        write!(&mut output, "{hostname} - {slurm_str} - cpu affinity:")?;
        for core in core_ids.into_iter() {
            write!(&mut output, " {}", core.id)?
        }
        writeln!(&mut output, "")?;

        Ok(())
    }
}
