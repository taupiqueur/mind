use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct CLI {
  /// Open a specific Mind tree at a given path.
  #[arg(short, long)]
  pub path: Option<PathBuf>,

  /// Use a CWD-tree instead of the global tree.
  #[arg(short, long, default_value_t = false)]
  pub cwd: bool,

  /// Interactive mode.
  ///
  /// When run in interactive mode, base selections can be selected via a fuzzy program.
  #[arg(short, long, default_value_t = false)]
  pub interactive: bool,

  #[command(subcommand)]
  pub cmd: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
  /// Insert a new node.
  ///
  /// This command requires a base selection.
  #[command(alias = "ins")]
  Insert {
    #[arg(default_value_t, short, long, value_enum)]
    mode: InsertMode,

    /// Select a base node to operate on.
    #[arg(short, long)]
    sel: Option<String>,

    /// Name of the node to create.
    #[arg(short, long)]
    name: Option<String>,
  },

  /// Remove a node
  ///
  /// This command requires a base selection.
  #[command(alias = "rm")]
  Remove {
    /// Select a base node to operate on.
    #[arg(short, long)]
    sel: Option<String>,
  },

  /// Rename a node.
  ///
  /// This command requires a base selection.
  Rename {
    /// Select a base node to operate on.
    #[arg(short, long)]
    sel: Option<String>,

    /// New name of the node.
    #[arg(short, long)]
    name: Option<String>,
  },

  /// Change the icon of a node.
  ///
  /// This command requires a base selection
  Icon {
    /// Select a base node to operate on.
    #[arg(short, long)]
    sel: Option<String>,

    /// New icon of the node.
    #[arg(short, long)]
    icon: Option<String>,
  },

  /// Move a node into another one.
  ///
  /// The selected node is the node to move and the path is the destination.
  #[command(alias = "mv")]
  Move {
    #[arg(default_value_t, short, value_enum)]
    mode: InsertMode,

    /// Select a base node to operate on.
    #[arg(short, long)]
    sel: Option<String>,

    /// Destination path
    #[arg(short, long)]
    dest: Option<String>,
  },

  /// Get all paths in a given node.
  Paths {
    /// Select a base node to operate on.
    #[arg(short, long)]
    sel: Option<String>,
  },

  /// Get associated data with a node.
  Data {
    /// Select a base node to operate on.
    #[arg(short, long)]
    sel: Option<String>,

    /// Data type to use for the node.
    #[arg(name = "type", short, long, value_enum)]
    ty: DataType,

    #[command(subcommand)]
    cmd: DataCommand,
  },
}

#[derive(Debug, Subcommand)]
pub enum DataCommand {
  Get,

  Set {
    /// If you ask for a given type of data and the node has no data associated with, the data operation will be
    /// refused. You can use this switch to create the data before operating on it. Depending on the data type, the
    /// content will be interpreted differently.
    #[arg(default_value_t)]
    content: String,
  },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, ValueEnum)]
pub enum InsertMode {
  /// Insert the node inside the selected node, at the top.
  #[value(name = "top")]
  InsideTop,

  /// Insert the node inside the selected node, at the bottom.
  #[default]
  #[value(name = "bottom")]
  InsideBottom,

  /// Insert the node as a sibling, just before the selected node (if the selected has a parent).
  Before,

  /// Insert the node as a sibling, just after the selected node (if the selected has a parent)
  After,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum DataType {
  File,
  Link,
}
