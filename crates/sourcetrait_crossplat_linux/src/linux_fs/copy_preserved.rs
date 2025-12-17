use crate::*;

#[inline]
pub fn copy_preserved<P1: AsRef<Path>, P2: AsRef<Path>>(src: P1, dst: P2) -> Result<(), io::Error> {
    unix::copy_preserved(src, dst)
}
