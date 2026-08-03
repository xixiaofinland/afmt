use crate::config::FileSelectionConfig;
use globset::{Glob, GlobSet, GlobSetBuilder};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscoveryError {
    pub path: PathBuf,
    pub message: String,
}

impl Display for DiscoveryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.path.display(), self.message)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiscoveryConfigError {
    InvalidPattern {
        kind: &'static str,
        pattern: String,
        message: String,
    },
    InputMissing(PathBuf),
    InputError {
        path: PathBuf,
        message: String,
    },
    InputUnsupported(PathBuf),
    WorkingDirectoryUnavailable(String),
    NoMatches,
}

impl Display for DiscoveryConfigError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPattern {
                kind,
                pattern,
                message,
            } => {
                write!(
                    formatter,
                    "Invalid {} glob {:?}: {}",
                    kind, pattern, message
                )
            }
            Self::InputMissing(path) => {
                write!(formatter, "Input path does not exist: {}", path.display())
            }
            Self::InputError { path, message } => write!(
                formatter,
                "Failed to inspect input {}: {}",
                path.display(),
                message
            ),
            Self::InputUnsupported(path) => write!(
                formatter,
                "Input path is not a file or directory: {}",
                path.display()
            ),
            Self::WorkingDirectoryUnavailable(message) => write!(
                formatter,
                "Failed to resolve the current working directory: {}",
                message
            ),
            Self::NoMatches => write!(formatter, "No eligible Apex files were found"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscoveryReport {
    pub files: Vec<PathBuf>,
    pub errors: Vec<DiscoveryError>,
}

pub fn discover_targets(
    targets: &[PathBuf],
    selection: &FileSelectionConfig,
    base: &Path,
) -> Result<DiscoveryReport, DiscoveryConfigError> {
    let includes = compile_globs("include", &selection.include)?;
    let excludes = compile_globs("exclude", &selection.exclude)?;
    let mut accumulator = DiscoveryAccumulator::new(base, includes, excludes)?;

    for target in targets {
        if accumulator.is_excluded(target, false) {
            continue;
        }

        let metadata = fs::symlink_metadata(target).map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                DiscoveryConfigError::InputMissing(target.clone())
            } else {
                DiscoveryConfigError::InputError {
                    path: target.clone(),
                    message: error.to_string(),
                }
            }
        })?;

        if metadata.is_file() {
            accumulator.consider_file(target, false);
        } else if metadata.is_dir() {
            if accumulator.is_excluded(target, true) {
                continue;
            }

            let excludes = accumulator.excludes.clone();
            let base = accumulator.base;
            let current_dir = accumulator.current_dir.clone();
            for entry in WalkDir::new(target)
                .follow_links(false)
                .into_iter()
                .filter_entry(|entry| {
                    !is_excluded_path(
                        base,
                        &current_dir,
                        &excludes,
                        entry.path(),
                        entry.file_type().is_dir(),
                    )
                })
            {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(error) => {
                        record_traversal_error(&mut accumulator.errors, target, &error);
                        continue;
                    }
                };

                if entry.file_type().is_file() {
                    accumulator.consider_file(entry.path(), true);
                }
            }
        } else {
            return Err(DiscoveryConfigError::InputUnsupported(target.clone()));
        }
    }

    accumulator.files.sort_by_key(|path| display_path(path));
    if accumulator.files.is_empty() {
        return Err(DiscoveryConfigError::NoMatches);
    }

    Ok(DiscoveryReport {
        files: accumulator.files,
        errors: accumulator.errors,
    })
}

fn compile_globs(kind: &'static str, patterns: &[String]) -> Result<GlobSet, DiscoveryConfigError> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        let normalized = pattern.replace('\\', "/");
        let glob =
            Glob::new(&normalized).map_err(|error| DiscoveryConfigError::InvalidPattern {
                kind,
                pattern: pattern.clone(),
                message: error.to_string(),
            })?;
        builder.add(glob);

        // Globset's recursive prefix is intentionally supplemented so a pattern
        // such as **/*.cls also matches a file directly at the matching root.
        if let Some(root_pattern) = normalized.strip_prefix("**/") {
            builder.add(Glob::new(root_pattern).map_err(|error| {
                DiscoveryConfigError::InvalidPattern {
                    kind,
                    pattern: pattern.clone(),
                    message: error.to_string(),
                }
            })?);
        }
    }
    builder
        .build()
        .map_err(|error| DiscoveryConfigError::InvalidPattern {
            kind,
            pattern: "<combined patterns>".to_string(),
            message: error.to_string(),
        })
}

struct DiscoveryAccumulator<'a> {
    base: &'a Path,
    /// Resolved once up front so relative paths can be made absolute without
    /// re-querying the process working directory for every walked entry.
    current_dir: PathBuf,
    includes: GlobSet,
    excludes: GlobSet,
    identities: HashSet<PathBuf>,
    files: Vec<PathBuf>,
    errors: Vec<DiscoveryError>,
}

impl<'a> DiscoveryAccumulator<'a> {
    fn new(
        base: &'a Path,
        includes: GlobSet,
        excludes: GlobSet,
    ) -> Result<Self, DiscoveryConfigError> {
        let current_dir = std::env::current_dir().map_err(|error| {
            DiscoveryConfigError::WorkingDirectoryUnavailable(error.to_string())
        })?;

        Ok(Self {
            base,
            current_dir,
            includes,
            excludes,
            identities: HashSet::new(),
            files: Vec::new(),
            errors: Vec::new(),
        })
    }

    fn consider_file(&mut self, path: &Path, discovered: bool) {
        if self.is_excluded(path, false) {
            return;
        }

        let candidate = normalized_match_path(path, self.base, &self.current_dir);
        if discovered && !self.includes.is_match(&candidate) {
            return;
        }

        let identity = match fs::canonicalize(path) {
            Ok(path) => path,
            Err(error) => {
                self.errors.push(DiscoveryError {
                    path: path.to_path_buf(),
                    message: format!("Failed to resolve file identity: {}", error),
                });
                return;
            }
        };
        if self.identities.insert(identity) {
            self.files.push(path.to_path_buf());
        }
    }

    fn is_excluded(&self, path: &Path, directory: bool) -> bool {
        is_excluded_path(
            self.base,
            &self.current_dir,
            &self.excludes,
            path,
            directory,
        )
    }
}

fn is_excluded_path(
    base: &Path,
    current_dir: &Path,
    excludes: &GlobSet,
    path: &Path,
    directory: bool,
) -> bool {
    let candidate = normalized_match_path(path, base, current_dir);
    excludes.is_match(&candidate) || (directory && excludes.is_match(format!("{candidate}/")))
}

fn normalized_match_path(path: &Path, base: &Path, current_dir: &Path) -> String {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        current_dir.join(path)
    };
    let path = absolute.strip_prefix(base).unwrap_or(&absolute);
    path.to_string_lossy().replace('\\', "/")
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn record_traversal_error(
    errors: &mut Vec<DiscoveryError>,
    fallback: &Path,
    error: &walkdir::Error,
) {
    errors.push(DiscoveryError {
        path: error.path().unwrap_or(fallback).to_path_buf(),
        message: error.to_string(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir_all, write};
    #[cfg(unix)]
    use std::os::unix::fs::symlink;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn cwd() -> PathBuf {
        std::env::current_dir().unwrap()
    }

    struct TempProject(PathBuf);

    impl TempProject {
        fn new() -> Self {
            let suffix = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = std::env::temp_dir().join(format!("afmt-discovery-{suffix}"));
            create_dir_all(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for TempProject {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn project_files() -> (TempProject, FileSelectionConfig) {
        let project = TempProject::new();
        create_dir_all(project.0.join("nested")).unwrap();
        create_dir_all(project.0.join(".git")).unwrap();
        create_dir_all(project.0.join(".sfdx")).unwrap();
        create_dir_all(project.0.join("node_modules")).unwrap();
        write(project.0.join("root.cls"), "class Root {}\n").unwrap();
        write(
            project.0.join("nested/trigger.trigger"),
            "trigger T on A (before insert) {}\n",
        )
        .unwrap();
        write(project.0.join("nested/query.apex"), "System.debug('x');\n").unwrap();
        write(project.0.join("nested/component.apexc"), "<apex:page/>\n").unwrap();
        write(project.0.join("nested/ignored.txt"), "ignored\n").unwrap();
        write(project.0.join(".git/ignored.cls"), "class Ignored {}\n").unwrap();
        write(project.0.join(".sfdx/ignored.cls"), "class Ignored {}\n").unwrap();
        write(
            project.0.join("node_modules/ignored.cls"),
            "class Ignored {}\n",
        )
        .unwrap();
        (project, FileSelectionConfig::default())
    }

    #[test]
    fn discovers_supported_root_and_nested_files_in_sorted_order() {
        let (project, selection) = project_files();
        let report =
            discover_targets(std::slice::from_ref(&project.0), &selection, &project.0).unwrap();
        let names = report
            .files
            .iter()
            .map(|path| path.to_string_lossy().replace('\\', "/"))
            .collect::<Vec<_>>();

        assert_eq!(
            names,
            vec![
                project
                    .0
                    .join("nested/component.apexc")
                    .to_string_lossy()
                    .replace('\\', "/"),
                project
                    .0
                    .join("nested/query.apex")
                    .to_string_lossy()
                    .replace('\\', "/"),
                project
                    .0
                    .join("nested/trigger.trigger")
                    .to_string_lossy()
                    .replace('\\', "/"),
                project
                    .0
                    .join("root.cls")
                    .to_string_lossy()
                    .replace('\\', "/"),
            ]
        );
    }

    #[test]
    fn exclusions_win_and_explicit_files_bypass_includes() {
        let (project, _) = project_files();
        let selection = FileSelectionConfig {
            include: vec!["**/*.custom".to_string()],
            exclude: vec!["**/nested/**".to_string()],
        };
        write(project.0.join("explicit.custom"), "class Explicit {}\n").unwrap();
        write(
            project.0.join("nested/excluded.custom"),
            "class Excluded {}\n",
        )
        .unwrap();

        let report = discover_targets(
            &[
                project.0.join("explicit.custom"),
                project.0.join("nested"),
                project.0.join("nested/excluded.custom"),
            ],
            &selection,
            &project.0,
        )
        .unwrap();
        assert_eq!(report.files, vec![project.0.join("explicit.custom")]);
    }

    #[test]
    fn built_in_exclusion_predicate_matches_root_and_nested_directories() {
        let (project, selection) = project_files();
        let nested = project.0.join("nested");
        for name in [".git", ".sfdx", "node_modules"] {
            create_dir_all(nested.join(name)).unwrap();
        }
        let accumulator = DiscoveryAccumulator::new(
            &project.0,
            compile_globs("include", &selection.include).unwrap(),
            compile_globs("exclude", &selection.exclude).unwrap(),
        )
        .unwrap();

        for path in [
            project.0.join(".git"),
            project.0.join(".sfdx"),
            project.0.join("node_modules"),
            nested.join(".git"),
            nested.join(".sfdx"),
            nested.join("node_modules"),
        ] {
            assert!(accumulator.is_excluded(&path, true), "{}", path.display());
        }
    }

    #[test]
    fn custom_excluded_directories_are_pruned_before_descent() {
        let (project, _) = project_files();
        let vendor = project.0.join("nested/vendor/deep");
        create_dir_all(&vendor).unwrap();
        let sentinel = vendor.join("sentinel.cls");
        write(&sentinel, "class Sentinel {}\n").unwrap();
        let selection = FileSelectionConfig {
            include: FileSelectionConfig::default().include,
            exclude: vec!["**/vendor/**".to_string()],
        };
        let accumulator = DiscoveryAccumulator::new(
            &project.0,
            compile_globs("include", &selection.include).unwrap(),
            compile_globs("exclude", &selection.exclude).unwrap(),
        )
        .unwrap();
        let vendor_match =
            normalized_match_path(&project.0.join("nested/vendor"), &project.0, &cwd());

        assert!(!accumulator.excludes.is_match(&vendor_match));
        assert!(accumulator.excludes.is_match(format!("{vendor_match}/")));
        assert!(accumulator.is_excluded(&project.0.join("nested/vendor"), true));

        let report =
            discover_targets(std::slice::from_ref(&project.0), &selection, &project.0).unwrap();

        assert!(!report.files.contains(&sentinel));
        assert!(report.files.contains(&project.0.join("root.cls")));
        assert!(report.errors.is_empty());
    }

    #[test]
    fn explicitly_excluded_directories_are_skipped() {
        let (project, selection) = project_files();
        let excluded = project.0.join("node_modules");

        assert_eq!(
            discover_targets(std::slice::from_ref(&excluded), &selection, &project.0),
            Err(DiscoveryConfigError::NoMatches)
        );

        let report = discover_targets(
            &[excluded, project.0.join("root.cls")],
            &selection,
            &project.0,
        )
        .unwrap();
        assert_eq!(report.files, vec![project.0.join("root.cls")]);
        assert!(report.errors.is_empty());
    }

    #[test]
    fn missing_descendants_of_excluded_directories_are_skipped() {
        let (project, selection) = project_files();
        let missing = project.0.join("node_modules/blocked/deep.cls");

        assert_eq!(
            discover_targets(std::slice::from_ref(&missing), &selection, &project.0),
            Err(DiscoveryConfigError::NoMatches)
        );
    }

    #[test]
    fn configured_include_can_select_custom_directory_extensions() {
        let (project, _) = project_files();
        let custom = project.0.join("nested/custom.custom");
        write(&custom, "class Custom {}\n").unwrap();
        let selection = FileSelectionConfig {
            include: vec!["**/*.custom".to_string()],
            exclude: Vec::new(),
        };

        let report =
            discover_targets(std::slice::from_ref(&project.0), &selection, &project.0).unwrap();

        assert_eq!(report.files, vec![custom]);
    }

    #[test]
    fn discovery_preserves_operational_paths_while_matching_portably() {
        let (project, selection) = project_files();
        let report =
            discover_targets(std::slice::from_ref(&project.0), &selection, &project.0).unwrap();

        assert!(report.files.contains(&project.0.join("root.cls")));
        assert_eq!(
            normalized_match_path(&project.0.join("root.cls"), &project.0, &cwd()),
            "root.cls"
        );
    }

    #[test]
    fn overlapping_inputs_are_deduplicated() {
        let (project, selection) = project_files();
        let file = project.0.join("root.cls");
        let report = discover_targets(&[project.0.clone(), file], &selection, &project.0).unwrap();

        assert_eq!(
            report
                .files
                .iter()
                .filter(|path| path.ends_with("root.cls"))
                .count(),
            1
        );
    }

    #[test]
    fn overlapping_directory_inputs_are_deduplicated() {
        let (project, selection) = project_files();
        let report = discover_targets(
            &[project.0.clone(), project.0.join("nested")],
            &selection,
            &project.0,
        )
        .unwrap();

        assert_eq!(report.files.len(), 4);
    }

    #[test]
    fn invalid_globs_and_no_matches_fail_before_formatting() {
        let project = TempProject::new();
        let invalid = FileSelectionConfig {
            include: vec!["[".to_string()],
            exclude: Vec::new(),
        };
        assert!(matches!(
            discover_targets(std::slice::from_ref(&project.0), &invalid, &project.0),
            Err(DiscoveryConfigError::InvalidPattern {
                kind: "include",
                ..
            })
        ));

        let invalid = FileSelectionConfig {
            include: Vec::new(),
            exclude: vec!["[".to_string()],
        };
        assert!(matches!(
            discover_targets(std::slice::from_ref(&project.0), &invalid, &project.0),
            Err(DiscoveryConfigError::InvalidPattern {
                kind: "exclude",
                ..
            })
        ));

        let empty = FileSelectionConfig {
            include: vec!["**/*.cls".to_string()],
            exclude: Vec::new(),
        };
        assert_eq!(
            discover_targets(std::slice::from_ref(&project.0), &empty, &project.0),
            Err(DiscoveryConfigError::NoMatches)
        );
    }

    #[test]
    fn windows_separators_match_portable_patterns() {
        let (project, _) = project_files();
        let selection = FileSelectionConfig {
            include: vec!["nested\\*.apex".to_string()],
            exclude: Vec::new(),
        };
        let report =
            discover_targets(std::slice::from_ref(&project.0), &selection, &project.0).unwrap();
        assert_eq!(report.files.len(), 1);
        assert!(report.files[0].ends_with("query.apex"));
    }

    #[test]
    fn matching_uses_config_base_and_absolute_paths_outside_it() {
        let (project, _) = project_files();
        let base = project.0.join("nested");

        assert_eq!(
            normalized_match_path(&base.join("query.apex"), &base, &cwd()),
            "query.apex"
        );
        let outside = normalized_match_path(&project.0.join("root.cls"), &base, &cwd());
        assert!(outside.ends_with("/root.cls") || outside.ends_with("\\root.cls"));
    }

    #[test]
    fn nonexistent_inputs_are_distinguished_from_empty_matches() {
        let project = TempProject::new();
        let missing = project.0.join("missing.cls");

        assert_eq!(
            discover_targets(
                std::slice::from_ref(&missing),
                &FileSelectionConfig::default(),
                &project.0
            ),
            Err(DiscoveryConfigError::InputMissing(missing))
        );

        let invalid = PathBuf::from("invalid\0path");
        assert!(matches!(
            discover_targets(
                std::slice::from_ref(&invalid),
                &FileSelectionConfig::default(),
                &project.0
            ),
            Err(DiscoveryConfigError::InputError { path, .. }) if path == invalid
        ));
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_directories_are_not_traversed_and_unsupported_inputs_are_rejected() {
        let (project, selection) = project_files();
        let linked = project.0.join("linked");
        symlink(project.0.join("nested"), &linked).unwrap();
        let report =
            discover_targets(std::slice::from_ref(&project.0), &selection, &project.0).unwrap();
        assert!(!report.files.iter().any(|path| path.starts_with(&linked)));

        let link_file = project.0.join("link-file.cls");
        symlink(project.0.join("root.cls"), &link_file).unwrap();
        assert_eq!(
            discover_targets(std::slice::from_ref(&link_file), &selection, &project.0),
            Err(DiscoveryConfigError::InputUnsupported(link_file))
        );
    }

    #[test]
    fn traversal_errors_retain_their_path_and_message() {
        let project = TempProject::new();
        let missing = project.0.join("walk-missing");
        let error = WalkDir::new(&missing)
            .follow_links(false)
            .into_iter()
            .next()
            .expect("walk should yield an error")
            .expect_err("missing walk root should fail");
        let mut errors = Vec::new();

        record_traversal_error(&mut errors, &project.0, &error);

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].path, missing);
        assert!(!errors[0].message.is_empty());
    }
}
