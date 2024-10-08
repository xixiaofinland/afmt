# Configurations to take into consideration

## `max_width`

Maximum width of each line

- **Default value**: `100`
- **Possible values**: any positive integer
- **Stable**: Yes

## `fn_params_layout`

It affects the layout of parameters in function signatures.

- **Default value**: `"Tall"`
- **Possible values**: `"Compressed"`, `"Tall"`, `"Vertical"`
- **Stable**: No (tracking issue: [#1]()

## `fn_call_width`

Maximum width of the args of a function call before falling back to vertical formatting.

- **Default value**: `60`
- **Possible values**: any positive integer that is less than or equal to the value specified for [`max_width`](#max_width)
- **Stable**: No

## `indent_size`
Each indent level size.

- **Default value**: `2`
- **Possible values**: any positive integer
- **Stable**: No

## `indent_style`

Indent on expressions or items.

- **Default value**: `"Block"`
- **Possible values**: `"Block"`, `"Visual"`
- **Stable**: No

## `short_array_element_width_threshold`

The width threshold for an array element to be considered "short".

The layout of an array is dependent on the length of each of its elements.
If the length of every element in an array is below this threshold (all elements are "short") then the array can be formatted in the mixed/compressed style, but if any one element has a length that exceeds this threshold then the array elements will have to be formatted vertically.

- **Default value**: `10`
- **Possible values**: any positive integer that is less than or equal to the value specified for [`max_width`](#max_width)
- **Stable**: No

## `use_small_heuristics`

This option can be used to simplify the management and bulk updates of the granular width configuration settings ([`fn_call_width`](#fn_call_width), [`attr_fn_like_width`](#attr_fn_like_width), [`struct_lit_width`](#struct_lit_width), [`struct_variant_width`](#struct_variant_width), [`array_width`](#array_width), [`chain_width`](#chain_width), [`single_line_if_else_max_width`](#single_line_if_else_max_width)), that respectively control when formatted constructs are multi-lined/vertical based on width.

Note that explicitly provided values for the width configuration settings take precedence and override the calculated values determined by `use_small_heuristics`.

- **Default value**: `"Default"`
- **Possible values**: `"Default"`, `"Off"`, `"Max"`
- **Stable**: Yes

## `wrap_comments`

Break comments to fit on the line

Note that no wrapping will happen if:
1. The comment is the start of a markdown header doc comment
2. An URL was found in the comment

- **Default value**: `false`
- **Possible values**: `true`, `false`
- **Stable**: No

## `empty_item_single_line`

Put empty-body functions and impls on a single line

- **Default value**: `true`
- **Possible values**: `true`, `false`
- **Stable**: No
