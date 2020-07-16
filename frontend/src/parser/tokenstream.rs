/*
 * ******************************************************************************************
 * Copyright (c) 2020 Pascal Kuthe. This file is part of the frontend project.
 * It is subject to the license terms in the LICENSE file found in the top-level directory
 *  of this distribution and at  https://gitlab.com/DSPOM/OpenVAF/blob/master/LICENSE.
 *  No part of frontend, including this file, may be copied, modified, propagated, or
 *  distributed except according to the terms contained in the LICENSE file.
 * *****************************************************************************************
 */

use crate::sourcemap::Span;
use crate::symbol::Symbol;
use crate::StringLiteral;
use core::fmt::Formatter;
use std::fmt::Display;

pub type SpannedToken = (Token, Span);

pub type TokenStream = Vec<SpannedToken>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Ident(Symbol),
    LiteralString(StringLiteral),
    RealLiteral(f64),
    IntLiteral(u32),

    Temperature,
    Vt,
    SimParam,
    SimParamStr,
    PortConnected,
    ParamGiven,
    Display,
    Strobe,
    Write,
    Debug,
    Finish,
    Stop,
    Info,
    Warn,
    Error,
    Fatal,

    Accessor,
    Semicolon,
    Colon,
    Comma,
    ParenOpen,
    ParenClose,
    AttributeStart,
    AttributeEnd,
    SquareBracketOpen,
    SquareBracketClose,
    Contribute,
    Assign,
    Hash,

    OpMul,
    OpDiv,
    OpModulus,
    Plus,
    Minus,
    OpExp,
    OpLogicNot,
    OpBitNot,

    OpArithmeticShiftLeft,
    OpArithmeticShiftRight,

    OpLess,
    OpLessEqual,
    OpGreater,
    OpGreaterEqual,
    OpEqual,
    OpNotEqual,
    OpLogicAnd,
    OpLogicalOr,

    OpBitAnd,
    OpBitXor,
    OpBitNXor,
    OpBitOr,

    OpCondition,

    //Keywords
    If,
    Else,
    Case,
    EndCase,
    Default,

    While,
    For,

    Begin,
    End,

    Module,
    EndModule,
    Discipline,
    EndDiscipline,

    Nature,
    EndNature,

    Branch,
    Parameter,
    DefineParameter,
    LocalParameter,

    Analog,
    Function,
    EndFunction,
    AnalogInitial,

    Input,
    Inout,
    Output,

    Signed,
    Vectored,
    Scalared,

    //Types
    String,
    Time,
    Realtime,
    Integer,
    Real,
    Reg,
    Wreal,
    Supply0,
    Supply1,
    Tri,
    TriAnd,
    TriOr,
    Tri0,
    Tri1,
    Wire,
    Uwire,
    Wand,
    Wor,
    Ground,

    Potential,
    Flow,
    Domain,
    Discrete,
    Continuous,

    TemperatureDerivative,
    TimeDerivative,
    PartialDerivative,
    TimeIntegral,
    TimeIntegralMod,
    LimExp,
    WhiteNoise,
    FlickerNoise,

    Pow,
    Sqrt,

    Hypot,
    Exp,
    Ln,
    Log,
    Min,
    Max,
    Abs,
    Floor,
    Ceil,

    Sin,
    Cos,
    Tan,

    ArcSin,
    ArcCos,
    ArcTan,
    ArcTan2,

    SinH,
    CosH,
    TanH,

    ArcSinH,
    ArcCosH,
    ArcTanH,

    From,
    Exclude,
    Infinity,
    MinusInfinity,

    Abstol,
    Access,
    TimeDerivativeNature,
    TimeIntegralNature,
    Units,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Ident(_) => f.write_str("Identifier"),
            Self::LiteralString(_) => f.write_str("string literal"),
            Self::RealLiteral(_) => f.write_str("real number"),
            Self::IntLiteral(_) => f.write_str("integer number"),
            Self::Accessor => f.write_str("."),
            Self::Semicolon => f.write_str(";"),
            Self::Colon => f.write_str(":"),
            Self::Comma => f.write_str(","),
            Self::ParenOpen => f.write_str("("),
            Self::ParenClose => f.write_str(")"),
            Self::AttributeStart => f.write_str("(*"),
            Self::AttributeEnd => f.write_str("*)"),
            Self::SquareBracketOpen => f.write_str("["),
            Self::SquareBracketClose => f.write_str("]"),
            Self::Contribute => f.write_str("<+"),
            Self::Assign => f.write_str("="),
            Self::Hash => f.write_str("#"),
            Self::OpMul => f.write_str("*"),
            Self::OpDiv => f.write_str("/"),
            Self::OpModulus => f.write_str("%"),
            Self::Plus => f.write_str("+"),
            Self::Minus => f.write_str("-"),
            Self::OpExp => f.write_str("**"),
            Self::OpLogicNot => f.write_str("!"),
            Self::OpBitNot => f.write_str("~"),
            Self::OpArithmeticShiftLeft => f.write_str("<<"),
            Self::OpArithmeticShiftRight => f.write_str(">>"),
            Self::OpLess => f.write_str("<"),
            Self::OpLessEqual => f.write_str("<="),
            Self::OpGreater => f.write_str(">"),
            Self::OpGreaterEqual => f.write_str(">="),
            Self::OpEqual => f.write_str("=="),
            Self::OpNotEqual => f.write_str("!="),
            Self::OpLogicAnd => f.write_str("&&"),
            Self::OpLogicalOr => f.write_str("||"),
            Self::OpBitAnd => f.write_str("&"),
            Self::OpBitXor => f.write_str("^"),
            Self::OpBitNXor => f.write_str("^~/~^"),
            Self::OpBitOr => f.write_str("|"),
            Self::OpCondition => f.write_str("?"),
            Self::If => f.write_str("if"),
            Self::Else => f.write_str("else"),
            Self::Case => f.write_str("case"),
            Self::EndCase => f.write_str("endcase"),
            Self::While => f.write_str("while"),
            Self::For => f.write_str("for"),
            Self::Begin => f.write_str("begin"),
            Self::End => f.write_str("end"),
            Self::Module => f.write_str("module"),
            Self::EndModule => f.write_str("endmodule"),
            Self::Discipline => f.write_str("discipline"),
            Self::EndDiscipline => f.write_str("enddiscipline"),
            Self::Nature => f.write_str("nature"),
            Self::EndNature => f.write_str("endnature"),
            Self::Branch => f.write_str("branch"),
            Self::Parameter => f.write_str("parameter"),
            Self::DefineParameter => f.write_str("defparam"),
            Self::LocalParameter => f.write_str("localparam"),
            Self::Analog => f.write_str("analog"),
            Self::AnalogInitial => f.write_str("inital"),
            Self::Input => f.write_str("input"),
            Self::Inout => f.write_str("inout"),
            Self::Output => f.write_str("output"),
            Self::Signed => f.write_str("signed"),
            Self::Vectored => f.write_str("vectored"),
            Self::Scalared => f.write_str("scalared"),
            Self::String => f.write_str("string"),
            Self::Time => f.write_str("time"),
            Self::Realtime => f.write_str("realtime"),
            Self::Integer => f.write_str("integer"),
            Self::Real => f.write_str("real"),
            Self::Reg => f.write_str("reg"),
            Self::Wreal => f.write_str("wreal"),
            Self::Supply0 => f.write_str("supply0"),
            Self::Supply1 => f.write_str("supply1"),
            Self::Tri => f.write_str("tri"),
            Self::TriAnd => f.write_str("triand"),
            Self::TriOr => f.write_str("trior"),
            Self::Tri0 => f.write_str("tri0"),
            Self::Tri1 => f.write_str("tri1"),
            Self::Wire => f.write_str("wire"),
            Self::Uwire => f.write_str("uwire"),
            Self::Wand => f.write_str("wand"),
            Self::Wor => f.write_str("wor"),
            Self::Ground => f.write_str("ground"),
            Self::Potential => f.write_str("potential"),
            Self::Flow => f.write_str("flow"),
            Self::Domain => f.write_str("domain"),
            Self::Discrete => f.write_str("discrete"),
            Self::Continuous => f.write_str("continuous"),
            Self::TimeDerivative => f.write_str("ddt"),
            Self::TemperatureDerivative => f.write_str("ddT"),
            Self::PartialDerivative => f.write_str("ddx"),
            Self::TimeIntegral => f.write_str("idt"),
            Self::TimeIntegralMod => f.write_str("idtmod"),
            Self::LimExp => f.write_str("limexp"),
            Self::WhiteNoise => f.write_str("whitenoise"),
            Self::FlickerNoise => f.write_str("flickrnoise"),
            Self::Pow => f.write_str("pow"),
            Self::Sqrt => f.write_str("sqrt"),
            Self::Hypot => f.write_str("hypot"),
            Self::Exp => f.write_str("exp"),
            Self::Ln => f.write_str("ln"),
            Self::Log => f.write_str("log"),
            Self::Min => f.write_str("min"),
            Self::Max => f.write_str("max"),
            Self::Abs => f.write_str("abs"),
            Self::Floor => f.write_str("floor"),
            Self::Ceil => f.write_str("ceil"),
            Self::Sin => f.write_str("sin"),
            Self::Cos => f.write_str("cos"),
            Self::Tan => f.write_str("tan"),
            Self::ArcSin => f.write_str("asin"),
            Self::ArcCos => f.write_str("acos"),
            Self::ArcTan => f.write_str("atan"),
            Self::ArcTan2 => f.write_str("atan2"),
            Self::SinH => f.write_str("sinh"),
            Self::CosH => f.write_str("cosh"),
            Self::TanH => f.write_str("tanh"),
            Self::ArcSinH => f.write_str("asinh"),
            Self::ArcCosH => f.write_str("acosh"),
            Self::ArcTanH => f.write_str("atanh"),
            Self::From => f.write_str("from"),
            Self::Exclude => f.write_str("exclude"),
            Self::Infinity => f.write_str("inf"),
            Self::MinusInfinity => f.write_str("-inf"),
            Self::Abstol => f.write_str("abstol"),
            Self::Access => f.write_str("access"),
            Self::TimeDerivativeNature => f.write_str("ddt_nature"),
            Self::TimeIntegralNature => f.write_str("idt_nature"),
            Self::Units => f.write_str("units"),
            Self::Temperature => f.write_str("$temperature"),
            Self::Vt => f.write_str("$vt"),
            Self::SimParam => f.write_str("$simparam"),
            Self::SimParamStr => f.write_str("$simparam$str"),
            Self::PortConnected => f.write_str("$port_connected"),
            Self::ParamGiven => f.write_str("$param_given"),
            Self::Display => f.write_str("$display"),
            Self::Strobe => f.write_str("$strobe"),
            Self::Write => f.write_str("$write"),
            Self::Debug => f.write_str("$debug"),
            Self::Function => f.write_str("function"),
            Self::EndFunction => f.write_str("endfunction"),
            Self::Finish => f.write_str("$finish"),
            Self::Stop => f.write_str("$stop"),
            Self::Info => f.write_str("$info"),
            Self::Warn => f.write_str("$warn"),
            Self::Error => f.write_str("$error"),
            Self::Fatal => f.write_str("$fatal"),
            Self::Default => f.write_str("default"),
        }
    }
}
