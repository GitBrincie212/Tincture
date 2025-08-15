#[macro_export]
macro_rules! scalar_to_packed {
    ($scalars: expr, $identity_fn: expr) => {{
        $scalars
            .par_iter()
            .map(|x| {
                AutoF32x4::new(
                    *x,
                    *x,
                    *x,
                    $identity_fn(x)
                )
            })
            .collect()
    }};
}

#[macro_export]
macro_rules! color_to_packed {
    ($colors: expr, $identity_fn: expr) => {{
        $colors.as_slice()
            .par_iter()
            .map(|x| AutoF32x4::from(extract_rgba_channels_by_type!(x, f32, $identity_fn)))
            .collect()
    }};
}

#[macro_export]
macro_rules! handle_lower_operation {
    ($self_lanes: expr, $other: expr, $operation: ident) => {
        $self_lanes
            .par_iter_mut()
            .for_each(|lane| {
                for src in &$other {
                    *lane = lane.$operation(*src);
                }
            });
    };
}