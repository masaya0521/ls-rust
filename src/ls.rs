use std::fs;
use std::os::unix::fs::PermissionsExt;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let dir = if args.len() > 1 { &args[1] } else { "." };

    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;
        let file_type = metadata.file_type();
        let permissions = metadata.permissions();
        let size = metadata.len();

        let modified = metadata
            .modified()
            .unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH);

        let permissions_str = format!("{:o}", permissions.mode() & 0o777);
        let modified_str = format!(
            "{}",
            chrono::DateTime::<chrono::Local>::from(modified).format("%b %e %H:%M")
        );

        print!("{} ", permissions_str);
        print!("{:>3} ", hardlink_count(&metadata));
        print!("{:8} ", user_group(&metadata));
        print!("{:8} ", size);
        print!("{:12} ", modified_str);

        if file_type.is_dir() {
            print!("{:?}", path);
        } else {
            print!("{}", path.file_name().unwrap().to_str().unwrap());
        }

        println!();
    }

    Ok(())
}

fn hardlink_count(metadata: &fs::Metadata) -> u64 {
    metadata.nlink()
}

fn user_group(metadata: &fs::Metadata) -> String {
    use users::{get_group_by_gid, get_user_by_uid};
    let uid = metadata.uid();
    let gid = metadata.gid();
    let user = get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().to_string())
        .unwrap_or_else(|| uid.to_string());
    let group = get_group_by_gid(gid)
        .map(|g| g.name().to_string_lossy().to_string())
        .unwrap_or_else(|| gid.to_string());
    format!("{} {}", user, group)
}
