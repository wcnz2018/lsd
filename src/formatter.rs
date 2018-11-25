use color::{Colors, Elem, PrecomputedElems};
use icon;
use meta::{Meta, Type};
use std::os::unix::fs::PermissionsExt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use time::Timespec;

const HOURE: u64 = 3600; // 1 HOURE == 3600 seconds
const DAY: u64 = HOURE * 24; // 1 DAY == 25 HOURE

pub struct Formatter {}

impl Formatter {
    pub fn new() -> Formatter {
        Formatter {}
    }

    pub fn format_name(&self, meta: &Meta) -> String {
        let mut content = String::new();

        let color = if meta.metadata.is_dir() {
            Colors[&Elem::Dir]
        } else {
            Colors[&Elem::UnrecognizedFile]
        };

        let mut name = meta.name.clone();
        if meta.metadata.is_dir() {
            name.push('/');
        }

        content = content + icon::from_meta(&meta) + "  " + &name;
        content = color.paint(content).to_string();

        content
    }

    pub fn format_symlink(&self, meta: &Meta) -> String {
        let mut content = String::new();

        if let Type::SymLink(ref target) = meta.node_type {
            let color = Colors[&Elem::Link];
            content += &color.paint(String::from(" ⇒ ") + &color.paint(target).to_string());
        }

        content
    }

    pub fn format_date(&self, meta: &Meta) -> String {
        let modified_time = meta
            .metadata
            .modified()
            .expect("failed to retrieve modified date");

        let now = SystemTime::now();

        let color;
        if modified_time > now - Duration::new(HOURE, 0) {
            color = Colors[&Elem::HourOld];
        } else if modified_time > now - Duration::new(DAY, 0) {
            color = Colors[&Elem::DayOld];
        } else {
            color = Colors[&Elem::Older];
        }

        let modified_time_since_epoch = modified_time
            .duration_since(UNIX_EPOCH)
            .expect("failed to convert modified time to timestamp");
        let time = time::at(Timespec::new(
            modified_time_since_epoch.as_secs() as i64,
            modified_time_since_epoch.subsec_nanos() as i32,
        ));

        color.paint(time.ctime().to_string()).to_string()
    }

    pub fn format_permissions(&self, meta: &Meta) -> String {
        let mut res = String::with_capacity(11);

        let mode = meta.metadata.permissions().mode();

        match meta.node_type {
            Type::File => res += PrecomputedElems[&Elem::File].as_str(),
            Type::Directory => res += PrecomputedElems[&Elem::Dir].as_str(),
            Type::SymLink(_) => res += PrecomputedElems[&Elem::SymLink].as_str(),
        }

        // User Read Permisssions
        match mode & 0o400 {
            0 => res += PrecomputedElems[&Elem::NoAccess].as_str(),
            _ => res += PrecomputedElems[&Elem::Read].as_str(),
        }

        // User Write Permisssions
        match mode & 0o200 {
            0 => res += PrecomputedElems[&Elem::NoAccess].as_str(),
            _ => res += PrecomputedElems[&Elem::Write].as_str(),
        }

        // User Exec Permisssions
        match mode & 0o100 {
            0 => res += PrecomputedElems[&Elem::NoAccess].as_str(),
            _ => res += PrecomputedElems[&Elem::Exec].as_str(),
        }

        // Group Read Permisssions
        match mode & 0o040 {
            0 => res += PrecomputedElems[&Elem::NoAccess].as_str(),
            _ => res += PrecomputedElems[&Elem::Read].as_str(),
        }

        // Group Write Permisssions
        match mode & 0o020 {
            0 => res += PrecomputedElems[&Elem::NoAccess].as_str(),
            _ => res += PrecomputedElems[&Elem::Write].as_str(),
        }

        // Group Exec Permisssions
        match mode & 0o010 {
            0 => res += PrecomputedElems[&Elem::NoAccess].as_str(),
            _ => res += PrecomputedElems[&Elem::Exec].as_str(),
        }

        // Other Read Permisssions
        match mode & 0o040 {
            0 => res += PrecomputedElems[&Elem::NoAccess].as_str(),
            _ => res += PrecomputedElems[&Elem::Read].as_str(),
        }

        // Other Write Permisssions
        match mode & 0o020 {
            0 => res += PrecomputedElems[&Elem::NoAccess].as_str(),
            _ => res += PrecomputedElems[&Elem::Write].as_str(),
        }

        // Other Exec Permisssions
        match mode & 0o010 {
            0 => res += PrecomputedElems[&Elem::NoAccess].as_str(),
            _ => res += PrecomputedElems[&Elem::Exec].as_str(),
        }

        res.to_string()
    }

    pub fn format_user(&self, user_name: &str, max_user_size: usize) -> String {
        if user_name.len() == max_user_size {
            return Colors[&Elem::User].paint(user_name).to_string();
        }

        let mut content = String::with_capacity(max_user_size);

        content += user_name;

        for _ in 0..(max_user_size - user_name.len()) {
            content.push(' ');
        }

        content
    }

    pub fn format_group(&self, group_name: &str, max_group_size: usize) -> String {
        if group_name.len() == max_group_size {
            return Colors[&Elem::Group].paint(group_name).to_string();
        }

        let mut content = String::with_capacity(max_group_size);
        content += group_name;

        for _ in 0..(max_group_size - group_name.len()) {
            content.push(' ');
        }

        content
    }

    pub fn format_size(
        &self,
        meta: &Meta,
        max_value_length: usize,
        max_unit_size: usize,
    ) -> String {
        let mut content = String::with_capacity(max_value_length + max_unit_size + 1);

        for _ in 0..(max_value_length - meta.size_value.len()) {
            content.push(' ');
        }

        content += meta.size_value.as_str();
        content.push(' ');
        content += meta.size_unit.as_str();

        for _ in 0..(max_unit_size - meta.size_unit.len()) {
            content.push(' ');
        }

        if meta.metadata.len() < 10 * 1044 * 1024 {
            Colors[&Elem::FileSmall].paint(content).to_string()
        } else if meta.metadata.len() < 100 * 1044 * 1024 {
            Colors[&Elem::FileMedium].paint(content).to_string()
        } else {
            Colors[&Elem::FileLarge].paint(content).to_string()
        }
    }
}