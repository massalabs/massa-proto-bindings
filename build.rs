// Copyright (c) 2023 MASSA LABS <info@massa.net>

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "tonic-build")]
    tonic::build()?;

    Ok(())
}

#[cfg(feature = "tonic-build")]
mod tonic {
    use glob::glob;
    use std::{
        env,
        path::{Path, PathBuf},
    };

    /// This function is responsible for building the Massa protobuf
    pub fn build() -> Result<(), Box<dyn std::error::Error>> {
        let parent_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        // Generate ABI bindings
        let protos_path = parent_dir.join("massa-proto/proto/abis");
        let protos = find_protos(protos_path)?;
        let proto_include_paths = [
            parent_dir.join("massa-proto/proto/abis"),
            parent_dir.join("massa-proto/proto/commons"),
            parent_dir.join("massa-proto/proto/third_party"),
        ];

        tonic_build::configure()
            .build_server(false)
            .build_transport(false)
            .build_client(false)
            .include_file("_abi_includes.rs")
            .out_dir("src/")
            .compile_protos(&protos, &proto_include_paths)
            .map_err(|e| format!("ABI protobuf compilation error: {:?}", e))?;

        // Generate PUBLIC API file descriptor set
        let public_api_path = parent_dir.join("massa-proto/proto/apis/massa/api/v1/public.proto");
        let proto_include_paths = [
            parent_dir.join("massa-proto/proto/apis"),
            parent_dir.join("massa-proto/proto/commons"),
            parent_dir.join("massa-proto/proto/third_party"),
        ];

        tonic_build::configure()
            .build_server(true)
            .build_transport(true)
            .build_client(true)
            .type_attribute(
                ".google.api.HttpRule",
                "#[cfg(not(doctest))]\n\
                 #[allow(dead_code)]\n\
                 pub struct HttpRuleComment{}\n\
                 /// HACK: see docs in [`HttpRuleComment`] ignored in doctest pass",
            )
            .include_file("_api_includes.rs")
            .file_descriptor_set_path("src/api_public.bin")
            .out_dir("src/")
            .compile_protos(&vec![public_api_path], &proto_include_paths)
            .map_err(|e| format!("PUBLIC API protobuf compilation error: {:?}", e))?;

        // Generate PRIVATE API file descriptor set
        let private_api_path = parent_dir.join("massa-proto/proto/apis/massa/api/v1/private.proto");
        let proto_include_paths = [
            parent_dir.join("massa-proto/proto/apis"),
            parent_dir.join("massa-proto/proto/commons"),
            parent_dir.join("massa-proto/proto/third_party"),
        ];

        tonic_build::configure()
            .build_server(true)
            .build_transport(true)
            .build_client(true)
            .type_attribute(
                ".google.api.HttpRule",
                "#[cfg(not(doctest))]\n\
                 #[allow(dead_code)]\n\
                 pub struct HttpRuleComment{}\n\
                 /// HACK: see docs in [`HttpRuleComment`] ignored in doctest pass",
            )
            .include_file("_api_includes.rs")
            .file_descriptor_set_path("src/api_private.bin")
            .out_dir("src/")
            .compile_protos(&vec![private_api_path], &proto_include_paths)
            .map_err(|e| format!("PRIVATE API protobuf compilation error: {:?}", e))?;

        // Generate API bindings
        let protos_path = parent_dir.join("massa-proto/proto/apis");
        let protos = find_protos(protos_path)?;
        let proto_include_paths = [
            parent_dir.join("massa-proto/proto/apis"),
            parent_dir.join("massa-proto/proto/commons"),
            parent_dir.join("massa-proto/proto/third_party"),
        ];

        tonic_build::configure()
            .build_server(true)
            .build_transport(true)
            .build_client(true)
            .type_attribute(
                ".google.api.HttpRule",
                "#[cfg(not(doctest))]\n\
                 #[allow(dead_code)]\n\
                 pub struct HttpRuleComment{}\n\
                 /// HACK: see docs in [`HttpRuleComment`] ignored in doctest pass",
            )
            .include_file("_api_includes.rs")
            .out_dir("src/")
            .compile_protos(&protos, &proto_include_paths)
            .map_err(|e| format!("API protobuf compilation error: {:?}", e))?;

        // Generate COMMONS bindings
        let protos_path = parent_dir.join("massa-proto/proto/commons");
        let protos = find_protos(protos_path)?;
        let proto_include_paths = [parent_dir.join("massa-proto/proto/commons")];

        tonic_build::configure()
            .build_server(false)
            .build_transport(false)
            .build_client(false)
            .include_file("_commons_includes.rs")
            .out_dir("src/")
            .compile_protos(&protos, &proto_include_paths)
            .map_err(|e| format!("COMMONS protobuf compilation error: {:?}", e))?;

        Ok(())
    }

    fn find_protos(dir_path: impl AsRef<Path>) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let glob_pattern = format!("{}/**/*.proto", dir_path.as_ref().display());
        let paths: Vec<_> = glob(&glob_pattern)?.filter_map(Result::ok).collect();

        if paths.is_empty() {
            return Err(format!(
                "no .proto files found in the specified directory: {}",
                dir_path.as_ref().display()
            )
            .into());
        }

        Ok(paths)
    }
}
