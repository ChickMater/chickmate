use directories::ProjectDirs;
lazy_static! {
    pub static ref PROJ_DIR: ProjectDirs =
        match ProjectDirs::from("", crate_authors!(), crate_name!()) {
            Some(pdir) => pdir,
            None => panic!("Cannot locate projects directory"),
        };
}
