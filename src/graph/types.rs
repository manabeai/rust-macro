use std::fmt::Debug;

pub trait GraphType {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Undirected {}
impl GraphType for Undirected {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Directed {}
impl GraphType for Directed {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tree {}
impl GraphType for Tree {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dag {}
impl GraphType for Dag {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node<NW> {
    pub weight: Option<NW>,
}