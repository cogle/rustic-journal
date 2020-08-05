
pub const SD_JOURNAL_LOCAL_ONLY: c_int = 1 << 0;
pub const SD_JOURNAL_RUNTIME_ONLY: c_int = 1 << 1;
pub const SD_JOURNAL_SYSTEM: c_int = 1 << 2;
pub const SD_JOURNAL_CURRENT_USER: c_int = 1 << 3;
pub const SD_JOURNAL_OS_ROOT: c_int = 1 << 4;
pub const SD_JOURNAL_ALL_NAMESPACES: c_int = 1 << 5;
pub const SD_JOURNAL_INCLUDE_DEFAULT_NAMESPACE: c_int = 1 << 6;

pub enum sd_journal {}

extern "C" {
    pub fn sd_journal_open(ret: mut* mut* sd_journal, flags: c_int) -> c_int;
}