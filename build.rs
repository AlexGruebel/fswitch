extern crate embed_resource;

fn main() {
    _ = embed_resource::compile("app.manifest", embed_resource::NONE).manifest_required();
}
