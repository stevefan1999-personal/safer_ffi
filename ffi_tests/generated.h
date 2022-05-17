/*! \file */
/*******************************************
 *                                         *
 *  File auto-generated by `::safer_ffi`.  *
 *                                         *
 *  Do not manually edit this file.        *
 *                                         *
 *******************************************/

#ifndef __RUST_FFI_TESTS__
#define __RUST_FFI_TESTS__
#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>
#include <stdint.h>

/** \remark Has the same ABI as `uint8_t` **/
#ifdef DOXYGEN
typedef enum Wow
#else
typedef uint8_t Wow_t; enum
#endif
{
    /** . */
    WOW_LEROY,
    /** . */
    WOW_JENKINS,
}
#ifdef DOXYGEN
Wow_t
#endif
;

typedef struct {

    Wow_t are_you_still_there;

} AnUnusedStruct_t;

/** <No documentation available> */
/** \remark Has the same ABI as `int8_t` **/
#ifdef DOXYGEN
typedef
#endif
enum Bar {
    /** <No documentation available> */
    BAR_A = 43,
    /** <No documentation available> */
    BAR_B = 42,
}
#ifndef DOXYGEN
; typedef int8_t
#endif
Bar_t;


#include <stdbool.h>

/** \brief
 *  Hello, `World`!
 */
typedef struct next_generation {
    /** \brief
     *  I test some `gen`-eration.
     */
    Bar_t gen;

    /** \brief
     *  with function pointers and everything!
     */
    bool (*cb)(bool);
} next_generation_t;

/** \brief
 *  Hello, `World`!
 */
/** \remark Has the same ABI as `uint8_t` **/
#ifdef DOXYGEN
typedef
#endif
enum triforce {
    /** <No documentation available> */
    TRIFORCE_DIN = 3,
    /** <No documentation available> */
    TRIFORCE_FARORE = 1,
    /** <No documentation available> */
    TRIFORCE_NARYU,
}
#ifndef DOXYGEN
; typedef uint8_t
#endif
triforce_t;

int32_t async_get_ft (void);

/** \brief
 *  This is a `#[repr(C)]` enum, which leads to a classic enum def.
 */
typedef enum SomeReprCEnum {
    /** \brief
     *  This is some variant.
     */
    SOME_REPR_C_ENUM_SOME_VARIANT,
} SomeReprCEnum_t;

void check_SomeReprCEnum (
    SomeReprCEnum_t _baz);

void check_bar (
    Bar_t _bar);

/** \brief
 *  Concatenate the two input strings into a new one.
 *
 *  The returned string must be freed using `free_char_p`.
 */
char * concat (
    char const * fst,
    char const * snd);

/** \brief
 *  Frees a string created by `concat`.
 */
void free_char_p (
    char * _string);

typedef struct foo foo_t;

void free_foo (
    foo_t * foo);

/** \brief
 *  `&'lt [T]` but with a guaranteed `#[repr(C)]` layout.
 * 
 *  # C layout (for some given type T)
 * 
 *  ```c
 *  typedef struct {
 *      // Cannot be NULL
 *      T * ptr;
 *      size_t len;
 *  } slice_T;
 *  ```
 * 
 *  # Nullable pointer?
 * 
 *  If you want to support the above typedef, but where the `ptr` field is
 *  allowed to be `NULL` (with the contents of `len` then being undefined)
 *  use the `Option< slice_ptr<_> >` type.
 */
typedef struct {

    int32_t const * ptr;

    size_t len;

} slice_ref_int32_t;

/** \brief
 *  Returns a pointer to the maximum integer of the input slice, or `NULL` if
 *  it is empty.
 */
int32_t const * max (
    slice_ref_int32_t xs);

foo_t * new_foo (void);

int32_t read_foo (
    foo_t const * foo);

typedef struct {

    void * env_ptr;

    void (*call)(void *, char const *);

} RefDynFnMut1_void_char_const_ptr_t;

/** \brief
 *  Same as `concat`, but with a callback-based API to auto-free the created
 *  string.
 */
void with_concat (
    char const * fst,
    char const * snd,
    RefDynFnMut1_void_char_const_ptr_t cb);

void with_foo (
    void (*cb)(foo_t *));


#ifdef __cplusplus
} /* extern \"C\" */
#endif

#endif /* __RUST_FFI_TESTS__ */
