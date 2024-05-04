(comment) @comment

(ty
  (id)) @type

(package_decl
  (id)) @type

(valid_semver) @string.special

(world_item
  name: (id) @type)

(interface_item
  name: (id) @type)

(import_item
  name: (id) @type
  (extern_type
    (interface_body)))

(import_item
  name: (id) @function
  (extern_type
    (func_type)))

(export_item
  name: (id) @type
  (extern_type
    (interface_body)))

(export_item
  name: (id) @function
  (extern_type
    (func_type)))

(type_item
  alias: (id) @type)

(func_item
  name: (id) @function)

(handle
  (id) @type)

(named_type
  name: (id) @variable)

(record_item
  name: (id) @type)

(record_field
  name: (id) @property)

(flags_items
  name: (id) @type)

(flags_body
  (id) @property)

(variant_items
  name: (id) @type)

(variant_case
  name: (id) @type)

(enum_items
  name: (id) @type)

(enum_body
  enum_cases: (id) @constant)

(resource_item
  name: (id) @type)

(resource_method
  "constructor" @constructor)

(toplevel_use_item
  "use" @keyword)

(use_item
  "use" @keyword)

(use_path
  (id) @type)

"func" @keyword

[
  "type"
  "interface"
  "world"
  "package"
  "resource"
  "record"
  "enum"
  "flags"
  "variant"
] @keyword

"static" @keyword

[
  "include"
  "import"
  "export"
] @keyword

[
  "u8"
  "u16"
  "u32"
  "u64"
  "s8"
  "s16"
  "s32"
  "s64"
  "f32"
  "f64"
  "char"
  "bool"
  "string"
  "tuple"
  "list"
  "option"
  "result"
  "borrow"
] @type

[
  "@"
  "->"
] @punctuation.special

(result
  "_" @property)

[
  "/"
  ";"
  ":"
  ","
] @punctuation.delimiter

[
  "{"
  "}"
  "("
  ")"
] @punctuation.bracket

"=" @operator
