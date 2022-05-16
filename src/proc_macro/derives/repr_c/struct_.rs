use super::*;

pub(in crate)
fn derive (
    attrs: &'_ mut Vec<Attribute>,
    vis: &'_ Visibility,
    StructName @ _: &'_ Ident,
    generics: &'_ Generics,
    fields: &'_ Fields,
) -> Result<TokenStream2>
{
    /* FIXME: should we deny zero-sized types?? */

    mod kw { ::syn::custom_keyword!(transparent); }
    if attrs.iter().any(|attr| {
        attr.path.is_ident("repr")
        &&
        attr.parse_args::<kw::transparent>().is_ok()
    })
    {
        return derive_transparent(
            attrs,
            vis,
            StructName,
            generics,
            fields,
        );
    }

    // Add docs about C layout.
    attrs.extend_::<Attribute, _>([
        parse_quote!(
            /// # C Layout
        ),
        parse_quote!(
            ///
        ),
        {
            let line = format!("{}  - [`{StructName}`_Layout](#impl-ReprC)", " ");
            parse_quote!(#[doc = #line])
        },
    ]);

    let mut ret = quote!();

    #[apply(let_quote!)]
    use ::safer_ffi::{
        ඞ,
        layout::{
            ConcreteReprC,
            CLayoutOf,
            ReprC,
        },
    };

    let EachFieldTy @ _ = || fields.iter().map(|Field { ty, .. }| ty);
    let each_field_name = || (0..).zip(fields).map(|(i, f)| match f.ident {
        | Some(ref ident) => ident.to_token_stream(),
        | None => Index { index: i, span: f.ty.span() }.into_token_stream(),
    });

    let ref StructName_Layout @ _ = format_ident!("{}_Layout", StructName);

    let ref ctype_generics = utils::ctype_generics(generics, &mut EachFieldTy());

    // define the CType
    ret.extend(crate::derives::c_type::struct_::derive(
        &[],
        &dbg_parse_quote!(pub),
        StructName_Layout,
        ctype_generics,
        &Fields::Named({
            let EachFieldTy = EachFieldTy();
            let each_field_name = (0_u8..).zip(fields).map(|(i, f)| match f.ident {
                | Some(ref ident) => ident.clone(),
                | None => format_ident!("_{}", i),
            });
            dbg_parse_quote!({
                #(
                    pub
                    #each_field_name: #CLayoutOf<#EachFieldTy>
                ),*
            })
        }),
    )?);

    // Impl ReprC to point to the just defined type
    ret.extend({
        let EachFieldTy @ _ = EachFieldTy();
        let each_field_name = each_field_name();
        let (intro_generics, fwd_generics, where_clauses) =
            ctype_generics.split_for_impl()
        ;
        quote!(
            #[allow(trivial_bounds)]
            unsafe
            impl #intro_generics
                #ReprC
            for
                #StructName #fwd_generics
            where
                #where_clauses
            {
                type CLayout = #StructName_Layout #fwd_generics;

                #[inline]
                fn is_valid (it: &'_ Self::CLayout)
                  -> #ඞ::bool
                {
                    let _: it;
                    true #(&& (
                        #ඞ::mem::size_of::<#EachFieldTy>() == 0
                        ||
                        <#EachFieldTy as #ReprC>::is_valid(
                            &it.#each_field_name
                        )
                    ))*
                }
            }
        )
    });

    Ok(ret)
}

pub(in crate)
fn derive_transparent (
    attrs: &'_ mut Vec<Attribute>,
    vis: &'_ Visibility,
    StructName @ _: &'_ Ident,
    generics: &'_ Generics,
    fields: &'_ Fields,
) -> Result<TokenStream2>
{
    todo!("transparent struct");
}
