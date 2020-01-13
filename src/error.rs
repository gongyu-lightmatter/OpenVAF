/*
 * ******************************************************************************************
 * Copyright (c) 2019 Pascal Kuthe. This file is part of the VARF project.
 * It is subject to the license terms in the LICENSE file found in the top-level directory
 *  of this distribution and at  https://gitlab.com/jamescoding/VARF/blob/master/LICENSE.
 *  No part of VARF, including this file, may be copied, modified, propagated, or
 *  distributed except according to the terms contained in the LICENSE file.
 * *****************************************************************************************
 */

use crate::{parser, SourceMap, Span};

#[derive(Debug, Clone, Copy)]
pub struct Error<ErrorType> {
    pub error_type: ErrorType,
    pub source: Span,
}
pub enum ErrorType {
    Parser(parser::Error),
}
pub trait PrettyPrintError: Sized {
    fn print<'source>(
        error: &'source Error<Self>,
        source_map: &'source SourceMap,
    ) -> std::fmt::Arguments<'source>;
}
