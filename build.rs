extern crate embed_resource;

fn main() {
    embed_resource::compile("./file-tag.rc", embed_resource::NONE);
}