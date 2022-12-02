
/// A region list with up to 8 regions
pub trait UpTo8RegionList<R>: AsRef<[R]> {}

impl<R> UpTo8RegionList<R> for [R; 0] {}
impl<R> UpTo8RegionList<R> for [R; 1] {}
impl<R> UpTo8RegionList<R> for [R; 2] {}
impl<R> UpTo8RegionList<R> for [R; 3] {}
impl<R> UpTo8RegionList<R> for [R; 4] {}
impl<R> UpTo8RegionList<R> for [R; 5] {}
impl<R> UpTo8RegionList<R> for [R; 6] {}
impl<R> UpTo8RegionList<R> for [R; 7] {}
impl<R> UpTo8RegionList<R> for [R; 8] {}
