use taffy::{
    LengthPercentage, MaxTrackSizingFunction, MinMax, MinTrackSizingFunction,
    NonRepeatedTrackSizingFunction,
};

use gosub_render_backend::layout::CssValue;

pub fn maybe_consume_line_name<'a>(input: &mut &'a [impl CssValue]) -> Vec<&'a str> {
    let Some(first) = input.first() else {
        return Vec::new();
    };

    if first.as_string() != Some("[") {
        return Vec::new();
    }

    let mut names = Vec::new();

    let mut i = 1;
    loop {
        let Some(name) = input.get(i) else {
            return names;
        };

        let Some(name) = name.as_string() else {
            *input = &input[i..];
            return names;
        };

        i += 1;

        if name == "]" {
            *input = &input[i..];
            return names;
        }

        names.push(name);
    }
}

pub fn maybe_consume_length_percentage(input: &mut &[impl CssValue]) -> Option<LengthPercentage> {
    let Some(first) = input.first() else {
        return None;
    };

    if let Some(percent) = first.as_percentage() {
        *input = &input[1..];
        return Some(LengthPercentage::Percent(percent / 100.0));
    };

    if let Some(len) = first.unit_to_px() {
        *input = &input[1..];
        return Some(LengthPercentage::Length(len));
    }

    None
}

fn to_min(func: MaxTrackSizingFunction) -> Option<MinTrackSizingFunction> {
    match func {
        MaxTrackSizingFunction::MinContent => Some(MinTrackSizingFunction::MinContent),
        MaxTrackSizingFunction::MaxContent => Some(MinTrackSizingFunction::MaxContent),
        MaxTrackSizingFunction::Auto => Some(MinTrackSizingFunction::Auto),
        MaxTrackSizingFunction::Fixed(len) => Some(MinTrackSizingFunction::Fixed(len)),
        _ => None,
    }
}

pub fn maybe_consume_track_size(
    input: &mut &[impl CssValue],
) -> Option<NonRepeatedTrackSizingFunction> {
    if let Some(len) = maybe_consume_length_percentage(input) {
        let func = NonRepeatedTrackSizingFunction::Length(len);

        return Some(MinMax {
            min: to_min(func)?,
            max: func,
        });
    }

    let Some(value) = input.first() else {
        return None;
    };

    fn breath(value: &impl CssValue) -> Option<MaxTrackSizingFunction> {
        if let Some(value) = value.as_string() {
            return match value {
                "min-content" => Some(MaxTrackSizingFunction::MinContent),
                "max-content" => Some(MaxTrackSizingFunction::MaxContent),
                "auto" => Some(MaxTrackSizingFunction::Auto),
                _ => None,
            };
        }

        None
    }

    if let Some(value) = breath(value) {
        *input = &input[1..];
        return Some(MinMax {
            min: to_min(value)?,
            max: value,
        });
    }

    if let Some((name, args)) = value.as_function() {
        if name == "minmax" {
            let min = to_min(breath(&args.get(0)?)?)?;
            let max = breath(&args.get(1))?;

            *input = &input[1..]; //consume the function in the input
            return Some(MinMax { min, max });
        }
    }

    None
}

pub fn maybe_consume_track_sizing_function(
    input: &mut &[impl CssValue],
) -> Vec<NonRepeatedTrackSizingFunction> {
    let mut functions = Vec::new();

    loop {
        _ = maybe_consume_line_name(input);

        let Some(func) = maybe_consume_track_size(input) else {
            return functions;
        };

        functions.push(func);
    }
}
