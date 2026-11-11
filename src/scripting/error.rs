use alloc::string::{String, ToString};

#[derive(Debug, Clone, Copy)]
pub enum ScriptErrorKind {
    SyntaxError,
    SemanticError,
    RuntimeError,
    ParseError,
    LoopError,
    ConditionalError,
    VariableError,
    ArgumentError,
    FileError,
    RecursionError,
}

impl ScriptErrorKind {
    pub fn tag(&self) -> &'static str {
        match self {
            ScriptErrorKind::SyntaxError => "SCRIPT:SYNTAX",
            ScriptErrorKind::SemanticError => "SCRIPT:SEMANTIC",
            ScriptErrorKind::RuntimeError => "SCRIPT:RUNTIME",
            ScriptErrorKind::ParseError => "SCRIPT:PARSE",
            ScriptErrorKind::LoopError => "SCRIPT:LOOP",
            ScriptErrorKind::ConditionalError => "SCRIPT:COND",
            ScriptErrorKind::VariableError => "SCRIPT:VAR",
            ScriptErrorKind::ArgumentError => "SCRIPT:ARG",
            ScriptErrorKind::FileError => "SCRIPT:FILE",
            ScriptErrorKind::RecursionError => "SCRIPT:RECURSE",
        }
    }

    pub fn category(&self) -> &'static str {
        match self {
            ScriptErrorKind::SyntaxError => "Syntax Error",
            ScriptErrorKind::SemanticError => "Semantic Error",
            ScriptErrorKind::RuntimeError => "Runtime Error",
            ScriptErrorKind::ParseError => "Parse Error",
            ScriptErrorKind::LoopError => "Loop Error",
            ScriptErrorKind::ConditionalError => "Conditional Error",
            ScriptErrorKind::VariableError => "Variable Error",
            ScriptErrorKind::ArgumentError => "Argument Error",
            ScriptErrorKind::FileError => "File Error",
            ScriptErrorKind::RecursionError => "Recursion Error",
        }
    }
}


#[derive(Debug, Clone)]
pub struct ScriptError {
    pub kind: ScriptErrorKind,
    pub message: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub source_line: Option<String>,
    pub hint: Option<String>,
}

impl ScriptError {
    pub fn new(kind: ScriptErrorKind, message: String, file: &str, line: usize, column: usize) -> Self {
        Self {
            kind,
            message,
            file: file.to_string(),
            line,
            column,
            source_line: None,
            hint: None,
        }
    }

    pub fn with_source_line(mut self, source: String) -> Self {
        self.source_line = Some(source);
        self
    }

    pub fn with_hint(mut self, hint: String) -> Self {
        self.hint = Some(hint);
        self
    }

    pub fn format_detailed(&self) -> String {
        let mut out = String::new();

        out.push_str(&alloc::format!(
            "\n  {}\n  File: {} | Line: {} | Column: {}\n",
            self.kind.category(),
            self.file,
            self.line,
            self.column
        ));

        out.push_str(&alloc::format!(" -> {}", self.message));

        if let Some(ref src) = self.source_line {
            let trimmed = src.trim_end();
            out.push_str(&alloc::format!("\n  | {}", trimmed));

            let mut caret = String::from("  | ");
            let col = if self.column > 0 { self.column - 1 } else { 0 };
            for (i, _ch) in trimmed.chars().enumerate() {
                if i == col {
                    caret.push('^');
                } else if i > 0 && i < col + 3 && i < trimmed.len() {
                    caret.push('~');
                } else if i <= col {
                    caret.push(' ');
                }
            }
            out.push_str(&alloc::format!("\n{}", caret));
        }

        // Hint if available. we dont support any emojis, but we will use it until char is added
        if let Some(ref hint) = self.hint {
            out.push_str(&alloc::format!("\n  💡 Hint: {}", hint));
        }

        out.push('\n');
        out
    }
}

pub type ScriptResult<T> = Result<T, ScriptError>;
