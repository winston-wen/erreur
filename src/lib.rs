/// in lib.rs or main.rs,
/// use erreur::*;
use std::{borrow::Cow, fmt, result::Result as StdResult};

#[must_use]
pub struct Erreur {
    name: Cow<'static, str>,
    file: Cow<'static, str>,
    line: u32,
    column: u32,
    context: Option<Cow<'static, str>>,
    inner: Option<Box<dyn std::error::Error + Send + Sync>>,
}

/// Convenience alias: `StdResult<T, Box<Erreur>>`.
pub type Resultat<T> = StdResult<T, Box<Erreur>>;

impl Erreur {
    #[must_use]
    pub fn new() -> Box<Self> {
        Box::new(Erreur {
            name: Cow::Borrowed("UnknownException"),
            file: Cow::Borrowed(""),
            line: 0,
            column: 0,
            context: None,
            inner: None,
        })
    }

    #[inline]
    pub fn set_file(&mut self, file: impl Into<Cow<'static, str>>) -> &mut Self {
        self.file = file.into();
        self
    }

    #[inline]
    pub fn set_name(&mut self, name: impl Into<Cow<'static, str>>) -> &mut Self {
        self.name = name.into();
        self
    }

    #[inline]
    pub fn set_line(&mut self, line: u32) -> &mut Self {
        self.line = line;
        self
    }

    #[inline]
    pub fn set_column(&mut self, column: u32) -> &mut Self {
        self.column = column;
        self
    }

    #[inline]
    pub fn set_context(&mut self, ctx: impl Into<Cow<'static, str>>) -> &mut Self {
        self.context = Some(ctx.into());
        self
    }

    #[inline]
    pub fn set_caused_by(
        &mut self,
        err: impl std::error::Error + Send + Sync + 'static,
    ) -> &mut Self {
        self.inner = Some(Box::new(err));
        self
    }

    #[inline]
    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[inline]
    #[must_use]
    pub fn get_context(&self) -> Option<&str> {
        self.context.as_deref()
    }
}

impl fmt::Display for Erreur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut msg: String = format!("Exception \"{}\" occurs at \"{}", self.name, self.file);
        if self.line > 0 {
            msg += &format!(":{}", &self.line);
        }
        if self.column > 0 {
            msg += &format!(":{}", &self.column);
        }
        msg += "\"";
        if let Some(ctx) = &self.context {
            let ctx = ctx.trim();
            if !ctx.is_empty() {
                msg += &format!("\nContext: {}", ctx);
            }
        }
        if let Some(inner) = &self.inner {
            msg += &format!("\nCaused by:\n{}", inner);
        }
        msg += "\n";
        write!(f, "{}", msg)
    }
}

impl fmt::Debug for Erreur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.to_string())
    }
}

impl std::error::Error for Erreur {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.as_deref().map(|e| e as &(dyn std::error::Error + 'static))
    }
}

pub trait Catch<T, E> {
    /// Wrap an error with a custom name and context.
    /// Source file, line and column are captured automatically via `#[track_caller]`.
    fn catch(self, name: impl AsRef<str>, ctx: impl AsRef<str>) -> Resultat<T>;

    /// Lazy variant of `catch`. Preserves the default exception name.
    fn catch_(self) -> Resultat<T>;
}

impl<T, E> Catch<T, E> for StdResult<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    #[track_caller]
    fn catch(self, name: impl AsRef<str>, ctx: impl AsRef<str>) -> Resultat<T> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let mut ex = Erreur::new();
                let loc = std::panic::Location::caller();
                ex.set_name(name.as_ref().to_string())
                    .set_file(loc.file())
                    .set_line(loc.line())
                    .set_column(loc.column())
                    .set_context(ctx.as_ref().to_string())
                    .set_caused_by(e);
                Err(ex)
            }
        }
    }

    #[track_caller]
    fn catch_(self) -> Resultat<T> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let mut ex = Erreur::new();
                let loc = std::panic::Location::caller();
                ex.set_file(loc.file())
                    .set_line(loc.line())
                    .set_column(loc.column())
                    .set_caused_by(e);
                Err(ex)
            }
        }
    }
}

#[macro_export]
macro_rules! throw {
    ($name:expr, $ctx:expr) => {{
        let mut ex = $crate::Erreur::new();
        let loc = std::panic::Location::caller();
        ex.set_name($name)
            .set_file(loc.file())
            .set_line(loc.line())
            .set_column(loc.column())
            .set_context($ctx);
        return Err(ex);
    }};
    () => {{
        $crate::throw!("UnknownException", "");
    }};
}

#[macro_export]
macro_rules! assert_throw {
    ($cond:expr, $name:expr, $ctx:expr) => {
        if !($cond) {
            let mut ex = $crate::Erreur::new();
            let loc = std::panic::Location::caller();
            let ctx = format!("Condition: {}\nExplanation: {}", stringify!($cond), $ctx);
            ex.set_name($name)
                .set_file(loc.file())
                .set_line(loc.line())
                .set_column(loc.column())
                .set_context(ctx);
            return Err(ex);
        }
    };
    ($cond:expr, $ctx:expr) => {
        if !($cond) {
            let mut ex = $crate::Erreur::new();
            let loc = std::panic::Location::caller();
            let ctx = format!("Condition: {}\nExplanation: {}", stringify!($cond), $ctx);
            ex.set_name("AssertionFailedException")
                .set_file(loc.file())
                .set_line(loc.line())
                .set_column(loc.column())
                .set_context(ctx);
            return Err(ex);
        }
    };
    ($cond:expr) => {
        if !($cond) {
            let mut ex = $crate::Erreur::new();
            let loc = std::panic::Location::caller();
            let ctx = format!("Condition: {}", stringify!($cond));
            ex.set_name("AssertionFailedException")
                .set_file(loc.file())
                .set_line(loc.line())
                .set_column(loc.column())
                .set_context(ctx);
            return Err(ex);
        }
    };
}

pub trait IfNone<T> {
    fn ifnone(self, name: impl AsRef<str>, ctx: impl AsRef<str>) -> Resultat<T>;
    fn ifnone_(self) -> Resultat<T>;
}

impl<T> IfNone<T> for std::option::Option<T> {
    #[track_caller]
    fn ifnone(self, name: impl AsRef<str>, ctx: impl AsRef<str>) -> Resultat<T> {
        match self {
            Some(t) => Ok(t),
            None => {
                let name = name.as_ref();
                let actual_name = if name.is_empty() {
                    "ObjectIsNone"
                } else {
                    name
                };
                let mut ex = Erreur::new();
                let loc = std::panic::Location::caller();
                ex.set_name(actual_name.to_string())
                    .set_file(loc.file())
                    .set_line(loc.line())
                    .set_column(loc.column())
                    .set_context(ctx.as_ref().to_string());
                Err(ex)
            }
        }
    }

    #[track_caller]
    fn ifnone_(self) -> Resultat<T> {
        match self {
            Some(t) => Ok(t),
            None => {
                let mut ex = Erreur::new();
                let loc = std::panic::Location::caller();
                ex.set_file(loc.file())
                    .set_line(loc.line())
                    .set_column(loc.column());
                Err(ex)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::io;

    // ── Erreur construction & getters ──

    #[test]
    fn new_has_default_name() {
        let ex = Erreur::new();
        assert_eq!(ex.get_name(), "UnknownException");
        assert_eq!(ex.get_context(), None);
    }

    #[test]
    fn setters_and_getters() {
        let mut ex = Erreur::new();
        ex.set_name("TestException")
            .set_file("src/main.rs")
            .set_line(42)
            .set_column(5)
            .set_context("something went wrong");
        assert_eq!(ex.get_name(), "TestException");
        assert_eq!(ex.get_context(), Some("something went wrong"));
    }

    #[test]
    fn set_name_accepts_string() {
        let mut ex = Erreur::new();
        ex.set_name(String::from("DynamicName"));
        assert_eq!(ex.get_name(), "DynamicName");
    }

    #[test]
    fn set_context_accepts_string() {
        let mut ex = Erreur::new();
        ex.set_context(format!("value = {}", 42));
        assert_eq!(ex.get_context(), Some("value = 42"));
    }

    // ── Display formatting ──

    #[test]
    fn display_basic() {
        let mut ex = Erreur::new();
        ex.set_name("Boom").set_file("lib.rs").set_line(10).set_column(3);
        let msg = ex.to_string();
        assert!(msg.contains("Boom"));
        assert!(msg.contains("lib.rs:10:3"));
    }

    #[test]
    fn display_omits_zero_line_and_column() {
        let mut ex = Erreur::new();
        ex.set_name("Boom").set_file("lib.rs");
        let msg = ex.to_string();
        assert!(msg.contains("\"lib.rs\""));
        assert!(!msg.contains(":0"));
    }

    #[test]
    fn display_includes_context() {
        let mut ex = Erreur::new();
        ex.set_name("Err").set_file("a.rs").set_line(1).set_context("details here");
        let msg = ex.to_string();
        assert!(msg.contains("Context: details here"));
    }

    #[test]
    fn display_skips_empty_context() {
        let mut ex = Erreur::new();
        ex.set_name("Err").set_file("a.rs").set_line(1).set_context("");
        let msg = ex.to_string();
        assert!(!msg.contains("Context:"));
    }

    #[test]
    fn display_skips_whitespace_only_context() {
        let mut ex = Erreur::new();
        ex.set_name("Err").set_file("a.rs").set_line(1).set_context("   ");
        let msg = ex.to_string();
        assert!(!msg.contains("Context:"));
    }

    #[test]
    fn display_includes_caused_by() {
        let inner = io::Error::new(io::ErrorKind::NotFound, "file missing");
        let mut ex = Erreur::new();
        ex.set_name("Wrap").set_file("a.rs").set_line(1).set_caused_by(inner);
        let msg = ex.to_string();
        assert!(msg.contains("Caused by:"));
        assert!(msg.contains("file missing"));
    }

    #[test]
    fn debug_equals_display() {
        let mut ex = Erreur::new();
        ex.set_name("Err").set_file("a.rs").set_line(1);
        assert_eq!(format!("{:?}", ex), format!("{}", ex));
    }

    // ── Error trait & source chain ──

    #[test]
    fn source_returns_none_without_inner() {
        let ex = Erreur::new();
        assert!(ex.source().is_none());
    }

    #[test]
    fn source_returns_inner_error() {
        let inner = io::Error::new(io::ErrorKind::PermissionDenied, "denied");
        let mut ex = Erreur::new();
        ex.set_caused_by(inner);
        let src = ex.source().unwrap();
        assert!(src.to_string().contains("denied"));
    }

    // ── throw! macro ──

    fn do_throw_with_args() -> Resultat<()> {
        throw!("TestException", "bad thing happened");
    }

    fn do_throw_no_args() -> Resultat<()> {
        throw!();
    }

    fn do_throw_with_format() -> Resultat<()> {
        let val = 123;
        throw!("ValueError", format!("got {}", val));
    }

    #[test]
    fn throw_with_args() {
        let err = do_throw_with_args().unwrap_err();
        assert_eq!(err.get_name(), "TestException");
        assert_eq!(err.get_context(), Some("bad thing happened"));
    }

    #[test]
    fn throw_no_args() {
        let err = do_throw_no_args().unwrap_err();
        assert_eq!(err.get_name(), "UnknownException");
    }

    #[test]
    fn throw_with_format_ctx() {
        let err = do_throw_with_format().unwrap_err();
        assert_eq!(err.get_name(), "ValueError");
        assert_eq!(err.get_context(), Some("got 123"));
    }

    #[test]
    fn throw_captures_file_location() {
        let err = do_throw_with_args().unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("src/lib.rs"));
    }

    // ── assert_throw! macro ──

    fn do_assert_throw_3args(v: i32) -> Resultat<()> {
        assert_throw!(v > 0, "NegativeValue", format!("v = {}", v));
        Ok(())
    }

    fn do_assert_throw_2args(v: i32) -> Resultat<()> {
        assert_throw!(v > 0, format!("v = {}", v));
        Ok(())
    }

    fn do_assert_throw_1arg(v: i32) -> Resultat<()> {
        assert_throw!(v > 0);
        Ok(())
    }

    #[test]
    fn assert_throw_passes_when_true() {
        assert!(do_assert_throw_3args(1).is_ok());
        assert!(do_assert_throw_2args(1).is_ok());
        assert!(do_assert_throw_1arg(1).is_ok());
    }

    #[test]
    fn assert_throw_3args_on_failure() {
        let err = do_assert_throw_3args(-1).unwrap_err();
        assert_eq!(err.get_name(), "NegativeValue");
        let ctx = err.get_context().unwrap();
        assert!(ctx.contains("v > 0"));
        assert!(ctx.contains("v = -1"));
    }

    #[test]
    fn assert_throw_2args_uses_default_name() {
        let err = do_assert_throw_2args(-1).unwrap_err();
        assert_eq!(err.get_name(), "AssertionFailedException");
        let ctx = err.get_context().unwrap();
        assert!(ctx.contains("v > 0"));
    }

    #[test]
    fn assert_throw_1arg_minimal() {
        let err = do_assert_throw_1arg(-1).unwrap_err();
        assert_eq!(err.get_name(), "AssertionFailedException");
        let ctx = err.get_context().unwrap();
        assert!(ctx.contains("v > 0"));
        assert!(!ctx.contains("Explanation:"));
    }

    // ── Catch trait ──

    fn io_err() -> Result<(), io::Error> {
        Err(io::Error::new(io::ErrorKind::NotFound, "not found"))
    }

    fn io_ok() -> Result<i32, io::Error> {
        Ok(42)
    }

    #[test]
    fn catch_on_ok_passes_through() {
        let v = io_ok().catch("X", "x").unwrap();
        assert_eq!(v, 42);
    }

    #[test]
    fn catch_on_err_wraps() {
        let err = io_err().catch("FileError", "opening config").unwrap_err();
        assert_eq!(err.get_name(), "FileError");
        assert_eq!(err.get_context(), Some("opening config"));
        assert!(err.source().unwrap().to_string().contains("not found"));
    }

    #[test]
    fn catch_lazy_on_ok() {
        let v = io_ok().catch_().unwrap();
        assert_eq!(v, 42);
    }

    #[test]
    fn catch_lazy_on_err() {
        let err = io_err().catch_().unwrap_err();
        assert_eq!(err.get_name(), "UnknownException");
        assert!(err.source().unwrap().to_string().contains("not found"));
    }

    #[test]
    fn catch_accepts_string_name() {
        let name = String::from("DynError");
        let err = io_err().catch(name, "ctx").unwrap_err();
        assert_eq!(err.get_name(), "DynError");
    }

    #[test]
    fn catch_accepts_string_ctx() {
        let ctx = format!("path = {}", "/tmp/x");
        let err = io_err().catch("E", ctx).unwrap_err();
        assert_eq!(err.get_context(), Some("path = /tmp/x"));
    }

    #[test]
    fn catch_accepts_ref_string() {
        let name = String::from("RefError");
        let ctx = String::from("details");
        let err = io_err().catch(&name, &ctx).unwrap_err();
        assert_eq!(err.get_name(), "RefError");
        assert_eq!(err.get_context(), Some("details"));
    }

    // ── IfNone trait ──

    #[test]
    fn ifnone_on_some_passes_through() {
        let v = Some(42).ifnone("X", "x").unwrap();
        assert_eq!(v, 42);
    }

    #[test]
    fn ifnone_on_none_with_name() {
        let err = Option::<i32>::None
            .ifnone("MissingValue", "expected a number")
            .unwrap_err();
        assert_eq!(err.get_name(), "MissingValue");
        assert_eq!(err.get_context(), Some("expected a number"));
    }

    #[test]
    fn ifnone_on_none_empty_name_defaults() {
        let err = Option::<i32>::None.ifnone("", "ctx").unwrap_err();
        assert_eq!(err.get_name(), "ObjectIsNone");
    }

    #[test]
    fn ifnone_lazy_on_some() {
        let v = Some(7).ifnone_().unwrap();
        assert_eq!(v, 7);
    }

    #[test]
    fn ifnone_lazy_on_none() {
        let err = Option::<i32>::None.ifnone_().unwrap_err();
        assert_eq!(err.get_name(), "UnknownException");
    }

    #[test]
    fn ifnone_accepts_string_args() {
        let name = String::from("Missing");
        let ctx = format!("key = {}", "foo");
        let err = Option::<i32>::None.ifnone(name, ctx).unwrap_err();
        assert_eq!(err.get_name(), "Missing");
        assert_eq!(err.get_context(), Some("key = foo"));
    }

    #[test]
    fn ifnone_accepts_ref_string() {
        let name = String::from("Absent");
        let ctx = String::from("detail");
        let err = Option::<i32>::None.ifnone(&name, &ctx).unwrap_err();
        assert_eq!(err.get_name(), "Absent");
        assert_eq!(err.get_context(), Some("detail"));
    }

    // ── Send + Sync ──

    #[test]
    fn erreur_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Erreur>();
    }

    // ── Error chain propagation ──

    fn inner_fn() -> Resultat<()> {
        throw!("InnerException", "root cause");
    }

    fn outer_fn() -> Resultat<()> {
        inner_fn().catch("OuterException", "while doing outer work")
    }

    #[test]
    fn error_chain_propagation() {
        let err = outer_fn().unwrap_err();
        assert_eq!(err.get_name(), "OuterException");
        let inner = err.source().unwrap();
        assert!(inner.to_string().contains("InnerException"));
        assert!(inner.to_string().contains("root cause"));
    }
}
