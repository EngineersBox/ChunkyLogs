#[macro_export]
macro_rules! ternary {
    ($condition:expr, $v1:expr, $v2:expr) => {
        if $condition { $v1 } else { $v2 };
    };
}

#[macro_export]
macro_rules! if_let_some {
    ($var:pat = $value:expr) => {
        let $var = if let Some(it) = $value {
            it
        } else {
            return;
        };
    };

    ($var:pat = $value:expr, $else_value:expr) => {
        #[allow(clippy::if_let_some_result)]
        let $var = if let Some(it) = $value {
            it
        } else {
            return $else_value;
        };
    }
}

#[macro_export]
macro_rules! if_let_ok {
    ($var:pat = $value:expr, $else_value:expr) => {
        let $var = match $value {
            Ok(it) => it,
            Err(err) => return $else_value(err),
        };
    }
}

#[macro_export]
macro_rules! try_except_return {
    ($connection_statement:expr, $msg:literal) => {
        match $connection_statement {
            Ok(value) => value,
            Err(e) => {
                error!(crate::LOGGER, "{}: {}", $msg, e);
                return;
            },
        }
    }
}

#[macro_export]
macro_rules! try_except_return_default {
    ($connection_statement:expr, $msg:literal, $default_value:expr) => {
        match $connection_statement {
            Ok(value) => value,
            Err(e) => {
                error!(crate::LOGGER, "{}: {}", $msg, e);
                $default_value
            },
        }
    }
}

#[macro_export]
macro_rules! inc {
    ($variable:expr) => {
        $variable += 1;
    }
}

#[macro_export]
macro_rules! option_same_block {
    ($conditional:expr, $some_statement:expr) => {
        if $conditional {
            return Option::Some($some_statement);
        }
        return Option::None;
    }
}

#[macro_export]
macro_rules! option_return_on_none {
    ($optional:expr) => {
        match $optional {
            Some(value) => value,
            None => return
        }
    }
}