use crate::*;

pub fn copy_file_preserved<P1: AsRef<Path>, P2: AsRef<Path>>(src: P1, dst: P2) -> Result<(), io::Error> {
    fs::copy(src, dst)?;
    
    let src = src.as_ref();
    let dst = dst.as_ref();
    let metadata = src.metadata()?;
    let src_c = CString::new(src.as_os_str().as_bytes())?;
    let dst_c = CString::new(dst.as_os_str().as_bytes())?;
    
    unsafe {
        libc::chown(
            dst_c.as_ptr(),
            metadata.uid(),
            metadata.gid(),
        );
        
        libc::chmod(dst_c.as_ptr(), metadata.mode());
        
        let times = [
            libc::timespec {
                tv_sec: metadata.atime(),
                tv_nsec: metadata.atime_nsec(),
            },
            libc::timespec {
                tv_sec: metadata.mtime(),
                tv_nsec: metadata.mtime_nsec(),
            },
        ];
        libc::utimensat(
            libc::AT_FDCWD,
            dst_c.as_ptr(),
            times.as_ptr(),
            0,
        );
    }
    
    copy_xattrs(&src_c, &dst_c)?;
    copy_selinux_context(&src_c, &dst_c)?;
    Ok(())
}

fn copy_xattrs(src: &CStr, dst: &CStr) -> std::io::Result<()> {
    unsafe {
        let size = libc::listxattr(
            src.as_ptr(),
            std::ptr::null_mut(),
            0,
        );
        
        if size <= 0 {
            return Ok(());
        }
        
        let mut list = vec![0u8; size as usize];
        libc::listxattr(
            src.as_ptr(),
            list.as_mut_ptr() as *mut i8,
            size as usize,
        );
        
        let mut pos = 0;
        while pos < list.len() {
            let name_start = pos;
            while pos < list.len() && list[pos] != 0 {
                pos += 1;
            }
            
            if pos <= name_start {
                break;
            }
            
            let name = CStr::from_bytes_with_nul(&list[name_start..=pos]).unwrap();
            
            let value_size = libc::getxattr(
                src.as_ptr(),
                name.as_ptr(),
                std::ptr::null_mut(),
                0,
            );
            
            if value_size > 0 {
                let mut value = vec![0u8; value_size as usize];
                libc::getxattr(
                    src.as_ptr(),
                    name.as_ptr(),
                    value.as_mut_ptr() as *mut libc::c_void,
                    value_size as usize,
                );
                
                libc::setxattr(
                    dst.as_ptr(),
                    name.as_ptr(),
                    value.as_ptr() as *const libc::c_void,
                    value.len(),
                    0,
                );
            }
            
            pos += 1;
        }
    }
    
    Ok(())
}

fn copy_selinux_context(src: &CStr, dst: &CStr) -> std::io::Result<()> {
    unsafe {
        let context_name = CString::new("security.selinux").unwrap();
        let size = libc::getxattr(
            src.as_ptr(),
            context_name.as_ptr(),
            std::ptr::null_mut(),
            0,
        );
        
        if size > 0 {
            let mut context = vec![0u8; size as usize];
            libc::getxattr(
                src.as_ptr(),
                context_name.as_ptr(),
                context.as_mut_ptr() as *mut libc::c_void,
                size as usize,
            );
            
            libc::setxattr(
                dst.as_ptr(),
                context_name.as_ptr(),
                context.as_ptr() as *const libc::c_void,
                context.len(),
                0,
            );
        }
    }
    
    Ok(())
}
