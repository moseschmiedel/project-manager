use std::{
    fs::DirEntry,
    ops::Deref,
    path::{Path, PathBuf},
    sync::Arc,
};

pub struct Project {
    info: Info,
}

impl Project {
    pub fn new(name: Arc<str>, slug: Arc<str>, path: PathBuf) -> Self {
        Project {
            info: Info { name, slug, path },
        }
    }
}

impl Project {
    fn fmt_path(&self) -> String {
        format!("{}", self.info.path.display());
    }
}

impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Project '{}' ({}): {}",
            self.info.name,
            self.info.slug,
            self.info.path.display()
        )
    }
}

struct Info {
    name: Arc<str>,
    slug: Arc<str>,
    path: PathBuf,
}

impl Info {
    fn parse_dir_path(dir: &Path) -> Info {
        let name = dir.file_name().unwrap().to_str().unwrap();
        Info {
            name: name.into(),
            slug: name.to_lowercase().replace(' ', "-").into(),
            path: dir.to_path_buf(),
        }
    }
}

pub struct RootNamespace {
    info: Info,
    items: Vec<NamespaceItem>,
}

impl RootNamespace {
    pub fn new(name: Arc<str>, slug: Arc<str>, path: PathBuf) -> Self {
        Self {
            info: Info { name, slug, path },
            items: Vec::new(),
        }
    }

    fn with_items(info: Info, items: Vec<NamespaceItem>) -> Self {
        Self { info, items }
    }

    pub fn build_project_slugs(&self) -> Vec<Arc<str>> {
        let mut slugs = Vec::new();
        for item in &self.items {
            match item {
                NamespaceItem::Project(project) => {
                    slugs.push(project.info.slug.clone());
                }
                NamespaceItem::Namespace(namespace) => {
                    slugs.extend(namespace.build_project_slugs("".into()));
                }
            }
        }
        slugs
    }
}

impl std::fmt::Display for RootNamespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "RootNamespace: {}", self.info.path.display())?;
        for item in &self.items {
            match item {
                NamespaceItem::Project(project) => {
                    writeln!(f, "  - {}", project)?;
                }
                NamespaceItem::Namespace(namespace) => {
                    writeln!(f, "  - {}", namespace)?;
                    namespace.display_items(f, 4)?;
                }
            }
        }

        Ok(())
    }
}

pub struct SubNamespace {
    info: Info,
    items: Vec<NamespaceItem>,
}

impl SubNamespace {
    pub fn new(name: Arc<str>, slug: Arc<str>, path: PathBuf) -> Self {
        Self {
            info: Info { name, slug, path },
            items: Vec::new(),
        }
    }

    pub fn build_project_slugs(&self, aggregated_slug: Arc<str>) -> Vec<Arc<str>> {
        let mut slugs = Vec::new();
        let my_slug = if aggregated_slug.is_empty() {
            self.info.slug.clone()
        } else {
            format!("{}.{}", aggregated_slug, self.info.slug).into()
        };
        for item in &self.items {
            match item {
                NamespaceItem::Project(project) => {
                    slugs.push(format!("{}.{}", my_slug, project.info.slug).into());
                }
                NamespaceItem::Namespace(namespace) => {
                    slugs.extend(namespace.build_project_slugs(my_slug.clone()));
                }
            }
        }
        slugs
    }

    fn with_items(info: Info, items: Vec<NamespaceItem>) -> Self {
        Self { info, items }
    }

    fn display_items(&self, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
        let indent_str = " ".repeat(indent);
        for item in &self.items {
            match item {
                NamespaceItem::Project(project) => {
                    writeln!(f, "{}- {}", indent_str, project)?;
                }
                NamespaceItem::Namespace(namespace) => {
                    writeln!(f, "{}- {}", indent_str, namespace)?;
                    namespace.display_items(f, indent + 2)?;
                }
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for SubNamespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Namespace '{}' ({}): {}",
            self.info.name,
            self.info.slug,
            self.info.path.display()
        )
    }
}

enum NamespaceItem {
    Project(Project),
    Namespace(SubNamespace),
}

pub struct Detector {
    root: PathBuf,
    config: DetectorConfig,
}

pub struct DetectorConfig {
    ignore_hidden: bool,
    ignore_patterns: Vec<String>,
}

impl DetectorConfig {
    pub fn ignore_hidden_files(mut self, ignore: bool) -> Self {
        self.ignore_hidden = ignore;
        self
    }

    pub fn ignore_pattern(mut self, pattern: String) -> Self {
        self.ignore_patterns.push(pattern);
        self
    }
}

impl Default for DetectorConfig {
    fn default() -> Self {
        DetectorConfig {
            ignore_hidden: true,
            ignore_patterns: vec![],
        }
    }
}

impl Detector {
    pub fn new(root: PathBuf) -> Self {
        Detector {
            root,
            config: DetectorConfig::default(),
        }
    }

    pub fn with_config(root: PathBuf, config: DetectorConfig) -> Self {
        Detector { root, config }
    }

    pub fn detect(self) -> RootNamespace {
        let mut items = Vec::new();

        self.parse_path(&self.root, &mut items);

        RootNamespace::with_items(
            Info {
                name: "Root".into(),
                slug: "root".into(),
                path: self.root,
            },
            items,
        )
    }

    fn parse_path(&self, path: &Path, items: &mut Vec<NamespaceItem>) {
        for entry in self.list_dir(path) {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                if let Some(project) = self.detect_project(&entry_path) {
                    items.push(NamespaceItem::Project(project));
                } else {
                    let mut sub_items = Vec::new();

                    self.parse_path(&entry_path, &mut sub_items);

                    items.push(NamespaceItem::Namespace(SubNamespace::with_items(
                        Info::parse_dir_path(&entry_path),
                        sub_items,
                    )))
                }
            }
        }
    }

    fn detect_project(&self, path: &Path) -> Option<Project> {
        self.detect_git(path)
            .or(self.detect_zig(path))
            .or(self.detect_cargo(path))
            .or(self.detect_meson(path))
            .or(self.detect_make(path))
            .or(self.detect_cmake(path))
            .or(self.detect_node(path))
            .or(self.detect_nix(path))
            .or(self.detect_go(path))
    }

    fn list_dir(&self, dir: &Path) -> Vec<DirEntry> {
        let mut entries = Vec::new();
        'entry_for: for entry in dir
            .read_dir()
            .expect(format!("Could not read directory: {}", dir.display()).as_str())
        {
            let entry = entry.unwrap();

            let filename = entry.file_name();
            let os_string = filename.to_string_lossy();
            let str = os_string.deref();

            if self.config.ignore_hidden {
                if str.starts_with('.') {
                    continue;
                }
            }

            for pattern in self.config.ignore_patterns.iter() {
                if str.contains(pattern) {
                    continue 'entry_for;
                }
            }

            entries.push(entry);
        }

        entries
    }

    fn detect_filenames(&self, filenames: &[&str], dir: &Path) -> bool {
        for entry in dir
            .read_dir()
            .expect(format!("Could not read directory: {}", dir.display()).as_str())
        {
            let entry = entry.unwrap();
            let filename = entry.file_name();
            let os_string = filename.to_string_lossy();
            let str = os_string.deref();
            if filenames.contains(&str) {
                return true;
            }
        }

        false
    }

    fn detect_git(&self, dir: &Path) -> Option<Project> {
        if !self.detect_filenames(&[".git"], dir) {
            return None;
        }

        Some(Project {
            info: Info::parse_dir_path(dir),
        })
    }
    fn detect_zig(&self, dir: &Path) -> Option<Project> {
        if !self.detect_filenames(&["build.zig"], dir) {
            return None;
        }

        Some(Project {
            info: Info::parse_dir_path(dir),
        })
    }
    fn detect_cargo(&self, dir: &Path) -> Option<Project> {
        if !self.detect_filenames(&["Cargo.toml"], dir) {
            return None;
        }

        Some(Project {
            info: Info::parse_dir_path(dir),
        })
    }
    fn detect_meson(&self, dir: &Path) -> Option<Project> {
        if !self.detect_filenames(&["meson.build"], dir) {
            return None;
        }

        Some(Project {
            info: Info::parse_dir_path(dir),
        })
    }
    fn detect_make(&self, dir: &Path) -> Option<Project> {
        if !self.detect_filenames(&["Makefile", "makefile", "GNUmakefile"], dir) {
            return None;
        }

        Some(Project {
            info: Info::parse_dir_path(dir),
        })
    }
    fn detect_cmake(&self, dir: &Path) -> Option<Project> {
        if !self.detect_filenames(&["CMakeLists.txt"], dir) {
            return None;
        }

        Some(Project {
            info: Info::parse_dir_path(dir),
        })
    }
    fn detect_node(&self, dir: &Path) -> Option<Project> {
        if !self.detect_filenames(&["package.json"], dir) {
            return None;
        }

        Some(Project {
            info: Info::parse_dir_path(dir),
        })
    }
    fn detect_nix(&self, dir: &Path) -> Option<Project> {
        if !self.detect_filenames(&["default.nix", "shell.nix", "flake.nix"], dir) {
            return None;
        }

        Some(Project {
            info: Info::parse_dir_path(dir),
        })
    }
    fn detect_go(&self, dir: &Path) -> Option<Project> {
        if !self.detect_filenames(&["go.mod"], dir) {
            return None;
        }

        Some(Project {
            info: Info::parse_dir_path(dir),
        })
    }
}
