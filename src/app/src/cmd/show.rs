use std::io::stdout;

use anyhow::Result;
use clap::{App, Arg, ArgMatches};

use huber_common::config::Config;
use huber_common::di::di_container;
use huber_common::output::factory::FactoryConsole;
use huber_common::output::OutputTrait;

use crate::cmd::CommandTrait;
use crate::service::package::PackageService;
use crate::service::release::{ReleaseService, ReleaseTrait};
use crate::service::ItemOperationTrait;
use huber_common::model::release::VecExtensionTrait;


pub(crate) const CMD_NAME: &str = "show";

pub(crate) struct ShowCmd;

impl ShowCmd {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> CommandTrait<'a, 'b> for ShowCmd {
    fn app(&self) -> App<'a, 'b> {
        App::new(CMD_NAME)
            .visible_alias("s")
            .about("Shows installed packages")
            .args(&vec![
                Arg::with_name("name")
                    .value_name("package name")
                    .help("Package name")
                    .takes_value(true),
                Arg::with_name("all")
                    .short("a")
                    .long("all")
                    .help("Show all the installed versions"),
                Arg::with_name("detail")
                    .short("d")
                    .long("detail")
                    .help("Show the detailed info of release"),
            ])
    }

    fn run(&self, config: &Config, matches: &ArgMatches<'a>) -> Result<()> {
        let container = di_container();
        let release_service = container.get::<ReleaseService>().unwrap();
        let pkg_service = container.get::<PackageService>().unwrap();

        let mut excluded_keys = if matches.is_present("detail") {
            vec![]
        } else {
            vec!["package"]
        };

        if matches.is_present("name") {
            let name = matches.value_of("name").unwrap();

            if !release_service.has(name)? {
                return Err(anyhow!("{} not found", name));
            }

            let pkg = pkg_service.get(name)?;
            let release = release_service.current(&pkg)?;

            if matches.is_present("all") {
                let mut releases = release_service.find(&pkg)?;
                releases.sort_by_version();

                releases = releases.into_iter().map(|it| {
                    if it.current {
                        release_service.current(&it.package).unwrap()
                    } else {
                        it
                    }
                }).collect();

                return output!(config.output_format, .display(
                    stdout(),
                    &releases,
                    None,
                    Some(excluded_keys),
                ));
            }

            return output!(config.output_format, .display(
                stdout(),
                &release,
                None,
                Some(excluded_keys),
            ));
        }

        let mut current_releases = release_service.list()?;
        current_releases.sort_by_version();
        excluded_keys.push("executables");

        let releases = if matches.is_present("all") {
            let mut all_releases = vec![];

            for cr in current_releases.iter() {
                let mut releases = release_service.find(&cr.package)?;
                releases.sort_by_version();

                all_releases.append(&mut releases);
            }

            all_releases
        } else {
            current_releases.sort_by_version();
            current_releases
        };

        output!(config.output_format, .display(
            stdout(),
            &releases,
            None,
            Some(excluded_keys),
        ))
    }
}
