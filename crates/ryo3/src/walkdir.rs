use std::path::Path;

use ::walkdir as walkdir_rs;
use pyo3::prelude::*;

use crate::fs::fspath::PathLike;

#[pyclass(name = "WalkDirEntry")]
#[derive(Clone, Debug)]
pub struct PyWalkDirEntry {
    de: walkdir_rs::DirEntry,
}

#[pymethods]
impl PyWalkDirEntry {
    #[getter]
    fn path(&self) -> String {
        self.de.path().to_str().unwrap().to_string()
    }

    #[getter]
    fn file_name(&self) -> String {
        self.de.file_name().to_str().unwrap().to_string()
    }

    #[getter]
    fn depth(&self) -> usize {
        self.de.depth()
    }

    fn __str__(&self) -> String {
        self.de.path().to_str().unwrap().to_string()
    }

    fn __repr__(&self) -> String {
        let s = self.de.path().to_str().unwrap().to_string();
        format!("WalkDirEntry({:?})", s)
    }
}

impl From<walkdir_rs::DirEntry> for PyWalkDirEntry {
    fn from(de: walkdir_rs::DirEntry) -> Self {
        Self { de: de }
    }
}

#[pyclass(name = "WalkdirGen")]
pub struct PyWalkdirGen {
    iter: walkdir_rs::IntoIter,
    files: bool,
    dirs: bool,
}

#[pymethods]
impl PyWalkdirGen {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyWalkDirEntry> {
        while let Some(Ok(entry)) = slf.iter.next() {
            if entry.file_type().is_file() && slf.files {
                return Some(PyWalkDirEntry::from(entry));
            } else if entry.file_type().is_dir() && slf.dirs {
                return Some(PyWalkDirEntry::from(entry));
            }
        }
        None
    }
}

#[pyclass(name = "FspathsGen")]
pub struct PyFspathsGen {
    iter: walkdir_rs::IntoIter,
    files: bool,
    dirs: bool,
}

#[pymethods]
impl PyFspathsGen {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<String> {
        while let Some(Ok(entry)) = slf.iter.next() {
            if entry.file_type().is_file() && slf.files {
                let path = entry.path();
                let path = path.to_str().unwrap().to_string();
                return Some(path);
            } else if entry.file_type().is_dir() && slf.dirs {
                let path = entry.path();
                let path = path.to_str().unwrap().to_string();
                return Some(path);
            }
        }
        None
    }
}

impl From<walkdir_rs::WalkDir> for PyWalkdirGen {
    fn from(wd: walkdir_rs::WalkDir) -> Self {
        let wdit = wd.into_iter();
        Self {
            iter: wdit,
            files: true,
            dirs: true,
        }
    }
}

impl From<walkdir_rs::WalkDir> for PyFspathsGen {
    fn from(wd: walkdir_rs::WalkDir) -> Self {
        let wdit = wd.into_iter();
        Self {
            // wd: wd,
            iter: wdit,
            files: true,
            dirs: true,
        }
    }
}

fn build_walkdir(
    path: &Path,
    contents_first: Option<bool>, // false
    min_depth: Option<usize>,     // default 0
    max_depth: Option<usize>,     // default None
    follow_links: Option<bool>,   // default false
    same_file_system: Option<bool>,
) -> walkdir_rs::WalkDir {
    let mut wd = walkdir_rs::WalkDir::new(path)
        .contents_first(contents_first.unwrap_or(false))
        .follow_links(follow_links.unwrap_or(false))
        .same_file_system(same_file_system.unwrap_or(false))
        .min_depth(min_depth.unwrap_or(0));
    if let Some(max_depth) = max_depth {
        wd = wd.max_depth(max_depth);
    }
    wd
}

#[pyfunction]
#[pyo3(signature = (path = None, *, files = true, dirs = true, contents_first = false, min_depth = 0, max_depth = None, follow_links = false, same_file_system = false))]
pub fn walkdir(
    path: Option<PathLike>,
    files: Option<bool>,            // true
    dirs: Option<bool>,             // true
    contents_first: Option<bool>,   // false
    min_depth: Option<usize>,       // default 0
    max_depth: Option<usize>,       // default None
    follow_links: Option<bool>,     // default false
    same_file_system: Option<bool>, // default false
) -> PyResult<PyWalkdirGen> {
    let wd = build_walkdir(
        path.unwrap_or(PathLike::Str(String::from("."))).as_ref(),
        contents_first,
        min_depth,
        max_depth,
        follow_links,
        same_file_system,
    );
    Ok(PyWalkdirGen {
        iter: wd.into_iter(),
        files: files.unwrap_or(true),
        dirs: dirs.unwrap_or(true),
    })
}

#[pyfunction]
#[pyo3(signature = (path = None, *, files = true, dirs = true, contents_first = false, min_depth = 0, max_depth = None, follow_links = false, same_file_system = false))]
pub fn fspaths(
    path: Option<PathLike>,
    files: Option<bool>,            // true
    dirs: Option<bool>,             // true
    contents_first: Option<bool>,   // false
    min_depth: Option<usize>,       // default 0
    max_depth: Option<usize>,       // default None
    follow_links: Option<bool>,     // default false
    same_file_system: Option<bool>, // default false
) -> PyResult<PyFspathsGen> {
    let wd = build_walkdir(
        path.unwrap_or(PathLike::Str(String::from("."))).as_ref(),
        contents_first,
        min_depth,
        max_depth,
        follow_links,
        same_file_system,
    );
    Ok(PyFspathsGen {
        iter: wd.into_iter(),
        files: files.unwrap_or(true),
        dirs: dirs.unwrap_or(true),
    })
}

pub fn madd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyWalkDirEntry>()?;
    m.add_class::<PyWalkdirGen>()?;
    m.add_class::<PyFspathsGen>()?;
    m.add_function(wrap_pyfunction!(self::walkdir, m)?)?;
    m.add_function(wrap_pyfunction!(self::fspaths, m)?)?;
    Ok(())
}
