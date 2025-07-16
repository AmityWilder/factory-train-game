#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![deny(clippy::perf)]
#![allow(dead_code)]
#![forbid(
    clippy::missing_safety_doc,
    clippy::undocumented_unsafe_blocks,
    clippy::multiple_unsafe_ops_per_block
)]
#![warn(clippy::unnecessary_safety_doc, clippy::unnecessary_safety_comment)]
#![allow(incomplete_features, reason = "i wanna try em out")]
#![feature(
    allocator_api,
    array_chunks,
    array_try_from_fn,
    array_try_map,
    array_windows,
    arbitrary_self_types,
    arbitrary_self_types_pointers,
    assert_matches,
    ptr_as_uninit,
    as_array_of_cells,
    str_as_str,
    slice_as_array,
    slice_iter_mut_as_mut_slice,
    slice_pattern,
    slice_split_once,
    slice_concat_trait,
    slice_concat_ext,
    slice_range,
    slice_index_methods,
    const_for,
    const_destruct,
    const_format_args,
    try_blocks,
    try_find,
    const_try,
    const_btree_len,
    generic_const_exprs,
    generic_arg_infer,
    generic_assert,
    generic_const_items,
    transmute_generic_consts,
    exhaustive_patterns,
    pattern,
    never_patterns,
    never_type,
    iter_advance_by,
    iter_array_chunks,
    iter_chain,
    iter_collect_into,
    iter_intersperse,
    iter_map_windows,
    iter_next_chunk,
    iter_order_by,
    iterator_try_collect,
    iterator_try_reduce,
    async_iterator,
    deref_patterns,
    guard_patterns,
    if_let_guard,
    pattern_type_macro,
    deref_pure_trait,
    split_array,
    maybe_uninit_array_assume_init,
    maybe_uninit_as_bytes,
    maybe_uninit_fill,
    maybe_uninit_slice,
    maybe_uninit_write_slice,
    clone_to_uninit,
    lazy_get,
    lazy_type_alias,
    bound_as_ref,
    trait_alias,
    fn_traits,
    fn_delegation,
    fundamental,
    f128,
    f16,
    more_float_constants,
    more_qualified_paths,
    const_trait_impl,
    associated_const_equality,
    unsized_const_params,
    unsafe_fields,
    nonzero_ops,
    strict_overflow_ops,
    cfg_eval,
    cfg_ub_checks,
    structural_match,
    postfix_match,
    ub_checks,
    cmp_minmax,
    sort_floats,
    new_range_api,
    bigint_helper_methods,
    const_closures,
    portable_simd,
    default_field_values,
    associated_type_defaults,
    inherent_associated_types,
    auto_traits,
    derive_const,
    duration_constants,
    const_array_each_ref,
    const_range_bounds
)]

use raylib::prelude::*;

mod coords;
use crate::coords::*;

mod ordinals;

mod player;
use player::Player;

mod chem;

mod factory;
use factory::*;

fn main() {
    #[allow(unused_imports)]
    use {KeyboardKey::*, MouseButton::*};

    let (mut rl, thread) = init().title("factory game").resizable().msaa_4x().build();

    rl.set_target_fps(60);
    rl.maximize_window();

    let mut resources = Resources::new(&mut rl, &thread);

    let font = rl
        .load_font_from_memory(
            &thread,
            ".ttf",
            include_bytes!("../assets/FiraCode-Regular.ttf"),
            20,
            None,
        )
        .unwrap();

    let mut player = Player::spawn(&mut rl, &thread, PlayerVector3::new(0, 0, 0));

    let mut factory: Factory = Factory {
        origin: RailVector3 { x: 0, y: 0, z: 0 },
        reactors: Vec::new(),
    };

    while !rl.window_should_close() {
        player.update(&mut rl, &thread, &mut factory);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d = d.begin_mode3D(Camera3D::perspective(
                Vector3::new(0.0, player.height(), 0.0),
                Vector3::new(0.0, player.height(), -1.0),
                Vector3::new(0.0, 1.0, 0.0),
                45.0,
            ));
            factory.draw(&mut d, &thread, &mut resources, &player.position);
        }

        d.draw_fps(0, 0);
        d.draw_text_ex(
            &font,
            &format!(
                "player position: ({:X}, {:X}, {:X})",
                player.position.x, player.position.y, player.position.z,
            ),
            Vector2::new(0.0, 20.0),
            20.0,
            0.0,
            Color::MAGENTA,
        );
    }
}
