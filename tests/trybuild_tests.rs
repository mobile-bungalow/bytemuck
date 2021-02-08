
#[test]
#[cfg(feature = "derive")]
fn derive_tests() {
    use std::{env, path::PathBuf, ffi::OsStr};

    std::env::set_var("TRYBUILD","overwrite");

    {
        let tester = trybuild::TestCases::new();
        tester.pass("tests/trybuild_tests_src/derive.rs");
        tester.compile_fail("tests/trybuild_tests_src/pod_derive_failure.rs");
        tester.compile_fail("tests/trybuild_tests_src/transparentwrapper_failure.rs");
        tester.compile_fail("tests/trybuild_tests_src/nonzeroable_fields.rs");
        tester.compile_fail("tests/trybuild_tests_src/contiguous_derive_failure.rs");
    }


    let source_dir = env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap();
    let trybuild_dir = source_dir.join("tests/trybuild_tests_src");

    for entry in std::fs::read_dir(trybuild_dir).unwrap() {
        let entry = entry.unwrap().path();
        let entry = entry.as_path();
        let extension = entry.extension().map(OsStr::to_str).flatten();
        dbg!(&extension);
        if  extension == Some("stderr") {
            std::fs::remove_file(entry).unwrap();
        }
    }
}
