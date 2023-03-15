mod circle;
mod flip;

pub use circle::*;
pub use flip::*;

#[macro_export]
macro_rules! plug {
    ($rjimp:ident.$plug:ident($options:expr)) => {{
        use $crate::plugins::$plug;
        $rjimp.plugin($plug, $options)
    }};

    ($rjimp:ident.$plug:ident()) => {{
        use $crate::plugins::$plug;
        $rjimp.plugin($plug, Default::default())
    }};
}

#[macro_export]
macro_rules! c_plug {
    ($rjimp:ident.$plug:ident($options:expr)) => {
        $rjimp.plugin($plug, $options)
    };

    ($rjimp:ident.$plug:ident()) => {
        $rjimp.plugin($plug, Default::default())
    };
}
