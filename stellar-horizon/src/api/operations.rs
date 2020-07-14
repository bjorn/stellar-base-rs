use crate::api::Page;
use crate::error::Result;
use crate::request::{Order, PageRequest, Request, StreamRequest, UrlPageRequestExt};
use crate::resources;
use url::Url;

#[derive(Debug, Clone)]
pub struct AllOperationsRequest {}

#[derive(Debug, Clone)]
pub struct SingleOperationRequest {}

#[derive(Debug, Clone)]
pub struct OperationsForAccountRequest {}

#[derive(Debug, Clone)]
pub struct OperationsForLedgerRequest {}
