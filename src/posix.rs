//! Target-independent POSIX baseline primitives.
//!
//! This module owns only the Phase 7.1 path/error contract surface. It does
//! not perform VFS lookup, syscall ABI translation, process current-working-
//! directory storage, descriptor-table work, or target I/O.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PosixError {
    OperationNotPermitted,
    NoEntry,
    Interrupted,
    Io,
    NotExecutable,
    BadDescriptor,
    NoChild,
    Again,
    NoMemory,
    AccessDenied,
    Fault,
    Busy,
    Exists,
    NoDevice,
    NotDirectory,
    IsDirectory,
    InvalidArgument,
    TooManyOpenFiles,
    NotTty,
    NoSpace,
    Pipe,
    Range,
    NameTooLong,
    NotImplemented,
    NotEmpty,
    NotSupported,
}

impl PosixError {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::OperationNotPermitted => "EPERM",
            Self::NoEntry => "ENOENT",
            Self::Interrupted => "EINTR",
            Self::Io => "EIO",
            Self::NotExecutable => "ENOEXEC",
            Self::BadDescriptor => "EBADF",
            Self::NoChild => "ECHILD",
            Self::Again => "EAGAIN",
            Self::NoMemory => "ENOMEM",
            Self::AccessDenied => "EACCES",
            Self::Fault => "EFAULT",
            Self::Busy => "EBUSY",
            Self::Exists => "EEXIST",
            Self::NoDevice => "ENODEV",
            Self::NotDirectory => "ENOTDIR",
            Self::IsDirectory => "EISDIR",
            Self::InvalidArgument => "EINVAL",
            Self::TooManyOpenFiles => "EMFILE",
            Self::NotTty => "ENOTTY",
            Self::NoSpace => "ENOSPC",
            Self::Pipe => "EPIPE",
            Self::Range => "ERANGE",
            Self::NameTooLong => "ENAMETOOLONG",
            Self::NotImplemented => "ENOSYS",
            Self::NotEmpty => "ENOTEMPTY",
            Self::NotSupported => "ENOTSUP",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PathStart {
    Root,
    CurrentWorkingDirectory,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PathLimits {
    pub(crate) max_path_len: usize,
    pub(crate) max_component_len: usize,
    pub(crate) max_components: usize,
}

impl PathLimits {
    pub(crate) const fn new(
        max_path_len: usize,
        max_component_len: usize,
        max_components: usize,
    ) -> Self {
        Self {
            max_path_len,
            max_component_len,
            max_components,
        }
    }
}

pub(crate) const DEFAULT_PATH_LIMITS: PathLimits = PathLimits::new(4096, 255, 64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PathComponent<'a> {
    bytes: &'a [u8],
}

impl<'a> PathComponent<'a> {
    const EMPTY: Self = Self { bytes: b"" };

    pub(crate) const fn bytes(self) -> &'a [u8] {
        self.bytes
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NormalizedPath<'a, const MAX_COMPONENTS: usize> {
    start: PathStart,
    components: [PathComponent<'a>; MAX_COMPONENTS],
    component_count: usize,
    requires_directory: bool,
}

impl<'a, const MAX_COMPONENTS: usize> NormalizedPath<'a, MAX_COMPONENTS> {
    pub(crate) const fn start(self) -> PathStart {
        self.start
    }

    pub(crate) const fn component_count(self) -> usize {
        self.component_count
    }

    pub(crate) fn components(&self) -> &[PathComponent<'a>] {
        &self.components[..self.component_count]
    }

    pub(crate) const fn requires_directory(self) -> bool {
        self.requires_directory
    }
}

pub(crate) fn normalize_path<'a, const MAX_COMPONENTS: usize>(
    path: &'a [u8],
    limits: PathLimits,
) -> Result<NormalizedPath<'a, MAX_COMPONENTS>, PosixError> {
    if path.is_empty() {
        return Err(PosixError::NoEntry);
    }
    if path.len() > limits.max_path_len {
        return Err(PosixError::NameTooLong);
    }
    if contains_nul(path) {
        return Err(PosixError::InvalidArgument);
    }

    let start = if path[0] == b'/' {
        PathStart::Root
    } else {
        PathStart::CurrentWorkingDirectory
    };
    let mut normalized = NormalizedPath {
        start,
        components: [PathComponent::EMPTY; MAX_COMPONENTS],
        component_count: 0,
        requires_directory: false,
    };
    let component_capacity = core::cmp::min(MAX_COMPONENTS, limits.max_components);

    let mut offset = 0;
    while offset < path.len() {
        while offset < path.len() && path[offset] == b'/' {
            offset += 1;
        }
        if offset == path.len() {
            break;
        }

        let component_start = offset;
        while offset < path.len() && path[offset] != b'/' {
            offset += 1;
        }
        let component = &path[component_start..offset];
        if component.len() > limits.max_component_len {
            return Err(PosixError::NameTooLong);
        }

        if component == b"." {
            continue;
        }
        if component == b".." {
            match start {
                PathStart::Root => {
                    if normalized.component_count != 0 {
                        normalized.component_count -= 1;
                    }
                }
                PathStart::CurrentWorkingDirectory => {
                    if normalized.component_count != 0
                        && normalized.components[normalized.component_count - 1].bytes != b".."
                    {
                        normalized.component_count -= 1;
                    } else {
                        push_component(&mut normalized, component, component_capacity)?;
                    }
                }
            }
            continue;
        }

        push_component(&mut normalized, component, component_capacity)?;
    }

    normalized.requires_directory = path.len() > 1
        && path[path.len() - 1] == b'/'
        && !(normalized.start == PathStart::Root && normalized.component_count == 0);

    Ok(normalized)
}

fn contains_nul(path: &[u8]) -> bool {
    let mut index = 0;
    while index < path.len() {
        if path[index] == 0 {
            return true;
        }
        index += 1;
    }
    false
}

fn push_component<'a, const MAX_COMPONENTS: usize>(
    normalized: &mut NormalizedPath<'a, MAX_COMPONENTS>,
    component: &'a [u8],
    component_capacity: usize,
) -> Result<(), PosixError> {
    if normalized.component_count >= component_capacity {
        return Err(PosixError::NameTooLong);
    }
    normalized.components[normalized.component_count] = PathComponent { bytes: component };
    normalized.component_count += 1;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LIMITS: PathLimits = PathLimits::new(64, 8, 4);

    fn normalize(path: &[u8]) -> Result<NormalizedPath<'_, 4>, PosixError> {
        normalize_path(path, TEST_LIMITS)
    }

    fn assert_components(path: &NormalizedPath<'_, 4>, expected: &[&[u8]]) {
        assert_eq!(path.component_count(), expected.len());
        let components = path.components();
        let mut index = 0;
        while index < expected.len() {
            assert_eq!(components[index].bytes(), expected[index]);
            index += 1;
        }
    }

    #[test_case]
    fn posix_error_names_match_baseline_contract() {
        let errors = [
            (PosixError::OperationNotPermitted, "EPERM"),
            (PosixError::NoEntry, "ENOENT"),
            (PosixError::Interrupted, "EINTR"),
            (PosixError::Io, "EIO"),
            (PosixError::NotExecutable, "ENOEXEC"),
            (PosixError::BadDescriptor, "EBADF"),
            (PosixError::NoChild, "ECHILD"),
            (PosixError::Again, "EAGAIN"),
            (PosixError::NoMemory, "ENOMEM"),
            (PosixError::AccessDenied, "EACCES"),
            (PosixError::Fault, "EFAULT"),
            (PosixError::Busy, "EBUSY"),
            (PosixError::Exists, "EEXIST"),
            (PosixError::NoDevice, "ENODEV"),
            (PosixError::NotDirectory, "ENOTDIR"),
            (PosixError::IsDirectory, "EISDIR"),
            (PosixError::InvalidArgument, "EINVAL"),
            (PosixError::TooManyOpenFiles, "EMFILE"),
            (PosixError::NotTty, "ENOTTY"),
            (PosixError::NoSpace, "ENOSPC"),
            (PosixError::Pipe, "EPIPE"),
            (PosixError::Range, "ERANGE"),
            (PosixError::NameTooLong, "ENAMETOOLONG"),
            (PosixError::NotImplemented, "ENOSYS"),
            (PosixError::NotEmpty, "ENOTEMPTY"),
            (PosixError::NotSupported, "ENOTSUP"),
        ];

        let mut index = 0;
        while index < errors.len() {
            let (error, name) = errors[index];
            assert_eq!(error.name(), name);
            index += 1;
        }
    }

    #[test_case]
    fn empty_path_maps_to_enoent() {
        assert_eq!(normalize(b""), Err(PosixError::NoEntry));
    }

    #[test_case]
    fn root_path_has_no_components() {
        let path = normalize(b"////").expect("root path");

        assert_eq!(path.start(), PathStart::Root);
        assert_components(&path, &[]);
        assert!(!path.requires_directory());
    }

    #[test_case]
    fn default_limits_cover_normal_contract_paths() {
        let path =
            normalize_path::<64>(b"/usr/bin/talos", DEFAULT_PATH_LIMITS).expect("default limits");

        assert_eq!(path.start(), PathStart::Root);
        assert_eq!(path.component_count(), 3);
        assert!(!path.requires_directory());
    }

    #[test_case]
    fn repeated_separators_and_dot_components_are_removed() {
        let path = normalize(b"/alpha//./beta").expect("normalized path");

        assert_eq!(path.start(), PathStart::Root);
        assert_components(&path, &[&b"alpha"[..], &b"beta"[..]]);
        assert!(!path.requires_directory());
    }

    #[test_case]
    fn absolute_dot_dot_clamps_above_root() {
        let path = normalize(b"/../alpha/../../").expect("absolute parent clamp");

        assert_eq!(path.start(), PathStart::Root);
        assert_components(&path, &[]);
        assert!(!path.requires_directory());
    }

    #[test_case]
    fn relative_leading_dot_dot_components_are_retained() {
        let path = normalize(b"../../alpha").expect("relative parents");

        assert_eq!(path.start(), PathStart::CurrentWorkingDirectory);
        assert_components(&path, &[&b".."[..], &b".."[..], &b"alpha"[..]]);
        assert!(!path.requires_directory());
    }

    #[test_case]
    fn relative_dot_dot_cancels_previous_normal_component() {
        let path = normalize(b"alpha/beta/../gamma").expect("relative cancellation");

        assert_eq!(path.start(), PathStart::CurrentWorkingDirectory);
        assert_components(&path, &[&b"alpha"[..], &b"gamma"[..]]);
        assert!(!path.requires_directory());
    }

    #[test_case]
    fn dot_path_is_relative_current_directory() {
        let path = normalize(b".").expect("dot path");

        assert_eq!(path.start(), PathStart::CurrentWorkingDirectory);
        assert_components(&path, &[]);
        assert!(!path.requires_directory());
    }

    #[test_case]
    fn trailing_slash_requires_directory_for_non_root_path() {
        let path = normalize(b"alpha/beta/").expect("trailing slash");

        assert_eq!(path.start(), PathStart::CurrentWorkingDirectory);
        assert_components(&path, &[&b"alpha"[..], &b"beta"[..]]);
        assert!(path.requires_directory());
    }

    #[test_case]
    fn relative_current_directory_with_trailing_slash_requires_directory() {
        let path = normalize(b"./").expect("dot slash");

        assert_eq!(path.start(), PathStart::CurrentWorkingDirectory);
        assert_components(&path, &[]);
        assert!(path.requires_directory());
    }

    #[test_case]
    fn embedded_nul_maps_to_einval() {
        assert_eq!(normalize(b"alpha\0beta"), Err(PosixError::InvalidArgument));
    }

    #[test_case]
    fn path_longer_than_limit_maps_to_enametoolong() {
        assert_eq!(
            normalize_path::<4>(b"alpha", PathLimits::new(4, 8, 4)),
            Err(PosixError::NameTooLong)
        );
    }

    #[test_case]
    fn component_longer_than_limit_maps_to_enametoolong() {
        assert_eq!(
            normalize_path::<4>(b"toolong", PathLimits::new(64, 3, 4)),
            Err(PosixError::NameTooLong)
        );
    }

    #[test_case]
    fn component_count_limit_maps_to_enametoolong() {
        assert_eq!(
            normalize_path::<4>(b"a/b/c", PathLimits::new(64, 8, 2)),
            Err(PosixError::NameTooLong)
        );
    }

    #[test_case]
    fn storage_capacity_limit_maps_to_enametoolong() {
        assert_eq!(
            normalize_path::<2>(b"a/b/c", PathLimits::new(64, 8, 4)),
            Err(PosixError::NameTooLong)
        );
    }
}
