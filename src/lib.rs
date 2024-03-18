use bash_builtins::variables::find_as_string;
use bash_builtins::{builtin_metadata, Args, Builtin, BuiltinOptions, Result};
use core_affinity::get_core_ids;
use std::io::{self, BufWriter, Write};

builtin_metadata!(
    name = "cpubind",
    create = CpuBind::default,
    short_doc = "cpubind [-i identifier]",
    long_doc = "
    Prints information about the task and it's cpu affinity

    Options:
        -i identifier used to identify that particular task
    ",
);

#[derive(BuiltinOptions)]
enum Opt {
    #[opt = 'i']
    Identifier(String),
}

#[derive(Default)]
struct CpuBind;

/// Gets the variable named var if it exists and return its value
/// or an empty string if it does not exists.
fn get_variable_string(var: &str) -> String {
    let var_ostring = find_as_string(var);
    if let Some(var_str) = var_ostring.as_ref().and_then(|v| v.to_str().ok()) {
        var_str.to_string()
    } else {
        "".to_string()
    }
}

impl Builtin for CpuBind {
    fn call(&mut self, args: &mut Args) -> Result<()> {
        let mut identifier: String = "".to_string();

        // managing options argument if any - none is ok
        if !args.is_empty() {
            for opt in args.options() {
                match opt? {
                    Opt::Identifier(s) => identifier = s,
                };
            }
        }

        // It is an error if we receive free arguments.
        args.finished()?;

        let core_ids = match get_core_ids() {
            Some(core_ids) => core_ids,
            None => Vec::new(),
        };

        let slurm_job_id = get_variable_string("SLURM_JOB_ID");
        let slurm_procid = get_variable_string("SLURM_PROCID");
        let slurm_localid = get_variable_string("SLURM_LOCALID");

        let slurm_str = format!("{identifier} - {slurm_job_id} - {slurm_procid} - {slurm_localid}");

        let hostname = match hostname::get()?.into_string() {
            Ok(string) => string,
            Err(_) => "".to_string(),
        };

        // using write!() and writeln!() instead of println!() to avoid
        // panicking if stdout is closed.
        let stdout_handle = io::stdout();
        let mut output = BufWriter::new(stdout_handle.lock());

        write!(&mut output, "{hostname} - {slurm_str} - cpu affinity:")?;
        for core in core_ids.into_iter() {
            write!(&mut output, " {}", core.id)?
        }
        writeln!(&mut output)?;

        Ok(())
    }
}
