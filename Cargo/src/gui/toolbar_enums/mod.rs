use strum::{IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCountMacro, EnumIter)]
pub enum FilesOptions {
    NewFile,
    OpenFile,
    SaveFile,
}

impl std::fmt::Display for FilesOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::NewFile => "New file",
            Self::OpenFile => "Open file",
            Self::SaveFile => "Save file",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCountMacro, EnumIter)]
pub enum EditOptions {
    Copy,
    Paste,
    Delete,
}

impl std::fmt::Display for EditOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Copy => "Copy",
            Self::Paste => "Paste",
            Self::Delete => "Delete",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumCountMacro, EnumIter)]
pub enum ViewOptions {
    NotKnownYet
}

impl std::fmt::Display for ViewOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::NotKnownYet => "NotKnownYet",
        })
    }
}