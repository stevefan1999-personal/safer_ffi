use ::repr_c::prelude::*;

/// Concatenate the two input strings into a new one.
///
/// The returned string must be freed using `free_char_p`.
#[ffi_export]
fn concat (
    fst: char_p_ref<'_>,
    snd: char_p_ref<'_>,
) -> char_p_boxed
{
    format!("{}{}\0", fst.to_str(), snd.to_str())
        .try_into()
        .unwrap()
}

/// Frees a string created by `concat`.
#[ffi_export]
fn free_char_p (_string: Option<char_p_boxed>)
{}

/// Same as `concat`, but with a callback-based API to auto-free the created
/// string.
#[ffi_export]
fn with_concat (
    fst: char_p_ref<'_>,
    snd: char_p_ref<'_>,
    /*mut*/ cb: RefDynFnMut1<'_, (), char_p_raw>,
)
{
    let mut cb = cb;
    let concat = concat(fst, snd);
    cb.call(concat.as_ref().into());
}

/// Returns a pointer to the maximum integer of the input slice, or `NULL` if
/// it is empty.
#[ffi_export]
fn max<'a> (
    xs: slice_ref<'a, i32>,
) -> Option<&'a i32>
{
    xs  .as_slice()
        .iter()
        .max()
}

#[repr_c::cfg_headers]
#[test]
fn generate_headers ()
  -> ::std::io::Result<()>
{
    let builder = ::repr_c::headers::builder();
    if ::std::env::var("FOO").ok().map_or(false, |it| it == "1") {
        builder
            .to_writer(::std::io::stdout())
            .generate()
    } else {
        builder
            .to_file(&"generated.h".to_string())?
            .generate()
    }
}