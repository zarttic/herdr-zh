/// Shorthand for translating a key using the current locale.
///
/// Equivalent to `crate::i18n::tr(key)`.
///
/// # Examples
///
/// ```ignore
/// let greeting = tr!("app.greeting");
/// ```
#[macro_export]
macro_rules! tr {
    ($key:expr) => {
        $crate::i18n::tr($key)
    };
}

/// Translate a key and replace `{name}` placeholders with provided values.
///
/// Returns a `String` with all `=>`-separated name/value pairs substituted.
///
/// # Examples
///
/// ```ignore
/// let msg = tr_fmt!("dialog.pane_count", count => 5);
/// // If the locale string is "{count} panes", result is "5 panes"
/// ```
#[macro_export]
macro_rules! tr_fmt {
    ($key:expr, $( $name:ident => $value:expr ),+ $(,)?) => {{
        let mut _s = $crate::i18n::tr($key).to_string();
        $(
            _s = _s.replace(
                &format!("{{{}}}", stringify!($name)),
                &format!("{}", $value),
            );
        )+
        _s
    }};
}
