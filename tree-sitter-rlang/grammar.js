/**
 * @file Rlang grammar for tree-sitter
 * @author RKR
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check
export default grammar({
  name: "rlang",

  extras: $ => [
    /\s/,
  ],

  rules: {
    source_file: $ => $._expression,

    _expression: $ => choice(
      $.number,
      $.identifier,
      $.call_expression,
      $.binary_expression,
      $.parenthesized_expression,
    ),

    number: $ => /-?[0-9]+(\.[0-9]*)?/,

    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,

    call_expression: $ => seq(
      field('function', $.identifier),
      '(',
      optional(seq(
        field('argument', $._expression),
        repeat(seq(',', field('argument', $._expression)))
      )),
      ')'
    ),

    parenthesized_expression: $ => seq(
      '(',
      field("expression", $._expression),
      ')'
    ),

    //
    binary_expression: $ => choice(
      prec.left(1, seq(
        field('left', $._expression),
        field('operator', choice(
          $.add_operator,
          $.subtract_operator,
        )),
        field('right', $._expression),
      )),

      prec.left(2, seq(
        field('left', $._expression),
        field('operator', choice(
          $.multiply_operator,
          $.divide_operator,
        )),
        field('right', $._expression),
      )),
    ),

    add_operator: $ => '+',
    subtract_operator: $ => '-',
    multiply_operator: $ => '*',
    divide_operator: $ => '/',
    //

    // binary_expression: $ => choice(
    //   prec.left(1, seq(
    //     field("left", $._expression),
    //     field('operator', choice('+', '-')),
    //     field('right', $._expression)
    //   )),
    //
    //   prec.left(2, seq(
    //     field("left", $._expression),
    //     field('operator', choice('*', '/')),
    //     field("right", $._expression),
    //   ))
    // ),
    //

  }
});
