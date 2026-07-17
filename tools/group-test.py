import pytest

from group import build_prefix_counts, choose_group


def groups(symbols):
    counts = build_prefix_counts(symbols)
    return {
        s: choose_group(s, counts)
        for s in symbols
    }


def test_top_level_namespace():
    assert groups([
        "accel_tap_service_subscribe",
        "window_stack_push",
        "window_stack_pop",
        "window_stack_remove",
    ]) == {
        "accel_tap_service_subscribe": "accel_tap_service",
        "window_stack_push": "window",
        "window_stack_pop": "window",
        "window_stack_remove": "window",
    }


def test_longest_shared_prefix_wins():
    assert groups([
        "foo_bar_alpha",
        "foo_bar_beta",
        "foo_bar_gamma",
        "foo_baz_alpha",
        "foo_baz_beta",
        "foo_baz_gamma",
    ]) == {
        "foo_bar_alpha": "foo_bar",
        "foo_bar_beta": "foo_bar",
        "foo_bar_gamma": "foo_bar",
        "foo_baz_alpha": "foo_baz",
        "foo_baz_beta": "foo_baz",
        "foo_baz_gamma": "foo_baz",
    }


def test_dont_choose_full_symbol():
    assert groups([
        "animation_create",
        "animation_destroy",
        "animation_clone",
    ]) == {
        "animation_create": "animation",
        "animation_destroy": "animation",
        "animation_clone": "animation",
    }


def test_small_cluster_falls_back():
    assert groups([
        "foo_alpha",
        "foo_beta",
        "bar_alpha",
        "bar_beta",
    ]) == {
        "foo_alpha": "foo_alpha",
        "foo_beta": "foo_beta",
        "bar_alpha": "bar_alpha",
        "bar_beta": "bar_beta",
    }


def test_nested_namespaces():
    assert groups([
        "a_b_c_one",
        "a_b_c_two",
        "a_b_c_three",
        "a_b_d_one",
        "a_b_d_two",
        "a_b_d_three",
    ]) == {
        "a_b_c_one": "a_b_c",
        "a_b_c_two": "a_b_c",
        "a_b_c_three": "a_b_c",
        "a_b_d_one": "a_b_d",
        "a_b_d_two": "a_b_d",
        "a_b_d_three": "a_b_d",
    }

def test_competing_prefixes():
    assert groups([
        "menu_cell_basic_header_draw",
        "menu_cell_layer_is_highlighted",
        "menu_cell_title_draw",
        "menu_layer_create",
        "menu_layer_destroy",
        "menu_layer_get_center_focused",
    ]) == {
        "menu_cell_basic_header_draw": "menu_cell",
        "menu_cell_layer_is_highlighted": "menu_cell",
        "menu_cell_title_draw": "menu_cell",
        "menu_layer_create": "menu_layer",
        "menu_layer_destroy": "menu_layer",
        "menu_layer_get_center_focused": "menu_layer",
    }


def test_many_shared_root_prefers_more_specific():
    assert groups([
        "foo_bar_alpha",
        "foo_bar_beta",
        "foo_bar_gamma",
        "foo_baz_alpha",
        "foo_baz_beta",
        "foo_baz_gamma",
        "foo_qux_alpha",
        "foo_qux_beta",
        "foo_qux_gamma",
    ]) == {
        "foo_bar_alpha": "foo_bar",
        "foo_bar_beta": "foo_bar",
        "foo_bar_gamma": "foo_bar",
        "foo_baz_alpha": "foo_baz",
        "foo_baz_beta": "foo_baz",
        "foo_baz_gamma": "foo_baz",
        "foo_qux_alpha": "foo_qux",
        "foo_qux_beta": "foo_qux",
        "foo_qux_gamma": "foo_qux",
    }
    
def test_unique_uses_full_form():
    assert groups([
        "foo_bar_alpha",
    ]) == {
        "foo_bar_alpha": "foo_bar_alpha",
    }

def test_groups_dict_serialize():
    assert groups([
        "dict_serialize_tuplets",
        "dict_serialize_tuplets_to_buffer",
        "dict_serialize_tuplets_to_buffer_with_iter",
    ]) == {
        "dict_serialize_tuplets": "dict_serialize",
        "dict_serialize_tuplets_to_buffer": "dict_serialize",
        "dict_serialize_tuplets_to_buffer_with_iter": "dict_serialize",
    }
