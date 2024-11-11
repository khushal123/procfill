use crate::utils;

pub fn start() {
    let internal_dirs = utils::dirutil::get_internal_dir_paths();
    utils::dirutil::remove_dirs(&internal_dirs);
    utils::dirutil::create_dirs(&internal_dirs);
    utils::datautil::create_tables(&internal_dirs[1]);
}
