#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{ffi::CString, path::PathBuf};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn get_3mf_model_from_file(filepath: PathBuf, o_model: &mut Lib3MF_Model) -> Lib3MFResult {
    let filepath_cstring = CString::new(filepath.to_string_lossy().as_bytes()).unwrap();
    let filetype_cstring = CString::new("3mf").unwrap();
    let mut reader: Lib3MF_Reader = std::ptr::null_mut();

    unsafe {
        let res = lib3mf_createmodel(o_model);
        if res != 0 {
            return res;
        }

        let res = lib3mf_model_queryreader(*o_model, filetype_cstring.as_ptr(), &mut reader);
        if res != 0 {
            return res;
        }

        lib3mf_reader_readfromfile(reader, filepath_cstring.as_ptr())
    }
}

pub fn write_3mf_to_file(filepath: PathBuf, model: &mut Lib3MF_Model) -> Lib3MFResult {
    let filepath_cstring = CString::new(filepath.to_string_lossy().as_bytes()).unwrap();

    unsafe {
        let mut writer: Lib3MFHandle = std::ptr::null_mut();
        let writer_type = CString::new("3mf").unwrap();
        let res = lib3mf_model_querywriter(*model, writer_type.as_ptr(), &mut writer);
        if res != 0 {
            return res;
        }

        let res = lib3mf_writer_writetofile(writer, filepath_cstring.as_ptr());
        lib3mf_release(writer);

        res
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn expected_version() {
        let mut major: Lib3MF_uint32 = 0;
        let mut minor: Lib3MF_uint32 = 0;
        let mut micro: Lib3MF_uint32 = 0;

        unsafe {
            lib3mf_getlibraryversion(&mut major, &mut minor, &mut micro);
        }
        assert_eq!(major, 2, "Major version does not match");
        assert_eq!(minor, 4, "Minor version does not match");
        assert_eq!(micro, 1, "Micro version does not match");
    }

    #[test]
    fn can_read_3mf_file() {
        let filepath = PathBuf::from("data/mgx-iron_giant_single.3mf")
            .canonicalize()
            .unwrap();

        let mut model: Lib3MF_Model = std::ptr::null_mut();
        let result = get_3mf_model_from_file(filepath, &mut model);
        assert_eq!(result, 0, "Read from file failed");

        let mut number_of_resources: u64 = 0;
        let mut number_of_mesh = 0;

        unsafe {
            let mut resource_it: Lib3MF_ResourceIterator = std::ptr::null_mut();
            let result = lib3mf_model_getresources(model, &mut resource_it);
            assert_eq!(result, 0);
            assert!(!resource_it.is_null(), "Iterator is null");

            let result = lib3mf_resourceiterator_count(resource_it, &mut number_of_resources);
            assert_eq!(result, 0, "Counting iterator is not successful");

            let mut obj_it: Lib3MF_ResourceIterator = std::ptr::null_mut();
            let result = lib3mf_model_getobjects(model, &mut obj_it);
            assert_eq!(result, 0, "Getting objects is not successful");

            let mut obj: Lib3MF_Object = std::ptr::null_mut();
            let mut has_next: bool = false;
            loop {
                // all iterators that belongs to a child of Resource is moved with the same function
                // lib3mf_resourceiterator_movenext
                let result = lib3mf_resourceiterator_movenext(obj_it, &mut has_next);
                if result != 0 {
                    break;
                }

                if !has_next {
                    break;
                }

                let mut is_mesh: bool = false;

                let result = lib3mf_objectiterator_getcurrentobject(obj_it, &mut obj);
                if result == 0 {
                    let result = lib3mf_object_ismeshobject(obj, &mut is_mesh);
                    assert_eq!(result, 0, "IsMesh check failed");
                    if is_mesh {
                        number_of_mesh += 1;
                    }
                }
            }

            lib3mf_release(obj);
            lib3mf_release(obj_it);
            lib3mf_release(resource_it);
            lib3mf_release(model);
        }

        assert_eq!(number_of_resources, 2, "Incorrect number of resources");
        assert_eq!(number_of_mesh, 1, "Incorrect number of meshes");
    }

    #[test]
    fn roundtrip_3mf_file() {
        let filepath = PathBuf::from("data/mgx-iron_giant_single.3mf")
            .canonicalize()
            .unwrap();

        let mut model: Lib3MFHandle = std::ptr::null_mut();
        let result = get_3mf_model_from_file(filepath, &mut model);
        assert_eq!(result, 0);

        let write_filepath = PathBuf::from("target/out.3mf");
        let write_result = write_3mf_to_file(write_filepath, &mut model);
        assert_eq!(write_result, 0);
    }
}
