extern crate embed_resource;

fn main() {
    #[cfg(target_os = "windows")]
    embed_resource::compile("resources/windows/manifest.rc", embed_resource::NONE);
}
